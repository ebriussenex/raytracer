use std::cell::RefCell;

use rand::{distr::Uniform, prelude::Distribution, rngs::ThreadRng, Rng};

use crate::core::{
    point3::{Point, MIN_FLOAT_64_PRECISION},
    ray::Ray,
    rgb::Rgb,
};

use super::hittable::{HitRec, NormalFace};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        _attenuation: &mut Rgb,
        _scattered: &mut Ray,
        _hr: &HitRec,
    ) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Rgb,
    reflectance: f64,
    rng: RefCell<ThreadRng>,
    between: Uniform<f64>,
}

impl Lambertian {
    pub fn new(albedo: Rgb, reflectance: f64) -> Self {
        let between = Uniform::new(0.0, 1.0 + MIN_FLOAT_64_PRECISION)
            .expect("constants should not cause panic in any universe");

        Lambertian {
            albedo,
            reflectance,
            rng: RefCell::new(rand::rng()),
            between,
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        attenuation: &mut Rgb,
        scattered: &mut Ray,
        hr: &HitRec,
    ) -> bool {
        if self.between.sample(&mut *self.rng.borrow_mut()) < self.reflectance {
            let mut scatter_dir = hr.n + Point::random_unit_on_sphere(&mut self.rng.borrow_mut());
            // we need to avoid zero scatter direction due to possibility of
            // later getting NaNs and infinities. It may happen when randomly generated vector
            // is opposite to normal vector.
            if scatter_dir.near_zero() {
                scatter_dir = hr.n;
            }
            *scattered = Ray::new(hr.p, scatter_dir);
            *attenuation = self.albedo / self.reflectance;
            true
        } else {
            false
        }
    }
}

// TODO: fuzz should be checked to be less or eq than 1,
// just for api clarity
pub struct Metal {
    albedo: Rgb,
    fuzz: Option<f64>,
}

impl Metal {
    pub fn new(albedo: Rgb, fuzz: Option<f64>) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, attenuation: &mut Rgb, scattered: &mut Ray, hr: &HitRec) -> bool {
        let mut reflected = r_in.dir().reflect(&hr.n);
        if let Some(fuzz) = self.fuzz {
            let mut rng = rand::rng();
            reflected = reflected.unit() + (Point::random_unit_on_sphere(&mut rng) * fuzz);
        }
        *scattered = Ray::new(hr.p, reflected);
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
    fn scatter(&self, r_in: &Ray, attenuation: &mut Rgb, scattered: &mut Ray, hr: &HitRec) -> bool {
        *attenuation = Rgb::new(1.0, 1.0, 1.0);
        let refraction_index = match hr.face {
            NormalFace::Inside => self.refraction_index,
            NormalFace::Outside => 1.0 / self.refraction_index,
        };
        let unit_dir = r_in.dir().unit();
        let cos_theta = f64::min((-unit_dir).scalar_prod(&hr.n), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        // NOTE: may be a little bit more optimal to use already calculated cos_theta,
        // but code will be even more hard to read
        let mut rng = rand::rng();
        let direction = if refraction_index * sin_theta > 1.0
            || reflectance(cos_theta, refraction_index) > rng.random::<f64>()
        {
            reflect(&unit_dir, &hr.n)
        } else {
            refract(&unit_dir, &hr.n, refraction_index)
        };

        *scattered = Ray::new(hr.p, direction);
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
