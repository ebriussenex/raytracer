use std::{f64::consts::FRAC_1_PI, f64::consts::PI, sync::Arc};

use crate::{
    core::{point3::Point, ray::Ray},
    utils::interval::Interval,
};

use super::{
    aabb::Aabb,
    hittable::{HitRec, Hittable},
    material::Material,
};

pub struct Sphere {
    r: f64,
    center: Center,
    mat: Arc<dyn Material>,
    bbox: Aabb,
}

pub enum Center {
    Static(Point),
    Moving(Point, Point),
}

// sphere linearly move from center1 to center2 if it's not static
impl Sphere {
    pub fn new_static(r: f64, center: Point, mat: Arc<dyn Material>) -> Self {
        let rvec = rvec(r);
        Sphere {
            r: f64::max(r, 0.0),
            center: Center::Static(center),
            mat,
            bbox: Aabb::from_points(&(center - rvec), &(center + rvec)),
        }
    }
    pub fn new(
        r: f64,
        center: Point,
        moving_center: Option<Point>,
        mat: Arc<dyn Material>,
    ) -> Self {
        match moving_center {
            Some(second_center) => {
                let rvec = rvec(r);
                let box1 = Aabb::from_points(&(center - rvec), &(center + rvec));
                let box2 = Aabb::from_points(&(second_center - rvec), &(second_center + rvec));

                Sphere {
                    r: f64::max(r, 0.0),
                    center: Center::Moving(center, second_center),
                    mat,
                    bbox: Aabb::expand(&box1, &box2),
                }
            }

            None => Self::new_static(r, center, mat),
        }
    }

    fn center_at(&self, tm: f64) -> Point {
        match self.center {
            Center::Static(point) => point,
            Center::Moving(before_c, later_c) => Point::new(
                before_c.x() + tm * (later_c.x() - before_c.x()),
                before_c.y() + tm * (later_c.y() - before_c.y()),
                before_c.z() + tm * (later_c.z() - before_c.z()),
            ),
        }
    }

    fn uv(p: &Point) -> (f64, f64) {
        let theta = -p.y().acos();
        let phi = -p.z().atan2(p.x()) + PI;
        (0.5 * phi * FRAC_1_PI, theta * FRAC_1_PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t_possible: &Interval) -> Option<super::hittable::HitRec> {
        let cur_center = self.center_at(ray.time());
        // temp vec for sphere center - origin of ray
        let oc = cur_center - ray.orig();
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
            let sqrd = f64::sqrt(d);
            let mut t = (h - sqrd) / a;
            if !ray_t_possible.surrounds(t) {
                t = (h + sqrd) / a;
            }

            if ray_t_possible.surrounds(t) {
                let p = ray.at(t);
                let outward_normal = (p - cur_center) / self.r;

                let mut hr = HitRec::new(p, outward_normal, t, Arc::clone(&self.mat));
                hr.set_face_normal(ray, &outward_normal);
                hr.set_uv(Sphere::uv(&outward_normal));
                Some(hr)
            } else {
                None
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

fn rvec(r: f64) -> Point {
    Point::new(r, r, r)
}
