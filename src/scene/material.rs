use std::sync::Arc;

use rand::{distr::Uniform, prelude::Distribution, Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use crate::core::{
    point3::{Point, MIN_FLOAT_64_PRECISION},
    ray::Ray,
    rgb::ARgb,
};

use super::{
    hittable::{HitRec, NormalFace},
    texture::{SolidColor, Texture},
};

pub trait Material: Send + Sync {
    fn scatter(
        &self,
        _r_in: &Ray,
        _attenuation: &mut ARgb,
        _scattered: &mut Ray,
        _hr: &HitRec,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    texture: Arc<dyn Texture>,
    reflectance: f64,
    between: Uniform<f64>,
}

impl Lambertian {
    pub fn new(albedo: ARgb, reflectance: f64) -> Self {
        let between = Uniform::new(0.0, 1.0 + MIN_FLOAT_64_PRECISION)
            .expect("constants should not cause panic in any universe");

        Lambertian {
            texture: Arc::new(SolidColor::new(albedo)),
            reflectance,
            between,
        }
    }

    pub fn with_texture(texture: &Arc<dyn Texture>, reflectance: f64) -> Self {
        let between = Uniform::new(0.0, 1.0 + MIN_FLOAT_64_PRECISION)
            .expect("constants should not cause panic in any universe");

        Lambertian {
            texture: Arc::clone(&texture),
            reflectance,
            between,
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        attenuation: &mut ARgb,
        scattered: &mut Ray,
        hr: &HitRec,
    ) -> bool {
        let mut rng = Xoshiro256PlusPlus::from_rng(&mut rand::rng());
        if self.between.sample(&mut rng) < self.reflectance {
            let mut scatter_dir = hr.n + Point::random_unit_on_sphere(&mut rng);
            // we need to avoid zero scatter direction due to possibility of
            // later getting NaNs and infinities. It may happen when randomly generated vector
            // is opposite to normal vector.
            if scatter_dir.near_zero() {
                scatter_dir = hr.n;
            }
            *scattered = Ray::new(hr.p, scatter_dir, Some(r_in.time()));
            *attenuation =
                self.texture.color(hr.tx_coord.u, hr.tx_coord.v, &hr.p) / self.reflectance;
            true
        } else {
            false
        }
    }
}

pub struct Metal {
    albedo: ARgb,
    fuzz: Option<f64>,
}

impl Metal {
    pub fn new(albedo: ARgb, fuzz: Option<f64>) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        attenuation: &mut ARgb,
        scattered: &mut Ray,
        hr: &HitRec,
    ) -> bool {
        let mut reflected = r_in.dir().reflect(&hr.n);
        if let Some(fuzz) = self.fuzz {
            let mut rng = rand::rng();
            reflected = reflected.unit() + (Point::random_unit_on_sphere(&mut rng) * fuzz);
        }
        *scattered = Ray::new(hr.p, reflected, Some(r_in.time()));
        *attenuation = self.albedo;
        self.fuzz.is_none() || scattered.dir().scalar_prod(&hr.n) > 0.0
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        attenuation: &mut ARgb,
        scattered: &mut Ray,
        hr: &HitRec,
    ) -> bool {
        *attenuation = ARgb::new(1.0, 1.0, 1.0);
        let refraction_index = match hr.face {
            NormalFace::Inside => self.refraction_index,
            NormalFace::Outside => 1.0 / self.refraction_index,
        };
        let unit_dir = r_in.dir().unit();
        let cos_theta = f64::min((-unit_dir).scalar_prod(&hr.n), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let mut rng = rand::rng();
        let direction = if refraction_index * sin_theta > 1.0
            || reflectance(cos_theta, refraction_index) > rng.random::<f64>()
        {
            reflect(&unit_dir, &hr.n)
        } else {
            refract(&unit_dir, &hr.n, refraction_index)
        };

        *scattered = Ray::new(hr.p, direction, Some(r_in.time()));
        true
    }
}

// snells law refraction with some math without proof
fn refract(uv: &Point, n: &Point, etai_over_etat: f64) -> Point {
    let cos_theta = f64::min((-*uv).scalar_prod(n), 1.0);

    let r_out_perpendicular = (*uv + *n * cos_theta) * etai_over_etat;
    let r_out_parallel = *n
        * -f64::sqrt(f64::abs(
            1.0 - r_out_perpendicular.scalar_prod(&r_out_perpendicular),
        ));
    r_out_perpendicular + r_out_parallel
}

fn reflect(v: &Point, n: &Point) -> Point {
    *v - *n * 2.0 * v.scalar_prod(n)
}

// shlick's approximation
fn reflectance(cos: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cos, 5)
}
