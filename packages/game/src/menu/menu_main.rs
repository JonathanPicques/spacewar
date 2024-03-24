use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::menu::menu_local::goto_local_menu;
use crate::menu::menu_online::goto_online_menu;
use crate::GameArgs;
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

fn setup(mut args: ResMut<GameArgs>, mut next_state: ResMut<NextState<State>>) {
    if args.local {
        args.local = false;
        goto_local_menu(&mut next_state);
    }
    if args.online {
        args.online = false;
        goto_online_menu(&mut next_state);
    }
}

fn update(
    mut contexts: EguiContexts,
    //
    mut next_state: ResMut<NextState<State>>,
) {
    egui::panel::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        if ui.button("Local").clicked() {
            goto_local_menu(&mut next_state);
        } else if ui.button("Online").clicked() {
            goto_online_menu(&mut next_state);
        }
    });
}

fn cleanup() {}

pub fn goto_main_menu(next_state: &mut NextState<State>) {
    next_state.set(State::MenuMain);
}
