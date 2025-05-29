use bevy::math::{Vec2, Vec3};
use rapier2d::math::{Isometry, Real, Translation, Vector};
use rapier2d::prelude::*;

pub trait ToBevyVecExt {
    fn to_bevy(&self) -> Vec2;
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

pub trait ToPhysicsVecExt {
    fn to_physics(&self) -> Vector<Real>;
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
