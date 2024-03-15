mod fsm;

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
use crate::core::utilities::maths::*;
use crate::spacewar::game::player::fsm::PlayerArgs;
use crate::spacewar::game::Game;
use crate::spacewar::{GameArgs, GameAssets, GameConfig, Layer};

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
    pub state: PlayerState,
    pub handle: PlayerHandle,
    pub direction: Direction,
    #[cfg_attr(feature = "stable", derivative(Hash = "ignore"))]
    pub shoot_clock: Clock,
    #[cfg_attr(feature = "stable", derivative(Hash = "ignore"))]
    pub throw_clock: Clock,
}

#[derive(Hash, Copy, Clone, Default)]
pub enum PlayerState {
    #[default]
    None,
    Idle,
    Walk,
    Jump,
    Fall,
    Shoot,
    Throw,
    ThrowEnd,
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
                shoot_clock: Clock::from_secs_f32(0.0).with_finished(true),
                throw_clock: Clock::from_secs_f32(1.0).with_finished(true),
                ..default()
            },
            //
            body: PhysicsBody::KinematicPositionBased,
            collider: PhysicsCollider::Rectangle { width: 14.0, height: 32.0 },
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
                        -68.0,
                        68.0,
                        if game_args.num_players == 1 {
                            0.0
                        } else {
                            handle as f32 / ((game_args.num_players - 1) as f32)
                        },
                    ),
                    -28.0,
                    5.0,
                )),
                ..default()
            },
            sprite_sheet_animator: SpriteSheetAnimator::new(game_assets.player_idle.clone()),
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

    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, transform, mut player, mut sprite, mut animator, mut controller) in query {
        let input = match inputs[player.handle] {
            (i, InputStatus::Confirmed) => i,
            (i, InputStatus::Predicted) => i,
            (_, InputStatus::Disconnected) => CoreInput::zeroed(),
        };

        player.tick(PlayerArgs {
            delta,
            input: &input,
            sprite: &mut sprite,
            assets: &game_assets,
            animator: &mut animator,
            commands: &mut commands,
            controller: &mut controller,
            translation: &transform.translation,
        });
    }
}
