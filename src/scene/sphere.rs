use crate::core::{point3::Point, ray::Ray};

#[derive(Copy, Clone)]
pub struct Sphere {
    r: f64,
    c: Point,
}

impl Sphere {
    pub fn new(r: f64, c: Point) -> Self {
        Sphere { r, c }
    }
    pub fn hit(&self, ray: &Ray) -> Option<f64> {
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
            None
        } else {
            // we want smallest t, nearest to camera intersection
            Some((h - f64::sqrt(d)) / a)
        }
    }
}
