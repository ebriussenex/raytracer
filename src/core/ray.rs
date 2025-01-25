use super::{point3::Point, rgb::Rgb};

pub struct Ray {
    orig: Point,
    dir: Point,
}

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

    pub fn color<'a, F>(&'a self, cl: F) -> Rgb
    where
        F: FnOnce(&'a Ray) -> Rgb + 'a,
    {
        cl(self)
    }
}
