use bevy::prelude::*;

pub mod game;
pub mod menu;

#[derive(Eq, Hash, Clone, Debug, States, Default, PartialEq)]
pub enum State {
    #[default]
    Load,
    //
    Menu,
    MenuConnect,
    //
    Game,
}
