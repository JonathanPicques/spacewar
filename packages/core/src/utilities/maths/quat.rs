use bevy::math::{Quat, Rot2};
use rapier2d::math::{Real, Rotation};

pub enum RotationAngle {
    Radians(f32),
    Degrees(f32),
}

pub trait ToBevyQuatExt {
    fn to_bevy(&self) -> Quat;
}

impl ToBevyQuatExt for Rotation<Real> {
    /// Creates a [`Rot2`] from this [`Rotation`].
    #[inline(always)]
    fn to_bevy(&self) -> Quat {
        Quat::from_rotation_z(self.angle())
    }
}

impl ToBevyQuatExt for RotationAngle {
    /// Creates a [`Quat`] from this [`Rotation`].
    #[inline(always)]
    fn to_bevy(&self) -> Quat {
        Quat::from_rotation_z(match *self {
            RotationAngle::Radians(radians) => radians,
            RotationAngle::Degrees(degrees) => degrees.to_radians(),
        })
    }
}

pub trait ToBevyRot2Ext {
    fn to_bevy(&self) -> Rot2;
}

impl ToBevyRot2Ext for Rotation<Real> {
    /// Creates a [`Rot2`] from this [`Rotation`].
    #[inline(always)]
    fn to_bevy(&self) -> Rot2 {
        Rot2::from(self.angle())
    }
}
