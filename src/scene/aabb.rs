use crate::{
    core::{point3::Point, ray::Ray},
    utils::{interval::Interval, math::Axis},
};

#[derive(Clone, Copy, Default, Debug)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        Aabb {
            x: *x,
            y: *y,
            z: *z,
        }
    }

    pub fn from_points(a: &Point, b: &Point) -> Self {
        let [x, y, z] = [0, 1, 2].map(|i| {
            let (min, max) = if a.e[i] <= b.e[i] {
                (a.e[i], b.e[i])
            } else {
                (b.e[i], a.e[i])
            };
            Interval::new(min, max)
        });

        Self { x, y, z }
    }

    pub fn merge(lhs: &Aabb, rhs: &Aabb) -> Aabb {
        Aabb::new(
            &Interval::enclosing(&lhs.x, &rhs.x),
            &Interval::enclosing(&lhs.y, &rhs.y),
            &Interval::enclosing(&lhs.z, &rhs.z),
        )
    }

    pub fn expand(&self, other: &Aabb) -> Aabb {
        Aabb::merge(self, other)
    }

    pub fn axis_interval(&self, n: Axis) -> &Interval {
        match n {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        crate::utils::math::AXES
            .iter()
            .all(|&axis| self.find_ray_hit_boundaries(r, axis, ray_t))
    }

    pub fn find_ray_hit_boundaries(&self, r: &Ray, axis: Axis, ray_t: &Interval) -> bool {
        let ax_interval = Aabb::axis_interval(self, axis);
        let axis: usize = axis.into();
        let origin_ax_component = r.orig().e[axis];
        let adinv = (r.dir().e[axis]).recip();
        let t0 = (ax_interval.min - origin_ax_component) * adinv;
        let t1 = (ax_interval.max - origin_ax_component) * adinv;
        let (min_t, max_t) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

        let mut ray_t = ray_t.to_owned();
        if min_t > ray_t.min {
            ray_t.min = min_t;
        }
        if max_t < ray_t.max {
            ray_t.max = max_t;
        }

        ray_t.max > ray_t.min
    }

    pub fn compare_over_axis(&self, other: &Aabb, axis: Axis) -> std::cmp::Ordering {
        self.axis_interval(axis)
            .min
            .partial_cmp(&other.axis_interval(axis).min)
            .expect("interval contains NaN which is impossible to compare")
    }
}

#[test]
fn test_find_ray_hit_boundaries() {
    let aabb = Aabb::new(
        &Interval::new(0.0, 2.0),
        &Interval::new(0.0, 2.0),
        &Interval::new(0.0, 2.0),
    );

    let ray = Ray::new(Point::new(1.0, 1.0, -1.0), Point::new(0.0, 0.0, 1.0), None);

    let interval = Interval::new(0.0, f64::INFINITY);

    let hit = aabb.find_ray_hit_boundaries(&ray, Axis::Z, &interval);
    assert!(hit);
}
