use std::{
    cell::RefCell,
    io::{self, Write},
};

use rand::{rngs::ThreadRng, Rng};

use crate::{
    core::{point3::Point, ray::Ray, rgb::Rgb},
    scene::hittable::Scene,
    utils::interval::Interval,
};

const VIEWPORT_HEIGHT: f64 = 2.0;

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
}

impl AntiAliaser {
    fn new(samples_per_pixel: u32) -> Self {
        AntiAliaser {
            samples_per_pixel,
            samples_scale: 1.0 / samples_per_pixel as f64,
            rng: RefCell::new(rand::thread_rng()),
        }
    }

    fn retrace_to_random_near(&self, wn: f64, hn: f64) -> (f64, f64) {
        let mut brng = self.rng.borrow_mut();
        let (w_offset, h_offset) = (brng.gen_range(-0.5..0.5), brng.gen_range(-0.5..0.5));
        (wn + w_offset, hn + h_offset)
    }
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
}

impl Camera {
    pub fn new(
        initial_pos: Point,
        img_width: u32,
        ratio: f64,
        focal_len: f64,
        aa_samples_per_px: Option<u32>,
    ) -> Self {
        let img_height: u32 = if img_width as f64 / ratio < 1.0 {
            1
        } else {
            (img_width as f64 / ratio) as u32
        };

        // viewport, arbitrary size in virtual units
        let vp_w = VIEWPORT_HEIGHT * (img_width as f64 / img_height as f64);
        // viewport vectors
        let vpv_h = Point::new(0.0, -VIEWPORT_HEIGHT, 0.0);
        let vpv_w = Point::new(vp_w, 0.0, 0.0);

        // pixel spacing, pixel delta
        let px_dw = vpv_w / img_width as f64;
        let px_dh = vpv_h / img_height as f64;
        Camera {
            pos: initial_pos,
            focal_len,
            vpv_w,
            vpv_h,
            px_dw,
            px_dh,
            img_width,
            img_height,
            anti_aliaser: aa_samples_per_px.map(AntiAliaser::new),
        }
    }

    // upper left of viewport, changes if camera pos is changed
    fn vp_upper_left(&self) -> Point {
        self.pos - Point::new(0.0, 0.0, self.focal_len) - (self.vpv_w + self.vpv_h) * 0.5
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
                let mut px_color = Rgb::new(0.0, 0.0, 0.0);
                if let Some(ref anti_aliaser) = &self.anti_aliaser {
                    for _ in 0..anti_aliaser.samples_per_pixel {
                        let r = self.ray_for(wn as f64, hn as f64);
                        px_color = px_color + color(&r, scene);
                    }
                    px_color = px_color * anti_aliaser.samples_scale;
                } else {
                    let px_center = self.upper_left_pixel_center()
                        + (self.px_dw * wn as f64)
                        + (self.px_dh * hn as f64);
                    let ray_dir = px_center - self.pos;
                    let ray = Ray::new(self.pos, ray_dir);
                    px_color = color(&ray, scene);
                }
                px_color.write(io::stdout()).map_err(RenderError::WritePx)?;
            }
        }
        Ok(())
    }
}

fn color(ray: &Ray, scene: &Scene) -> Rgb {
    if let Some(rec) = scene.hit(ray, &Interval::new(0.0, f64::INFINITY)) {
        let n = rec.n;
        (Rgb::new(n.x(), n.y(), n.z()) + Rgb::new(1.0, 1.0, 1.0)) * 0.5
    } else {
        let unit_dir = ray.dir().unit();
        let a = 0.5 * (unit_dir.y() + 1.0);
        Rgb::new(1.0, 1.0, 1.0) * (1.0 - a) + Rgb::new(0.5, 0.7, 1.0) * a
    }
}
