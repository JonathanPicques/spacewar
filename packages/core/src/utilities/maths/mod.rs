pub mod quat;
pub mod vector;

pub use quat::*;
pub use vector::*;

/// Returns the absolute value of a floating point number.
#[inline(always)]
pub fn abs(value: f32) -> f32 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}

/// Linearly interpolates between two values by a delta amount.
#[inline(always)]
pub fn lerp(from: f32, to: f32, delta: f32) -> f32 {
    from * (1.0 - delta) + to * delta
}

/// Returns the sign of a floating point number.
#[inline(always)]
pub fn sign(value: f32) -> f32 {
    if value < 0.0 {
        -1.0
    } else if value > 0.0 {
        1.0
    } else {
        0.0
    }
}

/// Clamps a value within a specified range.
#[inline(always)]
pub fn clamp<T>(value: T, min: T, max: T) -> T
where
    T: std::cmp::PartialOrd,
{
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Moves a value towards another value by a specified delta.
#[inline(always)]
pub fn move_towards(from: f32, to: f32, delta: f32) -> f32 {
    if abs(to - from) <= delta {
        to
    } else {
        from + (sign(to - from)) * delta
    }
}
