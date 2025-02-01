use std::{
    cell::RefCell,
    f64::consts::PI,
    io::{self, Write},
};

use rand::{distributions::Uniform, prelude::Distribution, rngs::ThreadRng};

use crate::{
    core::{point3::Point, ray::Ray, rgb::Rgb},
    scene::hittable::Scene,
    utils::interval::Interval,
};

// maximum number of ray bounces into scene
const MAX_RAY_BOUNCE: u32 = 10;

pub enum RenderError {
    WriteHeader(io::Error),
    WritePx(io::Error),
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
    fn new(samples_per_pixel: u32) -> Self {
        let rng = rand::thread_rng();
        AntiAliaser {
            samples_per_pixel,
            samples_scale: 1.0 / samples_per_pixel as f64,
            rng: RefCell::new(rng),
            between: Uniform::from(-0.5..0.5),
        }
    }

    fn retrace_to_random_near(&self, wn: f64, hn: f64) -> (f64, f64) {
        let brng = &mut *self.rng.borrow_mut();
        let (w_offset, h_offset) = (self.between.sample(brng), self.between.sample(brng));
        (wn + w_offset, hn + h_offset)
    }
}

// orthonormal basis, right hand
// v - up; w - opposite to "view at"; u - camera right
struct Basis {
    pub u: Point,
    pub v: Point,
    pub w: Point,
}

// Camera represents abstraction over view on objects through pixel-viewport
// upleft_px_pos, px00_pos thus depend on camera pos, those should be updated on each camera pos
// changes.
// px_dw, px_dh is constant after creation of camera, while those depend on chosen arbitrary
// viewport size.
pub struct Camera {
    pos: Point,
    focal_len: f64,
    vpv_h: Point,
    vpv_w: Point,
    px_dw: Point,
    px_dh: Point,
    img_width: u32,
    img_height: u32,
    anti_aliaser: Option<AntiAliaser>,
    vfov: f64,
    lookat: Point,
    b: Basis,
}

impl Camera {
    pub fn new(
        lookfrom: Option<Point>,
        lookat: Option<Point>,
        vup: Option<Point>,
        img_width: u32,
        ratio: f64,
        aa_samples_per_px: Option<u32>,
        vfov: f64,
    ) -> Self {
        let lookfrom = lookfrom.unwrap_or(Point::default());
        let lookat = lookat.unwrap_or(Point::new(0.0, 0.0, -1.0));
        let vup = vup.unwrap_or(Point::new(0.0, 1.0, 0.0));
        let focal_len = (lookfrom - lookat).size();
        let img_height: u32 = if img_width as f64 / ratio < 1.0 {
            1
        } else {
            (img_width as f64 / ratio) as u32
        };

        let h = f64::tan(vfov / 2.0);
        let vp_height = 2.0 * h * focal_len;

        // viewport, arbitrary size in virtual units
        let vp_w = vp_height * (img_width as f64 / img_height as f64);

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w);
        let v = w.cross(&u);
        // viewport vectors
        let vpv_h = -v * vp_height;
        let vpv_w = u * vp_w;

        // pixel spacing, pixel delta
        let px_dw = vpv_w / img_width as f64;
        let px_dh = vpv_h / img_height as f64;
        Camera {
            lookat,
            pos: lookfrom,
            focal_len,
            vpv_w,
            vpv_h,
            px_dw,
            px_dh,
            img_width,
            img_height,
            anti_aliaser: aa_samples_per_px.map(AntiAliaser::new),
            vfov,
            b: Basis { u, v, w },
        }
    }

    // upper left of viewport, changes if camera pos is changed
    fn vp_upper_left(&self) -> Point {
        self.pos - self.b.w * self.focal_len - (self.vpv_w + self.vpv_h) * 0.5
    }

    // px(0, 0), upper left pixel position, changes if camera pos is changed
    fn upper_left_pixel_center(&self) -> Point {
        self.vp_upper_left() + (self.px_dw + self.px_dh) * 0.5
    }

    // ray for pixel width number and height number
    fn ray_for(&self, wn: f64, hn: f64) -> Ray {
        let (mut wn, mut hn) = (wn, hn);
        if let Some(ref anti_aliaser) = &self.anti_aliaser {
            (wn, hn) = anti_aliaser.retrace_to_random_near(wn, hn);
        }

        let px_center = self.upper_left_pixel_center() + (self.px_dw * wn) + (self.px_dh * hn);
        let ray_dir = px_center - self.pos;
        Ray::new(self.pos, ray_dir)
    }

    pub fn render(&self, scene: &Scene) -> Result<(), RenderError> {
        io::stdout()
            .write_all(format!("P3\n{} {}\n255\n", self.img_width, self.img_height).as_bytes())
            .map_err(RenderError::WriteHeader)?;

        for hn in 0..self.img_height {
            for wn in 0..self.img_width {
                let max_depth = MAX_RAY_BOUNCE;
                let mut px_color = Rgb::new(0.0, 0.0, 0.0);
                if let Some(ref anti_aliaser) = &self.anti_aliaser {
                    for _ in 0..anti_aliaser.samples_per_pixel {
                        let r = self.ray_for(wn as f64, hn as f64);
                        px_color = px_color + color(&r, scene, max_depth);
                    }
                    px_color = px_color * anti_aliaser.samples_scale;
                } else {
                    let px_center = self.upper_left_pixel_center()
                        + (self.px_dw * wn as f64)
                        + (self.px_dh * hn as f64);
                    let ray_dir = px_center - self.pos;
                    let ray = Ray::new(self.pos, ray_dir);
                    px_color = color(&ray, scene, max_depth);
                }
                px_color.write(io::stdout()).map_err(RenderError::WritePx)?;
            }
        }
        Ok(())
    }
}

// it probably should be a scene method
fn color(ray: &Ray, scene: &Scene, depth: u32) -> Rgb {
    if depth == 0 {
        return Rgb::new(0.0, 0.0, 0.0);
    }

    // let rng = rand::thread_rng();
    if let Some(ref mut rec) = scene.hit(ray, &Interval::new(0.001, f64::INFINITY)) {
        let attenuation: &mut Rgb = &mut Default::default();
        let scattered: &mut Ray = &mut Default::default();
        if rec.mat.scatter(ray, attenuation, scattered, rec) {
            *attenuation * color(scattered, scene, depth - 1)
        } else {
            Default::default()
        }
        //  let dir = Point::random_on_spec_hemisphere(&mut rng, &rec.n);
        // color(&Ray::new(rec.p, dir), scene, depth - 1) * 0.5
    } else {
        let unit_dir = ray.dir().unit();
        let a = 0.5 * (unit_dir.y() + 1.0);
        Rgb::new(1.0, 1.0, 1.0) * (1.0 - a) + Rgb::new(0.5, 0.7, 1.0) * a
    }
}
