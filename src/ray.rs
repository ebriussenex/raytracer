use crate::{point3::Point, rgb::Rgb};

pub struct Ray {
    orig: Point,
    dir: Point,
}

pub type Colorizer = Box<dyn Fn(&Ray) -> Rgb>;

impl Ray {
    pub fn new(orig: Point, dir: Point) -> Self {
        Ray { orig, dir }
    }

    pub fn dir(&self) -> Point {
        self.dir
    }

    pub fn orig(&self) -> Point {
        self.orig
    }

    pub fn at(&self, t: f64) -> Point {
        self.orig + self.dir * t
    }

    pub fn color(&self, cl: Colorizer) -> Rgb {
        cl(self)
    }
}
