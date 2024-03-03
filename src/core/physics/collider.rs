use bevy::prelude::*;
use rapier2d::prelude::*;

#[derive(Clone, Component)]
pub enum PhysicsCollider {
    Circle { radius: f32 },
    Cuboid { width: f32, height: f32 },
}

#[derive(Clone, Component)]
pub struct PhysicsColliderOptions {
    pub friction: f32,
    pub restitution: f32,
}

#[derive(Clone, Component)]
pub(crate) struct PhysicsColliderHandle(pub(crate) ColliderHandle);

impl PhysicsCollider {
    pub(crate) fn build(&self) -> Collider {
        match self {
            Self::Circle { radius } => ColliderBuilder::ball(*radius).build(),
            Self::Cuboid { width, height } => ColliderBuilder::cuboid(*width, *height).build(),
        }
    }

    pub(crate) fn apply_options(&self, collider: &mut Collider, options: &PhysicsColliderOptions) {
        collider.set_friction(options.friction);
        collider.set_restitution(options.restitution);
    }
}

impl Default for PhysicsColliderOptions {
    fn default() -> Self {
        Self { friction: 1.0, restitution: 0.1 }
    }
}
