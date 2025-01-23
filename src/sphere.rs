use crate::{point3::Point, ray::Ray};

#[derive(Copy, Clone)]
pub struct Sphere {
    r: f64,
    c: Point,
}

impl Sphere {
    pub fn new(r: f64, c: Point) -> Self {
        Sphere { r, c }
    }
    pub fn hit(&self, ray: &Ray) -> bool {
        // temp vec for sphere center - origin of ray
        let oc = self.c - ray.orig();
        // quad equation
        let a = ray.dir().scalar_prod(ray.dir());
        let b = ray.dir().scalar_prod(oc) * 2.0;
        let c = oc.scalar_prod(oc) - self.r * self.r;
        b * b - 4.0 * a * c > 0.0
    }
}
