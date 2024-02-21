pub mod input;

use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{PlayerInputs, Rollback};
use bytemuck::Zeroable;

use crate::core::input::CoreInput;
use crate::game::conf::GameConfig;
use crate::game::player::input::{INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP};

#[derive(Eq, Ord, Clone, PartialEq, PartialOrd, Default, Component)]
pub struct Player {
    pub handle: usize,
}

pub fn player_system(mut commands: Commands, mut all_players: Query<(&Player, &mut Transform), With<Rollback>>, inputs: Res<PlayerInputs<GameConfig>>) {
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

        if transform.translation.x < 100.0 {
            commands.insert_resource(LevelSelection::index(0));
        } else {
            commands.insert_resource(LevelSelection::index(1));
        }
    }
}
