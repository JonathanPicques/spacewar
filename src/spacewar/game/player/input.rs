use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{LocalInputs, LocalPlayers};

use crate::core::input::CoreInput;
use crate::spacewar::conf::{GameArgs, GameConfig};

pub const INPUT_UP: u8 = 1 << 1;
pub const INPUT_DOWN: u8 = 1 << 2;
pub const INPUT_LEFT: u8 = 1 << 3;
pub const INPUT_RIGHT: u8 = 1 << 4;
pub const INPUT_JUMP: u8 = 1 << 5;

pub fn input_system(
    mut commands: Commands,
    //
    game_ars: Res<GameArgs>,
    local_players: Res<LocalPlayers>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let local_players = &local_players.0;
    let mut local_inputs = HashMap::new();

    for handle in local_players.iter() {
        let local = local_players.len() > 1;
        let mut input = CoreInput::default();

        if game_ars.randomize_input {
            input.set(Box::into_raw(Box::new(0xDEAD)) as u8);
        } else if !local || *handle == 0 {
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
            if keyboard_input.pressed(KeyCode::Numpad0) {
                input.set(INPUT_JUMP);
            }
        } else {
            if keyboard_input.pressed(KeyCode::Z) {
                input.set(INPUT_UP);
            }
            if keyboard_input.pressed(KeyCode::Q) {
                input.set(INPUT_LEFT);
            }
            if keyboard_input.pressed(KeyCode::S) {
                input.set(INPUT_DOWN);
            }
            if keyboard_input.pressed(KeyCode::D) {
                input.set(INPUT_RIGHT);
            }
            if keyboard_input.pressed(KeyCode::Space) {
                input.set(INPUT_JUMP);
            }
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<GameConfig>(local_inputs));
}
