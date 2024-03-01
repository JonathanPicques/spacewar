use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{LocalInputs, LocalPlayers};

use crate::core::input::CoreInput;
use crate::spacewar::{GameArgs, GameConfig};

pub const INPUT_UP: u8 = 1 << 1;
pub const INPUT_DOWN: u8 = 1 << 2;
pub const INPUT_LEFT: u8 = 1 << 3;
pub const INPUT_RIGHT: u8 = 1 << 4;
pub const INPUT_SHOOT: u8 = 1 << 5;

pub fn input_system(
    mut commands: Commands,
    //
    game_ars: Res<GameArgs>,
    local_players: Res<LocalPlayers>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let local_players = &local_players.0;
    let mut local_inputs = HashMap::new();

    for handle in local_players.iter() {
        let local = local_players.len() > 1;
        let mut input = CoreInput::default();

        if game_ars.randomize_input {
            input.set(Box::into_raw(Box::new(0xDEAD)) as u8);
        } else if !local || *handle == 0 {
            if keyboard_input.pressed(KeyCode::ArrowUp) {
                input.set(INPUT_UP);
            }
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                input.set(INPUT_LEFT);
            }
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                input.set(INPUT_DOWN);
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                input.set(INPUT_RIGHT);
            }
            if keyboard_input.pressed(KeyCode::ControlRight) {
                input.set(INPUT_SHOOT);
            }
        } else {
            if keyboard_input.pressed(KeyCode::KeyZ) {
                input.set(INPUT_UP);
            }
            if keyboard_input.pressed(KeyCode::KeyQ) {
                input.set(INPUT_LEFT);
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                input.set(INPUT_DOWN);
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                input.set(INPUT_RIGHT);
            }
            if keyboard_input.pressed(KeyCode::KeyE) {
                input.set(INPUT_SHOOT);
            }
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<GameConfig>(local_inputs));
}
