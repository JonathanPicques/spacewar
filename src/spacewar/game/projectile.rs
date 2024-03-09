use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackOrdered};
use ggrs::PlayerHandle;
use rapier2d::geometry::InteractionGroups;
use rapier2d::pipeline::QueryFilter;

use crate::core::clock::TimeToLive;
use crate::core::physics::body::{PhysicsBody, PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::physics::collider::{PhysicsCollider, PhysicsColliderHandle, PhysicsColliderOptions};
use crate::core::physics::Physics;
use crate::core::utilities::cmp::cmp_rollack;
use crate::spacewar::game::player::{Direction, Health, Player};
use crate::spacewar::game::Game;
use crate::spacewar::{GameAssets, Layer};

#[derive(Hash, Copy, Clone, Component)]
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
    pub fn new(player: &Player, transform: &Transform, game_assets: &GameAssets) -> Self {
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

pub fn projectile_system(
    query: Query<(
        Entity,
        &Rollback,
        &Projectile,
        &PhysicsColliderHandle,
    )>,
    mut healths: Query<(&Rollback, &mut Health, &PhysicsColliderHandle)>,
    mut commands: Commands,
    //
    order: Res<RollbackOrdered>,
    physics: Res<Physics>,
) {
    let mut healths = healths.iter_mut().collect::<Vec<_>>();
    healths.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    let mut projectiles = query.iter().collect::<Vec<_>>();
    projectiles.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (e, _, _, collider_handle) in projectiles {
        let rapier_collider = physics
            .colliders
            .get(collider_handle.handle())
            .expect("Collider not found");

        physics.query_pipeline.intersections_with_shape(
            &physics.bodies,
            &physics.colliders,
            rapier_collider.position(),
            rapier_collider.shape(),
            QueryFilter::default().exclude_collider(collider_handle.handle()),
            |hit_handle| {
                if let Some((_, target, ..)) = healths
                    .iter_mut()
                    .find(|(_, _, target_handle)| hit_handle == target_handle.handle())
                {
                    target.hp = target.hp.saturating_sub(1);
                    commands.entity(e).despawn_recursive();
                    return true;
                }
                false
            },
        );
    }
}
