use bevy::prelude::*;
use bevy_ggrs::ggrs::{PlayerType, SessionBuilder};
use bevy_ggrs::{LocalPlayers, Session};

use crate::game::{goto_game, CoreConfig, FPS, INPUT_DELAY, MAX_PREDICTION, NUM_PLAYERS};
use crate::State;

pub trait AddLocalMenuAppExt {
    fn add_local_menu(&mut self) -> &mut Self;
}

impl AddLocalMenuAppExt for App {
    fn add_local_menu(&mut self) -> &mut Self {
        self.add_systems(OnEnter(State::MenuLocal), setup)
            .add_systems(Update, update.run_if(in_state(State::MenuLocal)))
            .add_systems(OnExit(State::MenuLocal), cleanup)
    }
}

fn setup() {}

fn update(commands: Commands, next_state: ResMut<NextState<State>>) {
    let mut session_builder = SessionBuilder::<CoreConfig>::new()
        .with_fps(FPS)
        .expect("Invalid FPS")
        .with_max_prediction_window(MAX_PREDICTION)
        .expect("Invalid prediction window")
        .with_num_players(NUM_PLAYERS)
        .with_input_delay(INPUT_DELAY)
        .with_check_distance(0);

    for handle in 0..NUM_PLAYERS {
        session_builder = session_builder
            .add_player(PlayerType::Local, handle)
            .expect("Could not add local player");
    }

    let session = session_builder
        .start_synctest_session()
        .expect("Session could not be started");

    goto_game(
        commands,
        next_state,
        Session::SyncTest(session),
        LocalPlayers((0..NUM_PLAYERS).collect()),
    );
}

fn cleanup() {}

pub fn goto_local_menu(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::MenuLocal);
}
