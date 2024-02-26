pub mod input;

use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{PlayerInputs, Rollback};
use bytemuck::Zeroable;

use crate::core::anim::SpriteSheetAnimation;
use crate::core::input::CoreInput;
use crate::core::levels::{find_levels_around_positions, LoadedLevels};
use crate::core::physics::PlayerController;
use crate::core::utilities::maths::{compute_acceleration, compute_deceleration};
use crate::spacewar::conf::GameConfig;
use crate::spacewar::game::player::input::{INPUT_JUMP, INPUT_LEFT, INPUT_RIGHT};

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
    mut all_players: Query<
        (
            &mut Player,
            &mut PlayerController,
            &mut TextureAtlasSprite,
            &mut SpriteSheetAnimation,
        ),
        With<Rollback>,
    >,
    time: Res<Time>,
    inputs: Res<PlayerInputs<GameConfig>>,
) {
    let mut all_players = all_players.iter_mut().collect::<Vec<_>>();
    all_players.sort_by(|(player_a, ..), (player_b, ..)| player_a.cmp(player_b));

    for (mut player, mut player_controller, mut player_sprite, mut sprite_sheet_animation) in all_players {
        let input = match inputs[player.handle] {
            (i, InputStatus::Confirmed) => i,
            (i, InputStatus::Predicted) => i,
            (_, InputStatus::Disconnected) => CoreInput::zeroed(),
        };
        let mut velocity = player_controller.velocity;

        if input.is_set(INPUT_JUMP) && player_controller.is_on_floor() {
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

        if player_controller.is_on_floor() {
            if velocity.x != 0.0 {
                sprite_sheet_animation.start = 20;
                sprite_sheet_animation.finish = 29;
            } else {
                sprite_sheet_animation.start = 0;
                sprite_sheet_animation.finish = 3;
            }
        } else {
            sprite_sheet_animation.start = 12;
            sprite_sheet_animation.finish = 12;
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

        player_sprite.flip_x = match player.direction {
            Direction::Left => false,
            Direction::Right => true,
        };

        player_controller.velocity = velocity;
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
        ldtk_projects,
        ldtk_project_assets,
    );

    if loaded_levels.levels != levels {
        loaded_levels.levels = levels;
    }
}
