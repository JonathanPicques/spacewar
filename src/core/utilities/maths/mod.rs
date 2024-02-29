use bevy::math::{Quat, Vec2, Vec3};
use rapier2d::prelude::*;

/// Returns the absolute value of a floating point number.
pub fn abs(value: f32) -> f32 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}

/// Linearly interpolates between two values by a delta amount.
pub fn lerp(from: f32, to: f32, delta: f32) -> f32 {
    from * (1.0 - delta) + to * delta
}

/// Returns the sign of a floating point number.
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
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/// Moves a value towards another value by a specified delta.
pub fn move_towards(from: f32, to: f32, delta: f32) -> f32 {
    if abs(to - from) <= delta {
        to
    } else {
        from + (sign(to - from)) * delta
    }
}

/// Computes the deceleration of a value over time.
pub fn compute_deceleration(value: f32, delta: f32, deceleration: f32) -> f32 {
    move_towards(value, 0.0, deceleration * delta)
}

/// Computes the acceleration of a value over time.
pub fn compute_acceleration(value: f32, delta: f32, max_speed: f32, acceleration: f32) -> f32 {
    move_towards(value, max_speed, acceleration * delta)
}

/**
 * Vectors
 */

pub trait IntoBevyVecExt {
    fn to_bevy(&self) -> Vec2;
}
pub trait IntoPhysicsVecExt {
    fn to_physics(&self) -> Vector<Real>;
}

impl IntoBevyVecExt for Vector<Real> {
    /// Creates a [`Vec2`] from this [`Vector`].
    #[inline]
    fn to_bevy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}
impl IntoBevyVecExt for Isometry<Real> {
    /// Creates a [`Vec2`] from this [`Translation`].
    #[inline]
    fn to_bevy(&self) -> Vec2 {
        Vec2::new(self.translation.x, self.translation.y)
    }
}
impl IntoBevyVecExt for Translation<Real> {
    /// Creates a [`Vec2`] from this [`Translation`].
    #[inline]
    fn to_bevy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

impl IntoPhysicsVecExt for Vec2 {
    /// Creates a [`Vector`] from this [`Vec2`].
    #[inline]
    fn to_physics(&self) -> Vector<Real> {
        vector![self.x, self.y]
    }
}

impl IntoPhysicsVecExt for Vec3 {
    /// Creates a [`Vector`] from this [`Vec3`].
    #[inline]
    fn to_physics(&self) -> Vector<Real> {
        vector![self.x, self.y]
    }
}

/**
 * Quaternions
 */

pub enum Angle {
    Radians,
    Degrees,
}
pub trait IntoBevyQuatExt {
    fn to_bevy(self, angle: Angle) -> Quat;
}

impl<T> IntoBevyQuatExt for T
where
    T: Into<f32>,
{
    /// Creates a [`Quat`] from this [`f32`].
    #[inline]
    fn to_bevy(self, angle: Angle) -> Quat {
        Quat::from_rotation_z(match angle {
            Angle::Radians => self.into(),
            Angle::Degrees => self.into().to_radians(),
        })
    }
}
