use crate::core::{point3::Point, ray::Ray};

use super::hittable::{HitRec, Hittable};

#[derive(Copy, Clone)]
pub struct Sphere {
    r: f64,
    c: Point,
}

impl Sphere {
    pub fn new(r: f64, c: Point) -> Self {
        Sphere {
            r: f64::max(r, 0.0),
            c,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, min_t: f64, max_t: f64) -> Option<super::hittable::HitRec> {
        // temp vec for sphere center - origin of ray
        let oc = self.c - ray.orig();
        // quad equation
        // it's less computation if we assume b = -2h
        // thus h = (d, (C - Q)); t = (h - sqrt(d)/a);
        let a = ray.dir().scalar_prod(&ray.dir());
        let h = ray.dir().scalar_prod(&oc);
        let c = oc.scalar_prod(&oc) - self.r * self.r;
        let d = h * h - a * c;
        if d < 0.0 {
            return None;
        } else {
            // we want smallest t, nearest to camera intersection
            let sqrd = f64::sqrt(d);
            let mut t = (h - sqrd) / a;
            if t < min_t {
                t = (h + sqrd) / a;
            }

            if t > min_t && t < max_t {
                let p = ray.at(t);
                let n = (p - self.c) / self.r;
                Some(HitRec::new(p, n, t))
            } else {
                return None;
            }
        }
    }
}
