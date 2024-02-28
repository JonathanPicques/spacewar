use bevy::prelude::*;
use rapier2d::prelude::*;

#[derive(Clone, Default, Component)]
pub struct PhysicsCollider {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Component)]
pub struct PhysicsColliderHandle(pub(crate) ColliderHandle);

impl PhysicsCollider {
    pub(crate) fn build(&self) -> Collider {
        ColliderBuilder::cuboid(self.width, self.height)
            .restitution(0.7)
            .build()
    }
}
