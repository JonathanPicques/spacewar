use std::hash::{DefaultHasher, Hash, Hasher};

use bevy::prelude::*;

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
