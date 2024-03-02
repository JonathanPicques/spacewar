use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::prelude::*;

use crate::core::Physics;

/// Computes a hash for [`Physics`].
/// This is useful to make it so [`Physics`] contributes to the checksum of a [`bevy_ggrs`] frame.
pub fn physics_hasher(physics: &Physics) -> u64 {
    let mut hasher = DefaultHasher::new();

    for (_, body) in physics.bodies.iter() {
        let rotation = body.rotation().to_string();
        let translation = body.translation();

        rotation.hash(&mut hasher);
        translation.x.to_bits().hash(&mut hasher);
        translation.y.to_bits().hash(&mut hasher);
    }
    for (_, collider) in physics.colliders.iter() {
        let rotation = collider.rotation().to_string();
        let translation = collider.translation();

        rotation.hash(&mut hasher);
        translation.x.to_bits().hash(&mut hasher);
        translation.y.to_bits().hash(&mut hasher);
    }
    hasher.finish()
}

/// Computes a hash for [`Transform`].
/// This is useful to make it so [`Transform`] contributes to the checksum of a [`bevy_ggrs`] frame.
pub fn transform_hasher(transform: &Transform) -> u64 {
    let mut hasher = DefaultHasher::new();
    let translation = transform.translation;

    assert!(
        transform.translation.is_finite(),
        "Hashing is not stable for NaN f32 values."
    );

    translation.x.to_bits().hash(&mut hasher);
    translation.y.to_bits().hash(&mut hasher);
    translation.z.to_bits().hash(&mut hasher);

    hasher.finish()
}
