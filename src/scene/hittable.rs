use std::sync::Arc;

use crate::{
    core::{point3::Point, ray::Ray},
    utils::interval::Interval,
};

use assert_approx_eq::assert_approx_eq;

use super::{aabb::Aabb, bvh::Bvh, material::Material};

#[derive(Clone, Copy, Debug, Default)]
pub struct TextureCoord {
    pub u: f64,
    pub v: f64,
}

pub struct HitRec {
    // point, normal vec and t as scalar of where hit happened
    pub p: Point,
    pub n: Point,
    pub t: f64,
    pub face: NormalFace,
    pub mat: Arc<dyn Material>,
    pub tx_coord: TextureCoord,
}

// where from happened ray hit, inside surface or outside
pub enum NormalFace {
    Inside,
    Outside,
}

impl HitRec {
    pub fn new(p: Point, n: Point, t: f64, mat: Arc<dyn Material>) -> Self {
        HitRec {
            p,
            n,
            t,
            tx_coord: TextureCoord { u: 0.0, v: 0.0 },
            face: NormalFace::Inside,
            mat,
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
            self.face = NormalFace::Outside;
            *outward_normal
        } else {
            self.face = NormalFace::Inside;
            -*outward_normal
        }
    }

    pub fn set_uv(&mut self, uv: (f64, f64)) {
        self.tx_coord = TextureCoord { u: uv.0, v: uv.1 }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t_possible: &Interval) -> Option<HitRec>;
    fn bounding_box(&self) -> &Aabb;
}

#[derive(Default)]
pub struct Scene {
    objects: Vec<Arc<dyn Hittable>>,
    sum_aabb: Aabb,
    bvh: Option<Bvh>,
}

impl Scene {
    pub fn add(&mut self, object: &Arc<dyn Hittable>) {
        self.objects.push(Arc::clone(object));
        self.sum_aabb = self.sum_aabb.expand(object.bounding_box());
    }

    // should always be called before hit check
    pub fn build_bvh(&mut self) {
        if self.bvh.is_none() {
            self.bvh = Some(Bvh::from_vec(&self.objects));
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t_possible: &Interval) -> Option<HitRec> {
        let bvh = self
            .bvh
            .as_ref()
            .expect("expected bvh to exist when hit called");
        bvh.hit(ray, ray_t_possible)
    }
}
