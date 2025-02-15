#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]

use rand::seq::IndexedRandom;

pub fn f64_to_u32(value: f64) -> Option<u32> {
    if value.is_nan() || value < 0.0 || value > f64::from(u32::MAX) {
        None
    } else {
        Some(value as u32)
    }
}

pub fn safe_f64_to_u8_clamp(value: f64) -> Option<u8> {
    if value.is_nan() {
        None
    } else {
        // should not panic while u8::MIN < u8::MAX
        let clamped = value.clamp(u8::MIN.into(), u8::MAX.into());
        Some((clamped.round()) as u8)
    }
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub const AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];

impl Axis {
    pub fn rand() -> Axis {
        *AXES
            .choose(&mut rand::rng())
            .expect("cannot be empty because array is constant and non-zero sized")
    }
}

impl TryFrom<u8> for Axis {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Axis::X),
            1 => Ok(Axis::Y),
            2 => Ok(Axis::Z),
            _ => Err("axis over 3d space is not supported"),
        }
    }
}

impl From<Axis> for usize {
    fn from(val: Axis) -> Self {
        match val {
            Axis::X => 0,
            Axis::Y => 1,
            Axis::Z => 2,
        }
    }
}
