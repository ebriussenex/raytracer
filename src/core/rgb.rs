use std::{
    io::{Result, Write},
    ops::{Add, Div, Mul, RangeInclusive},
};

use rand::Rng;

use crate::utils::math::safe_f64_to_u8_clamp;

#[derive(Clone, Copy, Default)]
pub struct Rgb {
    rgb: [f64; 3],
}

// gamma 2 transformation
fn linear_to_gamma(linear: f64) -> f64 {
    if linear > 0.0 {
        f64::sqrt(linear)
    } else {
        0.0
    }
}

impl std::fmt::Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let u8r = self
            .rgb
            .map(linear_to_gamma)
            .map(|x| safe_f64_to_u8_clamp(x * 255.999).expect("f64 is nan!"));
        writeln!(f, "{} {} {}", u8r[0], u8r[1], u8r[2])
    }
}

impl Rgb {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Rgb { rgb: [r, g, b] }
    }

    pub fn write(&self, mut stream: impl Write) -> Result<()> {
        stream.write_all(self.to_string().as_bytes())?;
        Ok(())
    }

    pub fn random(rng: &mut impl Rng) -> Self {
        let range = 0.0..1.0;
        Self {
            rgb: [
                rng.random_range(range.clone()),
                rng.random_range(range.clone()),
                rng.random_range(range),
            ],
        }
    }

    pub fn random_with_interval(rng: &mut impl Rng, range: RangeInclusive<f64>) -> Self {
        let x = rng.random_range(range.clone());
        let y = rng.random_range(range.clone());
        let z = rng.random_range(range);
        Self { rgb: [x, y, z] }
    }
}

impl Mul<f64> for Rgb {
    type Output = Self;

    fn mul(self, l: f64) -> Self::Output {
        Self {
            rgb: self.rgb.map(|x| x * l),
        }
    }
}

impl Mul<Rgb> for Rgb {
    type Output = Self;

    fn mul(self, rhs: Rgb) -> Self::Output {
        Self {
            rgb: [
                self.rgb[0] * rhs.rgb[0],
                self.rgb[1] * rhs.rgb[1],
                self.rgb[2] * rhs.rgb[2],
            ],
        }
    }
}

impl Div<f64> for Rgb {
    type Output = Self;

    fn div(self, l: f64) -> Self::Output {
        Self {
            rgb: self.rgb.map(|x| x / l),
        }
    }
}

impl Add for Rgb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            rgb: [
                self.rgb[0] + rhs.rgb[0],
                self.rgb[1] + rhs.rgb[1],
                self.rgb[2] + rhs.rgb[2],
            ],
        }
    }
}
