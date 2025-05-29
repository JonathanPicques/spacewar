mod fsm;

use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{PlayerInputs, Rollback, RollbackOrdered};
use derivative::Derivative;
use ggrs::PlayerHandle;
use rapier2d::geometry::InteractionGroups;

use core::anim::SpriteSheetAnimator;
use core::clock::Clock;
use core::derive::RollbackEvent;
use core::event::events::RollbackEvents;
use core::input::CoreInput;
use core::physics::body::PhysicsBody;
use core::physics::collider::{PhysicsCollider, PhysicsColliderOptions};
use core::physics::controller::PhysicsCharacterController;
use core::utilities::cmp::cmp_rollback;
use core::utilities::maths::*;

use crate::game::player::fsm::PlayerArgs;
use crate::game::Game;
use crate::{GameArgs, GameAssets, GameConfig, Layer};

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

#[derive(Hash, Clone, RollbackEvent)]
pub struct DamageEvent {
    pub amount: u8,
    pub target: Entity,
    pub instigator: PlayerHandle,
}

#[derive(Hash, Copy, Clone, Default, Component)]
pub struct Player {
    pub handle: PlayerHandle,
    //
    pub fsm: PlayerFsm,
    pub state: PlayerState,
    pub next_state: Option<PlayerState>,
    //
    pub direction: Direction,
    pub hurt_clock: Clock,
    pub shoot_clock: Clock,
    pub throw_clock: Clock,
}

#[derive(Hash, Copy, Clone, Default, Derivative)]
pub enum PlayerFsm {
    #[default]
    None,
    DeadOnFloor,
    DeadAirborne,
}

#[derive(Hash, Copy, Clone, Default, PartialEq)]
pub enum PlayerState {
    #[default]
    None,
    Dead,
    Hurt,
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
    sprite: Sprite,
    animator: SpriteSheetAnimator,
    transform: Transform,
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
            sprite: Sprite {
                image: game_assets.player.clone(),
                anchor: Anchor::Custom(Vec2::new(0.0, -0.25)),
                texture_atlas: Some(TextureAtlas {
                    index: 0,
                    layout: game_assets.player_atlas_layout.clone(),
                }),
                ..default()
            },
            animator: SpriteSheetAnimator::new(game_assets.player_idle.clone()),
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
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn player_system(
    mut query: Query<
        (
            Entity,
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
    //
    mut damage_events: ResMut<RollbackEvents<DamageEvent>>,
) {
    let delta = time.delta();
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));

    for (entity, _, transform, mut player, mut sprite, mut animator, mut controller) in query {
        let input = match inputs[player.handle] {
            (i, InputStatus::Confirmed) => i,
            (i, InputStatus::Predicted) => i,
            (_, InputStatus::Disconnected) => CoreInput::default(),
        };

        if damage_events.iter().any(|d| d.target == entity) {
            player.transition_to_dead();
        }
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
    damage_events.clear();
}
