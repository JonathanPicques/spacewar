pub mod input;

use bevy::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_ecs_ldtk::ldtk::raw_level_accessor::RawLevelAccessor;
use bevy_ecs_ldtk::LevelIid;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{PlayerInputs, Rollback};
use bytemuck::Zeroable;

use crate::core::input::CoreInput;
use crate::core::levels::LoadedLevels;
use crate::game::conf::GameConfig;
use crate::game::player::input::{INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP};

#[derive(Eq, Ord, Hash, Clone, PartialEq, PartialOrd, Default, Component)]
pub struct Player {
    pub handle: usize,
}

pub fn player_system(mut all_players: Query<(&Player, &mut Transform), With<Rollback>>, inputs: Res<PlayerInputs<GameConfig>>) {
    let mut all_players = all_players.iter_mut().collect::<Vec<_>>();
    all_players.sort_by(|(player_a, ..), (player_b, ..)| player_a.cmp(player_b));

    for (player, mut transform) in all_players {
        let input = match inputs[player.handle] {
            (i, InputStatus::Confirmed) => i,
            (i, InputStatus::Predicted) => i,
            (_, InputStatus::Disconnected) => CoreInput::zeroed(),
        };

        if input.is_set(INPUT_UP) {
            transform.translation.y += 1.0;
        }
        if input.is_set(INPUT_DOWN) {
            transform.translation.y -= 1.0;
        }
        if input.is_set(INPUT_LEFT) {
            transform.translation.x -= 1.0;
        }
        if input.is_set(INPUT_RIGHT) {
            transform.translation.x += 1.0;
        }
    }
}

pub fn player_level_follow_system(
    players: Query<&Transform, With<Player>>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut loaded_levels: ResMut<LoadedLevels>,
) {
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("LDTk project resource not found");

    let levels = players
        .iter()
        .map(|player_transform| {
            {
                for level in ldtk_project.iter_raw_levels() {
                    let level_bounds = Rect {
                        min: Vec2::new(
                            level.world_x as f32,
                            level.world_y as f32,
                            //
                        ),
                        max: Vec2::new(
                            (level.world_x + level.px_wid) as f32,
                            (level.world_y + level.px_hei) as f32,
                        ),
                    };

                    if level_bounds.contains(player_transform.translation.truncate()) {
                        return Some(LevelIid::new(level.iid.clone()));
                    }
                }
                None
            }
        })
        .flatten()
        .collect();

    if levels != loaded_levels.levels {
        loaded_levels.levels = levels;
    }
}
