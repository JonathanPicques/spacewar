use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::{LocalInputs, LocalPlayers};
use bytemuck::{Pod, Zeroable};

use crate::game::GameConfig;

pub const INPUT_UP: u8 = 1 << 1;
pub const INPUT_DOWN: u8 = 1 << 2;
pub const INPUT_LEFT: u8 = 1 << 3;
pub const INPUT_RIGHT: u8 = 1 << 4;
pub const INPUT_JUMP: u8 = 1 << 5;

#[repr(C)]
#[derive(Eq, Pod, Copy, Clone, Zeroable, PartialEq)]
pub struct GameInput {
    pub input: u8,
}

impl GameInput {
    pub fn is_empty(self) -> bool {
        self.input == 0
    }
}

pub fn input_system(mut commands: Commands, local_players: Res<LocalPlayers>, keyboard_input: Res<Input<KeyCode>>) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input: u8 = 0;

        if keyboard_input.pressed(KeyCode::Up) {
            input |= INPUT_UP;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            input |= INPUT_LEFT;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            input |= INPUT_DOWN;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            input |= INPUT_RIGHT;
        }
        if keyboard_input.pressed(KeyCode::Space) {
            input |= INPUT_JUMP;
        }

        local_inputs.insert(*handle, GameInput { input });
    }

    commands.insert_resource(LocalInputs::<GameConfig>(local_inputs));
}
