use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    e: [f64; 3],
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { e: [x, y, z] }
    }

    pub fn x(self) -> f64 {
        self.e[0]
    }

    pub fn y(self) -> f64 {
        self.e[1]
    }

    pub fn z(self) -> f64 {
        self.e[2]
    }

    pub fn unit(&self) -> Self {
        Point {
            e: self.e.map(|x| {
                let r = x / self.size();
                assert!(r < 1.0);
                r
            }),
        }
    }

    // vector length
    pub fn size(&self) -> f64 {
        f64::sqrt(self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2])
    }

    pub fn scalar_prod(&self, rhs: Point) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()],
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            e: [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()],
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, l: f64) -> Self::Output {
        Self {
            e: self.e.map(|x| x * l),
        }
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, l: f64) -> Self::Output {
        Self {
            e: self.e.map(|x| x / l),
        }
    }
}
