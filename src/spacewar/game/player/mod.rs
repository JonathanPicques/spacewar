pub mod input;

use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{PlayerInputs, Rollback, RollbackOrdered};
use bytemuck::Zeroable;

use crate::core::anim::SpriteSheetAnimator;
use crate::core::input::CoreInput;
use crate::core::levels::{find_levels_around_positions, LoadNeighbours, LoadedLevels};
use crate::core::physics::PhysicsCharacterController;
use crate::core::utilities::cmp::cmp_rollack;
use crate::core::utilities::maths::*;
use crate::spacewar::conf::{GameAssets, GameConfig};
use crate::spacewar::game::player::input::{INPUT_LEFT, INPUT_RIGHT, INPUT_UP};

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

#[derive(Eq, Ord, Hash, Clone, PartialEq, PartialOrd, Default, Component)]
pub struct Player {
    pub handle: usize,
    pub direction: Direction,
}

pub fn player_system(
    mut query: Query<
        (
            &Rollback,
            &mut Player,
            &mut TextureAtlasSprite,
            &mut SpriteSheetAnimator,
            &mut PhysicsCharacterController,
        ),
        With<Rollback>,
    >,
    //
    time: Res<Time>,
    order: Res<RollbackOrdered>,
    inputs: Res<PlayerInputs<GameConfig>>,
    game_assets: Res<GameAssets>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, mut player, mut sprite, mut animator, mut controller) in query {
        let input = match inputs[player.handle] {
            (i, InputStatus::Confirmed) => i,
            (i, InputStatus::Predicted) => i,
            (_, InputStatus::Disconnected) => CoreInput::zeroed(),
        };
        let mut velocity = controller.velocity;

        if input.is_set(INPUT_UP) && controller.is_on_floor() {
            velocity.y = JUMP_STRENGTH;
        }
        if input.is_set(INPUT_LEFT) {
            velocity.x = compute_acceleration(
                velocity.x,
                time.delta_seconds(),
                -MAX_SPEED,
                ACCELERATION,
            );
        } else if input.is_set(INPUT_RIGHT) {
            velocity.x = compute_acceleration(
                velocity.x,
                time.delta_seconds(),
                MAX_SPEED,
                ACCELERATION,
            );
        } else {
            velocity.x = compute_deceleration(velocity.x, time.delta_seconds(), DECELERATION);
        }

        if controller.is_on_floor() {
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
            time.delta_seconds(),
            GRAVITY_MAX_SPEED,
            GRAVITY_ACCELERATION,
        );

        player.direction = match 0.0_f32.total_cmp(&velocity.x) {
            Ordering::Less => Direction::Left,
            Ordering::Equal => player.direction.clone(),
            Ordering::Greater => Direction::Right,
        };

        sprite.flip_x = match player.direction {
            Direction::Left => false,
            Direction::Right => true,
        };

        controller.velocity = velocity;
    }
}

pub fn player_level_follow_system(
    players: Query<&Transform, With<Player>>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut loaded_levels: ResMut<LoadedLevels>,
) {
    let levels = find_levels_around_positions(
        players
            .iter()
            .map(|p| p.translation.truncate())
            .collect(),
        LoadNeighbours::All,
        //
        &ldtk_projects,
        &ldtk_project_assets,
    );

    if loaded_levels.levels != levels {
        loaded_levels.levels = levels;
    }
}
