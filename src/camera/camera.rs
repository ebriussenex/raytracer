use std::{
    cell::RefCell,
    f64::consts::PI,
    io::{self, Write},
};

use rand::{
    distr::{uniform, Uniform},
    prelude::Distribution,
    rngs::ThreadRng,
    Rng,
};
use rand_xoshiro::{rand_core::SeedableRng, Xoshiro256PlusPlus};

use crate::{
    core::{point3::Point, ray::Ray, rgb::Rgb},
    scene::hittable::Scene,
    utils::{interval::Interval, math::f64_to_u32},
};

pub enum RenderError {
    WriteHeader(io::Error),
    WritePx(io::Error),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InitError {
    Antialiaser(uniform::Error),
}

struct AntiAliaser {
    pub samples_per_pixel: u32,
    pub samples_scale: f64,
    // NOTE:
    // not using interior mutability will cause really hard borrowing chains,
    // might be ok to remove rng logic in different struct later
    // later also might be useful to prepare random numbers, while in parallel doing other stuff
    rng: RefCell<ThreadRng>,
    // NOTE:
    // this usage of between makes some values of rng.gen (in retrace_to_random_near) to be prepared in compile time: https://docs.rs/rand_distr/latest/rand_distr/struct.Uniform.html
    between: Uniform<f64>,
}

impl AntiAliaser {
    fn build(samples_per_pixel: u32) -> Result<Self, uniform::Error> {
        let rng = rand::rng();
        let between = Uniform::new(-0.5, 0.5)?;
        Ok(AntiAliaser {
            samples_per_pixel,
            samples_scale: 1.0 / f64::from(samples_per_pixel),
            rng: RefCell::new(rng),
            between,
        })
    }

    fn retrace_offset(&self) -> (f64, f64) {
        let brng = &mut *self.rng.borrow_mut();
        let (w_offset, h_offset) = (self.between.sample(brng), self.between.sample(brng));
        (w_offset, h_offset)
    }
}

// orthonormal basis, right hand
// v - up; w - opposite to "view at"; u - camera right
// we allow dead_code here due to better correctnes - basis in 3d space should contain 3 elements
#[allow(dead_code)]
struct Basis {
    pub u: Point,
    pub v: Point,
    pub w: Point,
}

// defocus disk radiuses in two directions
struct Defocuser {
    pub angle: f64,
    disk_u_r: Point,
    disk_v_r: Point,
    rng: RefCell<Xoshiro256PlusPlus>,
}

impl Defocuser {
    fn new(b: &Basis, focus_r: f64, defocus_angle: f64, rng: Xoshiro256PlusPlus) -> Self {
        Defocuser {
            disk_u_r: b.u * focus_r,
            disk_v_r: b.v * focus_r,
            angle: defocus_angle,
            rng: RefCell::new(rng),
        }
    }
}

// shatter simulation for moving objects
struct Shatter {
    pub rng: RefCell<Xoshiro256PlusPlus>,
}

impl Shatter {
    fn ray_time(&self) -> f64 {
        self.rng.borrow_mut().random()
    }
}
// Camera represents abstraction over view on objects through pixel-viewport
// upleft_px_pos, px00_pos thus depend on camera pos, those should be updated on each camera pos
// changes.
// px_dw, px_dh is constant after creation of camera, while those depend on chosen arbitrary
// viewport size.
pub struct Camera {
    lookfrom: Point,
    px_du: Point,
    px_dv: Point,
    img_width: u32,
    img_height: u32,
    vp_upper_left: Point,
    px00_loc: Point,
    anti_aliaser: Option<AntiAliaser>,
    // TODO: make defocus optional
    defocus: Defocuser,
    max_bounce_depth: u32,
    shatter: Shatter,
}

impl Camera {
    pub fn build(
        lookfrom: Option<Point>,
        lookat: Option<Point>,
        vup: Option<Point>,
        img_width: u32,
        ratio: f64,
        aa_samples_per_px: Option<u32>, // use Some(0) to disable antialiasing
        vfov: Option<f64>,
        focus_dist: Option<f64>,
        defocus_angle: Option<f64>,
        max_bounce_depth: Option<u32>,
    ) -> Result<Self, InitError> {
        let lookfrom = lookfrom.unwrap_or_default();
        let lookat = lookat.unwrap_or(Point::new(0.0, 0.0, -1.0));
        let vup = vup.unwrap_or(Point::new(0.0, 1.0, 0.0)); // vector for camera to move up in 3d
                                                            // space, by coord, not by viewport position
        let vfov = vfov.unwrap_or(PI / 2.0); // vertical view angle
        let focus_dist = focus_dist.unwrap_or(1.0); // from camera to plane of perfect focus
        let defocus_angle = defocus_angle.unwrap_or(0.0); // variation angle of rays through each
                                                          // pixel
        let aa_samples_per_px = if aa_samples_per_px == Some(0) {
            None
        } else {
            Some(aa_samples_per_px.unwrap_or(100))
        };

        let max_bounce_depth = max_bounce_depth.unwrap_or(10);

        let img_height: u32 = if f64::from(img_width) / ratio < 1.0 {
            1
        } else {
            f64_to_u32(f64::from(img_width) / ratio).expect("img height in pixels should fit u32")
        };

        let h = f64::tan(vfov / 2.0);
        let vp_height = 2.0 * h * focus_dist;

        // viewport, arbitrary size in virtual units
        let vp_width = vp_height * (f64::from(img_width) / f64::from(img_height));

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w);
        let v = w.cross(&u);
        let basis = Basis { u, v, w };
        // viewport vectors
        let vp_v = -v * vp_height;
        let vp_u = u * vp_width;

        // pixel spacing, pixel delta
        let px_du = vp_u / f64::from(img_width);
        let px_dv = vp_v / f64::from(img_height);

        // location of 1st 0,0 px in vieport
        let vp_upper_left = lookfrom - (w * focus_dist) - (vp_u + vp_v) * 0.5;
        let px00_loc = vp_upper_left + (px_dv + px_du) * 0.5;

        // bluring - defocus radius
        let defocus_radius = focus_dist * f64::tan(defocus_angle / 2.0);
        let blur_rng = Xoshiro256PlusPlus::from_rng(&mut rand::rng());
        let defocus = Defocuser::new(&basis, defocus_radius, defocus_angle, blur_rng);

        // shatter
        let shatter_rng = RefCell::new(Xoshiro256PlusPlus::from_rng(&mut rand::rng()));
        let shatter = Shatter { rng: shatter_rng };

        // antialiaser
        let anti_aliaser = aa_samples_per_px
            .map(|samples| AntiAliaser::build(samples).map_err(InitError::Antialiaser))
            .transpose()?;

        Ok(Camera {
            lookfrom,
            px_du,
            px_dv,
            img_width,
            img_height,
            vp_upper_left,
            px00_loc,
            anti_aliaser,
            defocus,
            max_bounce_depth,
            shatter,
        })
    }

    // ray for pixel width number and height number
    fn ray_for(&self, wn: f64, hn: f64) -> Ray {
        // construct from the defocus disk and direct at randomly sampled point arount pixel
        // location wn,hn
        let (mut w_offset, mut h_offset) = (0.0, 0.0);

        if let Some(ref anti_aliaser) = &self.anti_aliaser {
            (w_offset, h_offset) = anti_aliaser.retrace_offset();
        }
        let px_sample =
            self.px00_loc + (self.px_du * (wn + w_offset)) + (self.px_dv * (hn + h_offset));

        let ray_orig = if self.defocus.angle <= 0.0 {
            self.lookfrom
        } else {
            self.defocus_disk_sample(&mut *self.defocus.rng.borrow_mut())
        };

        let ray_dir = px_sample - ray_orig;
        let ray_tm = self.shatter.ray_time();

        Ray::new(ray_orig, ray_dir, Some(ray_tm))
    }

    pub fn render(&self, scene: &Scene) -> Result<(), RenderError> {
        io::stdout()
            .write_all(format!("P3\n{} {}\n255\n", self.img_width, self.img_height).as_bytes())
            .map_err(RenderError::WriteHeader)?;

        for hn in 0..self.img_height {
            for wn in 0..self.img_width {
                let max_depth = self.max_bounce_depth;
                let mut px_color = Rgb::default();
                if let Some(ref anti_aliaser) = &self.anti_aliaser {
                    for _ in 0..anti_aliaser.samples_per_pixel {
                        let r = self.ray_for(f64::from(wn), f64::from(hn));
                        px_color = px_color + color(&r, scene, max_depth);
                    }
                    px_color = px_color * anti_aliaser.samples_scale;
                } else {
                    let px_center = self.vp_upper_left
                        + (self.px_du * f64::from(wn))
                        + (self.px_dv * f64::from(hn));
                    let ray_dir = px_center - self.lookfrom;
                    let ray = Ray::new(self.lookfrom, ray_dir, None);
                    px_color = color(&ray, scene, max_depth);
                }
                px_color.write(io::stdout()).map_err(RenderError::WritePx)?;
            }
        }
        Ok(())
    }

    fn defocus_disk_sample(&self, rng: &mut impl Rng) -> Point {
        let p = Point::random_on_unit_disk(rng);
        self.lookfrom + (self.defocus.disk_u_r * p.x()) + (self.defocus.disk_v_r * p.y())
    }
}

// it probably should be a scene method
fn color(ray: &Ray, scene: &Scene, depth: u32) -> Rgb {
    if depth == 0 {
        return Rgb::new(0.0, 0.0, 0.0);
    }

    if let Some(ref mut rec) = scene.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
        let attenuation = &mut Rgb::default();
        let scattered = &mut Ray::default();
        if rec.mat.scatter(ray, attenuation, scattered, rec) {
            *attenuation * color(scattered, scene, depth - 1)
        } else {
            Rgb::default()
        }
    } else {
        let unit_dir = ray.dir().unit();
        let a = 0.5 * (unit_dir.y() + 1.0);
        Rgb::new(1.0, 1.0, 1.0) * (1.0 - a) + Rgb::new(0.5, 0.7, 1.0) * a
    }
}
