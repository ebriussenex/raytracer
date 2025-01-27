use std::{default, rc::Rc};

use crate::{
    core::{point3::Point, ray::Ray},
    utils::interval::Interval,
};

use assert_approx_eq::assert_approx_eq;

pub struct HitRec {
    // point, normal vec and t as scalar of where hit happened
    pub p: Point,
    pub n: Point,
    pub t: f64,
    pub face: NormalFace,
}

// where from happened ray hit, inside surface or outside
pub enum NormalFace {
    Inside,
    Outside,
    Unknown,
}

impl HitRec {
    pub fn new(p: Point, n: Point, t: f64) -> Self {
        HitRec {
            p,
            n,
            t,
            face: NormalFace::Unknown,
        }
    }
    // We assume that every normal is in opposite direction to ray
    // and save info about where from hit happened in HitRec.
    // It is also assumed that outward_normal is always unit vector
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Point) {
        debug_assert!({
            assert_approx_eq!(outward_normal.size(), 1.0);
            true
        });
        self.n = if r.dir().scalar_prod(outward_normal) < 0.0 {
            self.face = NormalFace::Inside;
            *outward_normal
        } else {
            self.face = NormalFace::Outside;
            -*outward_normal
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t_possible: &Interval) -> Option<HitRec>;
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Rc<dyn Hittable>>,
}

impl Scene {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, ray_t_possible: &Interval) -> Option<HitRec> {
        let mut cur_closest = ray_t_possible.max();
        let mut res_hr = None;
        self.objects.iter().for_each(|ho| {
            if let Some(hr) = ho.hit(ray, &Interval::new(ray_t_possible.min(), cur_closest)) {
                cur_closest = hr.t;
                res_hr = Some(hr);
            }
        });
        res_hr
    }
}
