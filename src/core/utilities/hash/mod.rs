use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::prelude::*;

use crate::core::Physics;

/// Computes a hash for [`f32`].
/// This is useful to make it so [`f32`] contributes to the checksum of a [`bevy_ggrs`] frame.
pub fn f32_hasher<T>(f: f32, mut hasher: &mut T)
where
    T: Hasher,
{
    assert!(
        f.is_finite(),
        "Hashing is not stable for NaN f32 values."
    );
    f.to_bits().hash(&mut hasher)
}

/// Computes a hash for [`Physics`].
/// This is useful to make it so [`Physics`] contributes to the checksum of a [`bevy_ggrs`] frame.
pub fn physics_hasher(physics: &Physics) -> u64 {
    let mut hasher = DefaultHasher::new();

    for (_, body) in physics.bodies.iter() {
        let rotation = body.rotation().angle();
        let translation = body.translation();

        f32_hasher(rotation, &mut hasher);
        f32_hasher(translation.x, &mut hasher);
        f32_hasher(translation.y, &mut hasher);
    }
    for (_, collider) in physics.colliders.iter() {
        let rotation = collider.rotation().angle();
        let translation = collider.translation();

        f32_hasher(rotation, &mut hasher);
        f32_hasher(translation.x, &mut hasher);
        f32_hasher(translation.y, &mut hasher);
    }
    hasher.finish()
}

/// Computes a hash for [`Transform`].
/// This is useful to make it so [`Transform`] contributes to the checksum of a [`bevy_ggrs`] frame.
pub fn transform_hasher(transform: &Transform) -> u64 {
    let mut hasher = DefaultHasher::new();

    let scale = transform.scale;
    let rotation = transform.rotation.to_euler(EulerRot::ZYX).0;
    let translation = transform.translation;

    f32_hasher(scale.x, &mut hasher);
    f32_hasher(scale.y, &mut hasher);
    f32_hasher(scale.z, &mut hasher);
    f32_hasher(rotation, &mut hasher);
    f32_hasher(translation.x, &mut hasher);
    f32_hasher(translation.y, &mut hasher);
    f32_hasher(translation.z, &mut hasher);
    hasher.finish()
}
