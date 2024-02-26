use bevy::prelude::*;

use crate::game::AddGameAppExt;

pub mod core;
pub mod game;

fn main() {
    App::new().add_game().run();
}
