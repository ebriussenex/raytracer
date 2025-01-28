use std::{
    io::{Result, Write},
    ops::{Add, Div, Mul},
};

#[derive(Clone, Copy)]
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
            .map(|x| (x * 255.999).clamp(0.0, 256.0).round() as u8);
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
}

impl Mul<f64> for Rgb {
    type Output = Self;

    fn mul(self, l: f64) -> Self::Output {
        Self {
            rgb: self.rgb.map(|x| x * l),
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
