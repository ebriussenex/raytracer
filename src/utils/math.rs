#![allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
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
