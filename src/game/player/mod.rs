use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::PlayerInputs;
use bytemuck::Zeroable;

use crate::game::core::input::{CoreInput, INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP};
use crate::game::CoreConfig;

#[derive(Eq, Ord, Clone, PartialEq, PartialOrd, Default, Component)]
pub struct Player {
    pub handle: usize,
}

pub fn player_system(mut query: Query<(&Player, &mut Transform)>, mut commands: Commands, inputs: Res<PlayerInputs<CoreConfig>>) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(player_a, ..), (player_b, ..)| player_a.cmp(player_b));

    for (player, mut transform) in query {
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
