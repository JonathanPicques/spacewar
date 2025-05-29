use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackOrdered};
use ggrs::PlayerHandle;
use rapier2d::geometry::InteractionGroups;
use rapier2d::pipeline::QueryFilter;

use core::anim::SpriteSheetAnimator;
use core::clock::TimeToLive;
use core::event::events::RollbackEvents;
use core::physics::body::{PhysicsBody, PhysicsBodyOptions, PhysicsBodyVelocity};
use core::physics::collider::{PhysicsCollider, PhysicsColliderHandle, PhysicsColliderOptions};
use core::physics::Physics;
use core::utilities::cmp::cmp_rollback;

use crate::game::player::{DamageEvent, Direction, Health, Player};
use crate::game::Game;
use crate::{GameAssets, Layer};

const BULLET_SPEED: f32 = 250.0;

#[derive(Hash, Copy, Clone, Component)]
pub struct Bullet {
    owner: PlayerHandle,
}

#[derive(Bundle)]
pub struct BulletBundle {
    game: Game,
    bullet: Bullet,
    time_to_live: TimeToLive,
    //
    body: PhysicsBody,
    body_options: PhysicsBodyOptions,
    body_velocity: PhysicsBodyVelocity,
    collider: PhysicsCollider,
    collider_options: PhysicsColliderOptions,
    //
    sprite: Sprite,
    animator: SpriteSheetAnimator,
    transform: Transform,
}

impl BulletBundle {
    pub fn new(player: &Player, game_assets: &GameAssets, translation: &Vec3) -> Self {
        Self {
            game: Game {},
            bullet: Bullet { owner: player.handle },
            time_to_live: TimeToLive::from_secs_f32(3.0),
            //
            body: PhysicsBody::Dynamic,
            body_options: PhysicsBodyOptions { ccd: true, gravity_scale: 0.0, ..default() },
            body_velocity: PhysicsBodyVelocity {
                linear_velocity: Some(match player.direction {
                    Direction::Left => Vec2::new(-BULLET_SPEED, 0.0),
                    Direction::Right => Vec2::new(BULLET_SPEED, 0.0),
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
            sprite: Sprite {
                image: game_assets.bullet.clone(),
                flip_x: player.direction == Direction::Left,
                texture_atlas: Some(TextureAtlas {
                    index: 0,
                    layout: game_assets.bullet_atlas_layout.clone(),
                }),
                ..default()
            },
            animator: SpriteSheetAnimator::new(game_assets.bullet_idle_anim.clone()),
            transform: match player.direction {
                Direction::Left => Transform::from_translation(*translation + Vec3::new(-17.0, 7.0, 0.0)),
                Direction::Right => Transform::from_translation(*translation + Vec3::new(17.0, 7.0, 0.0)),
            },
        }
    }
}

pub fn bullet_system(
    bullets: Query<(Entity, &Rollback, &Bullet, &PhysicsColliderHandle)>,
    healths: Query<(Entity, &Rollback, &Health, &PhysicsColliderHandle)>,
    mut commands: Commands,
    //
    order: Res<RollbackOrdered>,
    physics: Res<Physics>,
    //
    mut damage_events: ResMut<RollbackEvents<DamageEvent>>,
) {
    let mut bullets = bullets.iter().collect::<Vec<_>>();
    bullets.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));

    let mut healths = healths.iter().collect::<Vec<_>>();
    healths.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));

    for (e, _, bullet, collider_handle) in bullets {
        let collider = physics
            .colliders
            .get(collider_handle.handle())
            .expect("Collider not found");

        physics.query_pipeline.intersections_with_shape(
            &physics.bodies,
            &physics.colliders,
            collider.position(),
            collider.shape(),
            QueryFilter::default().exclude_collider(collider_handle.handle()),
            |hit_handle| {
                if let Some((target, ..)) = healths
                    .iter()
                    .find(|(_, _, _, target_handle)| hit_handle == target_handle.handle())
                {
                    commands.entity(e).despawn();
                    damage_events.push(DamageEvent {
                        amount: 1,
                        target: *target,
                        instigator: bullet.owner,
                    });
                    return true;
                }
                false
            },
        );
    }
}
