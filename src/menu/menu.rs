use bevy::prelude::*;

use crate::menu::menu_local::goto_local_menu;
use crate::menu::menu_online::goto_online_menu;
use crate::State;

pub trait AddMainMenuAppExt {
    fn add_main_menu(&mut self) -> &mut Self;
}

impl AddMainMenuAppExt for App {
    fn add_main_menu(&mut self) -> &mut Self {
        self.add_systems(OnEnter(State::MenuMain), setup)
            .add_systems(Update, update.run_if(in_state(State::MenuMain)))
            .add_systems(OnExit(State::MenuMain), cleanup)
    }
}

fn setup(next_state: ResMut<NextState<State>>) {
    match true {
        true => goto_online_menu(next_state),
        false => goto_local_menu(next_state),
    }
}

fn update() {}

fn cleanup() {}

pub fn goto_main_menu(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::MenuMain);
}
