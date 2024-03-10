use std::cmp::Ordering;
use std::time::Duration;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{PlayerInputs, Rollback, RollbackOrdered};
use bytemuck::Zeroable;
use derivative::Derivative;
use ggrs::PlayerHandle;
use rapier2d::geometry::InteractionGroups;

use crate::core::anim::SpriteSheetAnimator;
use crate::core::clock::Clock;
use crate::core::input::CoreInput;
use crate::core::physics::body::PhysicsBody;
use crate::core::physics::collider::{PhysicsCollider, PhysicsColliderOptions};
use crate::core::physics::controller::PhysicsCharacterController;
use crate::core::utilities::cmp::cmp_rollack;
use crate::core::utilities::ggrs::SpawnWithRollbackCommandsExt;
use crate::core::utilities::maths::*;
use crate::spacewar::game::input::{INPUT_LEFT, INPUT_RIGHT, INPUT_SHOOT, INPUT_UP};
use crate::spacewar::game::projectile::ProjectileBundle;
use crate::spacewar::game::Game;
use crate::spacewar::{GameArgs, GameAssets, GameConfig, Layer};

const MAX_SPEED: f32 = 2.0;
const ACCELERATION: f32 = 7.0;
const DECELERATION: f32 = 16.0;

const JUMP_STRENGTH: f32 = 6.0;

const GRAVITY_MAX_SPEED: f32 = -12.0;
const GRAVITY_ACCELERATION: f32 = 20.0;

#[derive(Eq, Hash, Copy, Clone, Default, PartialEq)]
pub enum Direction {
    #[default]
    Left,
    Right,
}

#[derive(Hash, Copy, Clone, Default, Component)]
pub struct Stats {
    pub shots: u8,
    pub kills: u8,
}

#[derive(Hash, Copy, Clone, Default, Component)]
pub struct Health {
    pub hp: u8,
}

#[derive(Copy, Clone, Default, Component, Derivative)]
#[derivative(Hash)]
pub struct Player {
    pub handle: PlayerHandle,
    pub direction: Direction,
    #[cfg_attr(feature = "stable", derivative(Hash = "ignore"))]
    pub shoot_clock: Clock,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    game: Game,
    stats: Stats,
    health: Health,
    player: Player,
    //
    body: PhysicsBody,
    collider: PhysicsCollider,
    collider_options: PhysicsColliderOptions,
    character_controller: PhysicsCharacterController,
    //
    sprite_sheet_bundle: SpriteSheetBundle,
    sprite_sheet_animator: SpriteSheetAnimator,
}

impl PlayerBundle {
    pub fn new(handle: usize, game_args: &GameArgs, game_assets: &GameAssets) -> Self {
        Self {
            game: default(),
            stats: Stats::default(),
            health: Health { hp: 1 },
            player: Player {
                handle,
                shoot_clock: Clock::from_secs_f32(1.0).with_finished(true),
                ..default()
            },
            //
            body: PhysicsBody::KinematicPositionBased,
            collider: PhysicsCollider::Rectangle { width: 0.8, height: 1.8 },
            collider_options: PhysicsColliderOptions::from_collision_groups(InteractionGroups {
                filter: Layer::Wall.into(),
                memberships: Layer::Wall.into(),
            }),
            character_controller: default(),
            //
            sprite_sheet_bundle: SpriteSheetBundle {
                atlas: TextureAtlas {
                    index: 0,
                    layout: game_assets.player_atlas_layout.clone(),
                },
                sprite: Sprite {
                    anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                    ..default()
                },
                texture: game_assets.player.clone(),
                transform: Transform::from_translation(Vec3::new(
                    lerp(
                        -2.0,
                        2.0,
                        if game_args.num_players == 1 {
                            0.5
                        } else {
                            handle as f32 / ((game_args.num_players - 1) as f32)
                        },
                    ),
                    -6.0,
                    5.0,
                )),
                ..default()
            },
            sprite_sheet_animator: SpriteSheetAnimator {
                clock: Clock::from_secs_f32(0.1),
                animation: game_assets.player_idle_anim.clone(),
            },
        }
    }
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
            commands.spawn_with_rollback(ProjectileBundle::new(
                &player,
                transform,
                &game_assets,
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
            Ordering::Equal => player.direction,
            Ordering::Greater => Direction::Left,
        };

        sprite.flip_x = match player.direction {
            Direction::Left => true,
            Direction::Right => false,
        };

        controller.velocity = velocity;
    }
}
