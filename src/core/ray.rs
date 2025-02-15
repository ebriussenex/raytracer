use super::point3::Point;

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    orig: Point,
    dir: Point,
    tm: f64,
}

impl Ray {
    pub fn new(orig: Point, dir: Point, tm: Option<f64>) -> Self {
        Ray {
            orig,
            dir,
            tm: tm.unwrap_or_default(),
        }
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

    pub fn time(&self) -> f64 {
        self.tm
    }
}
