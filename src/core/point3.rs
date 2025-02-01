use std::ops::{Add, Div, Mul, Neg, Range, Sub};

use rand::{rngs::ThreadRng, Rng};

const MIN_FLOAT_64_PRECISION: f64 = 1e-160;
const ALMOST_ZERO: f64 = 1e-8;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
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

    fn random(rng: &mut ThreadRng) -> Self {
        let range = 0.0..0.1;
        Point {
            e: [
                rng.gen_range(range.clone()),
                rng.gen_range(range.clone()),
                rng.gen_range(range),
            ],
        }
    }

    fn random_with_interval(rng: &mut ThreadRng, range: Range<f64>) -> Self {
        let x = rng.gen_range(range.clone());
        let y = rng.gen_range(range.clone());
        let z = rng.gen_range(range);
        Point { e: [x, y, z] }
    }

    // TODO:
    // it's very inefficient, change it later
    //
    // random unit vector inside unit spehere,
    // here imagine 1x1x1 surrounding cube, we try to
    // find vector in cube which will be inside surrounded sphere
    // and unit it, so it fit sphere radius
    pub fn random_unit_on_sphere(rng: &mut ThreadRng) -> Self {
        // TODO: add a to_stop_on val, which will make
        // code return when we waiting too much on generating vectors
        loop {
            let p = Point::random_with_interval(rng, -1.0..1.0);
            let sqlen = p.squared_size();
            // to not get infinity 1e-160 is least for f64
            if sqlen < 1.0 && sqlen > MIN_FLOAT_64_PRECISION {
                return p / sqlen;
            }
        }
    }

    // we can use outwarding normal - if scalar production is > 0
    // the random vector is on the "right" hemisphere
    pub fn random_on_spec_hemisphere(rng: &mut ThreadRng, n: &Point) -> Point {
        let on_unit_sphere = Point::random_unit_on_sphere(rng);
        if on_unit_sphere.scalar_prod(n) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
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
