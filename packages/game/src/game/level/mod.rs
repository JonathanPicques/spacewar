use bevy::prelude::*;
use rapier2d::geometry::InteractionGroups;

use core::physics::body::PhysicsBody;
use core::physics::collider::{PhysicsCollider, PhysicsColliderOptions};
use core::utilities::maths::Rotation;

use crate::game::Game;
use crate::Layer;

#[derive(Bundle)]
pub struct LevelRectBundle {
    game: Game,
    transform: Transform,
    //
    body: PhysicsBody,
    collider: PhysicsCollider,
    collider_options: PhysicsColliderOptions,
}

impl LevelRectBundle {
    pub fn new(collider: PhysicsCollider, rotation: Rotation, translation: Vec3) -> Self {
        Self {
            game: Game {},
            transform: Transform::default()
                .with_rotation(rotation.into())
                .with_translation(translation),
            //
            body: PhysicsBody::Fixed,
            collider,
            collider_options: PhysicsColliderOptions::from_collision_groups(InteractionGroups {
                filter: Layer::Wall.into(),
                memberships: Layer::Wall.into(),
            }),
        }
    }
}
