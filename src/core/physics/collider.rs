use bevy::prelude::*;
use rapier2d::prelude::*;

#[derive(Clone, Component)]
pub enum PhysicsCollider {
    Cuboid { width: f32, height: f32 },
}

#[derive(Clone, Component)]
pub(crate) struct PhysicsColliderHandle(pub(crate) ColliderHandle);

impl PhysicsCollider {
    pub(crate) fn build(&self) -> Collider {
        match self {
            Self::Cuboid { width, height } => ColliderBuilder::cuboid(*width, *height).build(),
        }
    }
}
