use crate::{point3::Point, rgb::Rgb};

pub struct Ray {
    orig: Point,
    dir: Point,
}

type Colorizer = fn(&Ray) -> Rgb;

impl Ray {
    pub fn new(orig: Point, dir: Point) -> Self {
        Ray { orig, dir }
    }

    pub fn dir(&self) -> Point {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + self.dir * t
    }

    pub fn color(&self, cl: Colorizer) -> Rgb {
        cl(self)
    }
}
