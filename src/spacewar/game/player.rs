use std::cmp::Ordering;
use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{PlayerInputs, Rollback, RollbackOrdered};
use bytemuck::Zeroable;
use derivative::Derivative;

use crate::core::anim::SpriteSheetAnimator;
use crate::core::clock::{Clock, TimeToLive};
use crate::core::input::CoreInput;
use crate::core::physics::body::{PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::physics::collider::PhysicsColliderOptions;
use crate::core::physics::{PhysicsBody, PhysicsCharacterController, PhysicsCollider};
use crate::core::utilities::cmp::cmp_rollack;
use crate::core::utilities::ggrs::SpawnWithRollbackCommandsExt;
use crate::core::utilities::maths::*;
use crate::spacewar::game::input::{INPUT_LEFT, INPUT_RIGHT, INPUT_SHOOT, INPUT_UP};
use crate::spacewar::game::Game;
use crate::spacewar::{GameAssets, GameConfig};

const MAX_SPEED: f32 = 2.0;
const ACCELERATION: f32 = 7.0;
const DECELERATION: f32 = 16.0;

const JUMP_STRENGTH: f32 = 6.0;

const GRAVITY_MAX_SPEED: f32 = -12.0;
const GRAVITY_ACCELERATION: f32 = 20.0;

#[derive(Eq, Ord, Hash, Clone, PartialEq, PartialOrd, Default)]
pub enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Clone, Default, Component, Derivative)]
#[derivative(Hash)]
pub struct Player {
    pub handle: usize,
    pub direction: Direction,
    #[derivative(Hash = "ignore")]
    pub shoot_clock: Clock,
}

#[allow(clippy::type_complexity)]
pub fn player_system(
    mut query: Query<
        (
            &Rollback,
            &Transform,
            &mut Player,
            &mut Sprite,
            &mut SpriteSheetAnimator,
            &mut PhysicsCharacterController,
        ),
        With<Rollback>,
    >,
    mut commands: Commands,
    //
    time: Res<Time>,
    order: Res<RollbackOrdered>,
    inputs: Res<PlayerInputs<GameConfig>>,
    game_assets: Res<GameAssets>,
) {
    let delta = time.delta_seconds();
    let delta_d = Duration::from_secs_f32(delta);

    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, transform, mut player, mut sprite, mut animator, mut controller) in query {
        let input = match inputs[player.handle] {
            (i, InputStatus::Confirmed) => i,
            (i, InputStatus::Predicted) => i,
            (_, InputStatus::Disconnected) => CoreInput::zeroed(),
        };

        let on_floor = controller.is_on_floor() || (controller.wall.left && controller.wall.right);
        let mut velocity = controller.velocity;

        player.shoot_clock.tick(delta_d);

        if input.is_set(INPUT_SHOOT) && player.shoot_clock.finished() {
            player.shoot_clock.reset();
            commands.spawn_with_rollback((
                Game {},
                TimeToLive::new(2.0),
                SpriteBundle {
                    texture: game_assets.bullet.clone(),
                    transform: match player.direction {
                        Direction::Left => Transform::from_translation(transform.translation + Vec3::new(-15.0, 6.0, 0.0)),
                        Direction::Right => Transform::from_translation(transform.translation + Vec3::new(15.0, 6.0, 0.0)),
                    },
                    ..default()
                },
                //
                PhysicsBody::Dynamic,
                PhysicsBodyOptions { gravity_scale: 0.0, ..default() },
                PhysicsBodyVelocity {
                    linear_velocity: Some(match player.direction {
                        Direction::Left => Vec2::new(-80.0, 0.0),
                        Direction::Right => Vec2::new(80.0, 0.0),
                    }),
                    ..default()
                },
                //
                PhysicsCollider::Circle { radius: 0.1 },
                PhysicsColliderOptions { friction: 0.0, restitution: 0.0, ..default() },
            ));
        }

        if input.is_set(INPUT_UP) && on_floor {
            velocity.y = JUMP_STRENGTH;
        }
        if input.is_set(INPUT_LEFT) {
            velocity.x = compute_acceleration(velocity.x, delta, -MAX_SPEED, ACCELERATION);
        } else if input.is_set(INPUT_RIGHT) {
            velocity.x = compute_acceleration(velocity.x, delta, MAX_SPEED, ACCELERATION);
        } else {
            velocity.x = compute_deceleration(velocity.x, delta, DECELERATION);
        }

        if on_floor {
            if !input.is_set(INPUT_UP) {
                velocity.y = 0.0; // stick to floor
            }
            if velocity.x != 0.0 {
                animator.animation = game_assets.player_walk_anim.clone();
            } else {
                animator.animation = game_assets.player_idle_anim.clone();
            }
        } else {
            animator.animation = game_assets.player_jump_anim.clone();
        }

        velocity.y = compute_acceleration(
            velocity.y,
            delta,
            GRAVITY_MAX_SPEED,
            GRAVITY_ACCELERATION,
        );

        player.direction = match 0.0_f32.total_cmp(&velocity.x) {
            Ordering::Less => Direction::Right,
            Ordering::Equal => player.direction.clone(),
            Ordering::Greater => Direction::Left,
        };

        sprite.flip_x = match player.direction {
            Direction::Left => true,
            Direction::Right => false,
        };

        controller.velocity = velocity;
    }
}
