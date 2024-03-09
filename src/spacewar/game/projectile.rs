use bevy::prelude::*;
use ggrs::PlayerHandle;
use rapier2d::geometry::InteractionGroups;

use crate::core::clock::TimeToLive;
use crate::core::physics::body::{PhysicsBody, PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::physics::collider::{PhysicsCollider, PhysicsColliderOptions};
use crate::spacewar::game::player::{Direction, Player};
use crate::spacewar::game::Game;
use crate::spacewar::{GameAssets, Layer};

#[derive(Hash, Clone, Component)]
pub struct Projectile {
    owner: PlayerHandle,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    game: Game,
    projectile: Projectile,
    time_to_live: TimeToLive,
    //
    body: PhysicsBody,
    body_options: PhysicsBodyOptions,
    body_velocity: PhysicsBodyVelocity,
    collider: PhysicsCollider,
    collider_options: PhysicsColliderOptions,
    //
    sprite_bundle: SpriteBundle,
}

impl ProjectileBundle {
    pub(crate) fn new(player: &Player, transform: &Transform, game_assets: &GameAssets) -> Self {
        Self {
            game: Game {},
            projectile: Projectile { owner: player.handle },
            time_to_live: TimeToLive::from_secs_f32(3.0),
            //
            body: PhysicsBody::Dynamic,
            body_options: PhysicsBodyOptions { gravity_scale: 0.0, ..default() },
            body_velocity: PhysicsBodyVelocity {
                linear_velocity: Some(match player.direction {
                    Direction::Left => Vec2::new(-80.0, 0.0),
                    Direction::Right => Vec2::new(80.0, 0.0),
                }),
                ..default()
            },
            //
            collider: PhysicsCollider::Circle { radius: 0.1 },
            collider_options: PhysicsColliderOptions {
                friction: 0.0,
                restitution: 0.0,
                collision_groups: InteractionGroups {
                    filter: Layer::Wall.into(),
                    memberships: Layer::Projectile.into(),
                },
                ..default()
            },
            //
            sprite_bundle: SpriteBundle {
                texture: game_assets.bullet.clone(),
                transform: match player.direction {
                    Direction::Left => Transform::from_translation(transform.translation + Vec3::new(-15.0, 6.0, 0.0)),
                    Direction::Right => Transform::from_translation(transform.translation + Vec3::new(15.0, 6.0, 0.0)),
                },
                ..default()
            },
        }
    }
}
