use std::cell::RefCell;

use rand::{distributions::Uniform, prelude::Distribution, rngs::ThreadRng};

use crate::core::{point3::Point, ray::Ray, rgb::Rgb};

use super::hittable::HitRec;

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
        Lambertian {
            albedo,
            reflectance,
            rng: RefCell::new(rand::thread_rng()),
            between: Uniform::new(0.0, 1.0),
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
            let mut scatter_dir = hr.n + Point::random_unit_sphere(&mut self.rng.borrow_mut());
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
            let mut rng = rand::thread_rng();
            reflected = reflected.unit() + (Point::random_unit_sphere(&mut rng) * fuzz)
        }
        *scattered = Ray::new(hr.p, reflected);
        *attenuation = self.albedo;
        self.fuzz.is_none() || scattered.dir().scalar_prod(&hr.n) > 0.0
    }
}
