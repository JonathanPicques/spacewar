use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackOrdered};
use ggrs::PlayerHandle;
use rapier2d::geometry::{Group, InteractionGroups};

use crate::core::anim::SpriteSheetAnimator;
use crate::core::clock::TimeToLive;
use crate::core::physics::body::{PhysicsBody, PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::physics::collider::{PhysicsCollider, PhysicsColliderHandle, PhysicsColliderOptions};
use crate::core::utilities::cmp::cmp_rollack;
use crate::spacewar::game::player::{Direction, Player};
use crate::spacewar::game::Game;
use crate::spacewar::{GameAssets, Layer};

const LINEAR_IMPULSE: Vec2 = Vec2::new(150.0, 300.0);
const ANGULAR_IMPULSE: f32 = 135.0;

#[derive(Hash, Copy, Clone, Component)]
pub struct Grenade {
    owner: PlayerHandle,
}

#[derive(Bundle)]
pub struct GrenadeBundle {
    game: Game,
    grenade: Grenade,
    time_to_live: TimeToLive,
    //
    body: PhysicsBody,
    body_options: PhysicsBodyOptions,
    body_velocity: PhysicsBodyVelocity,
    collider: PhysicsCollider,
    collider_options: PhysicsColliderOptions,
    //
    sprite_bundle: SpriteBundle,
    sprite_sheet_animator: SpriteSheetAnimator,
}

impl GrenadeBundle {
    pub fn new(player: &Player, game_assets: &GameAssets, translation: &Vec3) -> Self {
        Self {
            game: Game {},
            grenade: Grenade { owner: player.handle },
            time_to_live: TimeToLive::from_secs_f32(3.0),
            //
            body: PhysicsBody::Dynamic,
            body_options: PhysicsBodyOptions {
                ccd: true,
                linear_damping: 0.0,
                angular_damping: 10.0,
                ..default()
            },
            body_velocity: PhysicsBodyVelocity {
                linear_velocity: Some(match player.direction {
                    Direction::Left => Vec2::new(-LINEAR_IMPULSE.x, LINEAR_IMPULSE.y),
                    Direction::Right => Vec2::new(LINEAR_IMPULSE.x, LINEAR_IMPULSE.y),
                }),
                angular_velocity: Some(match player.direction {
                    Direction::Left => ANGULAR_IMPULSE,
                    Direction::Right => -ANGULAR_IMPULSE,
                }),
            },
            //
            collider: PhysicsCollider::Circle { radius: 3.5 },
            collider_options: PhysicsColliderOptions {
                friction: 2.0,
                restitution: 0.6,
                collision_groups: InteractionGroups {
                    filter: Layer::Wall.into(),
                    memberships: Into::<Group>::into(Layer::Wall) | Layer::Projectile.into(),
                },
                ..default()
            },
            //
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    flip_x: player.direction == Direction::Left,
                    ..default()
                },
                texture: game_assets.grenade.clone(),
                transform: match player.direction {
                    Direction::Left => Transform::from_translation(*translation + Vec3::new(0.0, 15.0, 0.0)),
                    Direction::Right => Transform::from_translation(*translation + Vec3::new(0.0, 15.0, 0.0)),
                },
                ..default()
            },
            sprite_sheet_animator: SpriteSheetAnimator::new(game_assets.bullet_idle_anim.clone()),
        }
    }
}

pub fn grenade_system(
    grenades: Query<
        (
            Entity,
            &Rollback,
            &Grenade,
            &PhysicsColliderHandle,
        ),
        With<PhysicsBodyVelocity>,
    >,
    mut commands: Commands,
    //
    order: Res<RollbackOrdered>,
) {
    let mut grenades = grenades.iter().collect::<Vec<_>>();
    grenades.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (e, ..) in grenades {
        commands.entity(e).remove::<PhysicsBodyVelocity>();
    }
}
