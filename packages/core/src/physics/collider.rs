use bevy::prelude::*;
use derivative::Derivative;
use rapier2d::prelude::*;

use crate::Scaler;

#[derive(Copy, Clone, Component, Derivative)]
#[derivative(Hash)]
pub enum PhysicsCollider {
    Circle {
        #[derivative(Hash = "ignore")]
        radius: f32,
    },
    Rectangle {
        #[derivative(Hash = "ignore")]
        width: f32,
        #[derivative(Hash = "ignore")]
        height: f32,
    },
}

#[derive(Copy, Clone, Component, Derivative)]
#[derivative(Hash)]
pub struct PhysicsColliderOptions {
    #[derivative(Hash = "ignore")]
    pub friction: f32,
    #[derivative(Hash = "ignore")]
    pub restitution: f32,
    #[derivative(Hash = "ignore")]
    pub active_events: ActiveEvents,
    pub collision_groups: InteractionGroups,
    pub active_collision_types: ActiveCollisionTypes,
}

#[derive(Hash, Copy, Clone, Component)]
pub struct PhysicsColliderHandle(pub(crate) ColliderHandle);

impl PhysicsCollider {
    pub(crate) fn build(&self, scaler: &Scaler) -> Collider {
        match self {
            Self::Circle { radius } => ColliderBuilder::ball(scaler.pixels_to_meters(radius)).build(),
            Self::Rectangle { width, height } => ColliderBuilder::cuboid(
                scaler.pixels_to_meters(width) / 2.0,
                scaler.pixels_to_meters(height) / 2.0,
            )
            .build(),
        }
    }

    pub(crate) fn apply_options(&self, _scaler: &Scaler, collider: &mut Collider, options: &PhysicsColliderOptions) {
        collider.set_friction(options.friction);
        collider.set_restitution(options.restitution);
        collider.set_active_events(options.active_events);
        collider.set_collision_groups(options.collision_groups);
        collider.set_active_collision_types(options.active_collision_types);
    }
}

impl PhysicsColliderHandle {
    #[inline(always)]
    pub fn handle(&self) -> ColliderHandle {
        self.0
    }
}

impl PhysicsColliderOptions {
    pub fn from_friction(friction: f32) -> Self {
        Self { friction, ..default() }
    }

    pub fn from_restitution(restitution: f32) -> Self {
        Self { restitution, ..default() }
    }

    pub fn from_collision_groups(collision_groups: InteractionGroups) -> Self {
        Self { collision_groups, ..default() }
    }
}

impl Default for PhysicsColliderOptions {
    fn default() -> Self {
        Self {
            friction: 1.0,
            restitution: 0.1,
            active_events: default(),
            collision_groups: default(),
            active_collision_types: default(),
        }
    }
}
