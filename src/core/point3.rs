use std::{
    f64::consts::TAU,
    ops::{Add, Div, Mul, Neg, RangeInclusive, Sub},
};

use rand::{distr::StandardUniform, Rng};

use crate::utils::math::Axis;

pub const MIN_FLOAT_64_PRECISION: f64 = 1e-160;
const ALMOST_ZERO: f64 = 1e-8;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Point {
    pub e: [f64; 3],
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { e: [x, y, z] }
    }

    pub fn coord(&self, ax: Axis) -> f64 {
        self.e[std::convert::Into::<usize>::into(ax)]
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
        *self / self.size()
    }

    // vector length
    pub fn size(&self) -> f64 {
        f64::sqrt(self.squared_size())
    }

    fn squared_size(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn scalar_prod(&self, rhs: &Point) -> f64 {
        if self == rhs {
            self.squared_size()
        } else {
            self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
        }
    }

    pub fn cross(&self, rhs: &Point) -> Point {
        Point {
            e: [
                self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
                self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
                self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
            ],
        }
    }

    fn random(rng: &mut impl Rng) -> Self {
        let range = 0.0..1.0;
        Point {
            e: [
                rng.random_range(range.clone()),
                rng.random_range(range.clone()),
                rng.random_range(range),
            ],
        }
    }

    fn random_with_interval(rng: &mut impl Rng, range: RangeInclusive<f64>) -> Self {
        let x = rng.random_range(range.clone());
        let y = rng.random_range(range.clone());
        let z = rng.random_range(range);
        Point { e: [x, y, z] }
    }

    pub fn random_unit_on_sphere(rng: &mut impl Rng) -> Self {
        let x = rng.sample(StandardUniform);
        let y = rng.sample(StandardUniform);
        let z = rng.sample(StandardUniform);
        Point::new(x, y, z).unit()
    }

    // we can use outwarding normal - if scalar production is > 0
    // the random vector is on the "right" hemisphere
    pub fn random_on_spec_hemisphere(rng: &mut impl Rng, n: &Point) -> Point {
        let on_unit_sphere = Point::random_unit_on_sphere(rng);
        if on_unit_sphere.scalar_prod(n) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn random_on_unit_disk(rng: &mut impl Rng) -> Self {
        let r = rng.random::<f64>().sqrt();
        let theta = rng.random_range(0.0..TAU);
        Point::new(r * theta.cos(), r * theta.sin(), 0.0)
    }

    pub fn near_zero(&self) -> bool {
        self.e.iter().all(|x| x.abs() < ALMOST_ZERO)
    }

    pub fn reflect(&self, n: &Point) -> Point {
        *self - *n * 2.0 * self.scalar_prod(n)
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

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Point {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}
