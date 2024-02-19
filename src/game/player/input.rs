use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{LocalInputs, LocalPlayers};

use crate::core::input::CoreInput;
use crate::game::conf::GameConfig;

pub const INPUT_UP: u8 = 1 << 1;
pub const INPUT_DOWN: u8 = 1 << 2;
pub const INPUT_LEFT: u8 = 1 << 3;
pub const INPUT_RIGHT: u8 = 1 << 4;
pub const INPUT_JUMP: u8 = 1 << 5;

pub fn input_system(mut commands: Commands, local_players: Res<LocalPlayers>, keyboard_input: Res<Input<KeyCode>>) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input = CoreInput::new();

        if keyboard_input.pressed(KeyCode::Up) {
            input.set(INPUT_UP);
        }
        if keyboard_input.pressed(KeyCode::Left) {
            input.set(INPUT_LEFT);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            input.set(INPUT_DOWN);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            input.set(INPUT_RIGHT);
        }
        if keyboard_input.pressed(KeyCode::Space) {
            input.set(INPUT_JUMP);
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<GameConfig>(local_inputs));
}
