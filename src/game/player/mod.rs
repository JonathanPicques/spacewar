use bevy::prelude::*;
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::PlayerInputs;

use crate::game::core::input::{INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP};
use crate::game::GameConfig;

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

pub fn player_system(mut query: Query<(&Player, &mut Transform)>, inputs: Res<PlayerInputs<GameConfig>>) {
    for (player, mut transform) in query.iter_mut() {
        let bitflag = match inputs[player.handle] {
            (i, InputStatus::Confirmed) => i.input,
            (i, InputStatus::Predicted) => i.input,
            (_, InputStatus::Disconnected) => 0,
        };

        if bitflag & INPUT_UP != 0 {
            transform.translation.y += 1.0;
        }
        if bitflag & INPUT_DOWN != 0 {
            transform.translation.y -= 1.0;
        }
        if bitflag & INPUT_LEFT != 0 {
            transform.translation.x -= 1.0;
        }
        if bitflag & INPUT_RIGHT != 0 {
            transform.translation.x += 1.0;
        }
    }
}
