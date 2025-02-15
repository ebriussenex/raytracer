use std::{cmp::Ordering, sync::Arc};

use crate::{
    core::ray::Ray,
    utils::{interval::Interval, math::Axis},
};

use super::{
    aabb::{self, Aabb},
    hittable::{HitRec, Hittable},
};

enum Either {
    Left,
    Right,
}

pub struct Bvh {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl Bvh {
    pub fn from_vec(objects: &[Arc<dyn Hittable>]) -> Self {
        Self::new(objects.to_owned().as_mut())
    }
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        // build bbox of the span of source objects
        let mut bbox = aabb::EMPTY;
        objects
            .iter()
            .for_each(|obj| bbox = bbox.expand(obj.bounding_box()));

        let axis = bbox.longest_axis();

        let (left, right) = match objects.len() {
            0 => unreachable!(),
            1 => (Arc::clone(&objects[0]), (Arc::clone(&objects[0]))),
            2 => {
                let left = Arc::clone(&objects[0]);
                let right = Arc::clone(&objects[1]);
                match Self::compare_hittables(&left, &right, axis) {
                    Either::Left => (left, right),
                    Either::Right => (right, left),
                }
            }
            span => {
                objects.sort_unstable_by(Self::cmp(axis));
                let mid = span / 2;
                (
                    Arc::new(Self::new(&mut objects[..mid])) as Arc<dyn Hittable>,
                    Arc::new(Self::new(&mut objects[mid..])) as Arc<dyn Hittable>,
                )
            }
        };
        Self { left, right, bbox }
    }

    fn compare_hittables(lhs: &Arc<dyn Hittable>, rhs: &Arc<dyn Hittable>, axis: Axis) -> Either {
        match lhs
            .bounding_box()
            .compare_over_axis(rhs.bounding_box(), axis)
        {
            Ordering::Less | Ordering::Equal => Either::Left,
            Ordering::Greater => Either::Right,
        }
    }

    fn cmp(axis: Axis) -> impl FnMut(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
        move |lhs, rhs| {
            lhs.bounding_box()
                .compare_over_axis(rhs.bounding_box(), axis)
        }
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, ray_t_possible: &Interval) -> Option<HitRec> {
        if self.bbox.hit(ray, ray_t_possible) {
            let hit_left = self.left.hit(ray, ray_t_possible);
            let hit_right = match hit_left {
                Some(ref left_hr) => self
                    .right
                    .hit(ray, &Interval::new(ray_t_possible.min, left_hr.t)),
                None => self.right.hit(ray, ray_t_possible),
            };
            hit_right.or(hit_left)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
