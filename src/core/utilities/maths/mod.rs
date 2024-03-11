use bevy::math::{Quat, Vec2, Vec3};
use rapier2d::prelude::*;

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

/// Computes the deceleration of a value over time.
#[inline(always)]
pub fn compute_deceleration(value: f32, deceleration: f32) -> f32 {
    move_towards(value, 0.0, deceleration)
}

/// Computes the acceleration of a value over time.
#[inline(always)]
pub fn compute_acceleration(value: f32, max_speed: f32, acceleration: f32) -> f32 {
    move_towards(value, max_speed, acceleration)
}

/**
 * Vectors
 */

pub trait ToBevyVecExt {
    fn to_bevy(&self) -> Vec2;
}
pub trait ToPhysicsVecExt {
    fn to_physics(&self) -> Vector<Real>;
}

impl ToBevyVecExt for Vector<Real> {
    /// Creates a [`Vec2`] from this [`Vector`].
    #[inline(always)]
    fn to_bevy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
impl ToBevyVecExt for Isometry<Real> {
    /// Creates a [`Vec2`] from this [`Translation`].
    #[inline(always)]
    fn to_bevy(&self) -> Vec2 {
        Vec2::new(self.translation.x, self.translation.y)
    }
}
impl ToBevyVecExt for Translation<Real> {
    /// Creates a [`Vec2`] from this [`Translation`].
    #[inline(always)]
    fn to_bevy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl ToPhysicsVecExt for Vec2 {
    /// Creates a [`Vector`] from this [`Vec2`].
    #[inline(always)]
    fn to_physics(&self) -> Vector<Real> {
        vector![self.x, self.y]
    }
}

impl ToPhysicsVecExt for Vec3 {
    /// Creates a [`Vector`] from this [`Vec3`].
    #[inline(always)]
    fn to_physics(&self) -> Vector<Real> {
        vector![self.x, self.y]
    }
}

/**
 * Quaternions
 */

pub enum Rotation {
    Radians(f32),
    Degrees(f32),
}

impl From<Rotation> for Quat {
    /// Creates a [`Quat`] from this [`Rotation`].
    #[inline(always)]
    fn from(value: Rotation) -> Self {
        Quat::from_rotation_z(match value {
            Rotation::Radians(radians) => radians,
            Rotation::Degrees(degrees) => degrees.to_radians(),
        })
    }
}
