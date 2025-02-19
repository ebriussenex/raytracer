use std::sync::Arc;

use image::RgbImage;

use crate::core::{
    point3::Point,
    rgb::{ARgb, SOLID_CYAN_COLOR},
};

pub trait Texture: Send + Sync {
    fn color(&self, u: f64, v: f64, p: &Point) -> ARgb;
}

pub struct SolidColor {
    albedo: ARgb,
}

impl SolidColor {
    pub fn new(albedo: ARgb) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64, _p: &Point) -> ARgb {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even_color: ARgb, odd_color: ARgb) -> Self {
        Self {
            inv_scale: scale.recip(),
            even: Arc::new(SolidColor::new(even_color)),
            odd: Arc::new(SolidColor::new(odd_color)),
        }
    }
}

impl Texture for CheckerTexture {
    fn color(&self, u: f64, v: f64, p: &Point) -> ARgb {
        let is_even =
            p.e.iter()
                .fold(0, |acc: i32, e| acc + (e * self.inv_scale).floor() as i32)
                % 2
                == 0;
        if is_even {
            self.even.color(u, v, p)
        } else {
            self.odd.color(u, v, p)
        }
    }
}

pub struct ImageTexture {
    px_colors: Arc<RgbImage>,
}

impl ImageTexture {
    pub fn new(px_colors: Arc<RgbImage>) -> Self {
        Self { px_colors }
    }
}

impl Texture for ImageTexture {
    // this may not happen, in case of [0,1] f64
    #![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn color(&self, u: f64, v: f64, _p: &Point) -> ARgb {
        let width = self.px_colors.width();
        let height = self.px_colors.height();

        if height == 0 {
            return SOLID_CYAN_COLOR;
        }

        let u = u.clamp(0.0, 1.0);
        let v = v.abs().clamp(0.0, 1.0);

        let i = ((u * f64::from(width)) as u32).min(width - 1);
        let j = ((v * f64::from(height)) as u32).min(height - 1);
        let px = self.px_colors.get_pixel(i, j).0;
        let r = f64::from(px[0]) / 255.0;
        let g = f64::from(px[1]) / 255.0;
        let b = f64::from(px[2]) / 255.0;

        ARgb::new(r, g, b)
    }
}
