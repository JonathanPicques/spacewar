use bevy::prelude::*;
use bevy_ggrs::ggrs::{PlayerType, SessionBuilder};
use bevy_ggrs::{LocalPlayers, Session};

use crate::states::game::{transition_to_game, GameConfig, FPS, INPUT_DELAY, MAX_PREDICTION, NUM_PLAYERS};
use crate::states::State;

pub trait AddMenuAppExt {
    fn add_menu(&mut self) -> &mut Self;
}

impl AddMenuAppExt for App {
    fn add_menu(&mut self) -> &mut Self {
        self.add_systems(OnEnter(State::Menu), setup)
            .add_systems(Update, update.run_if(in_state(State::Menu)))
            .add_systems(OnExit(State::Menu), cleanup)
    }
}

fn setup() {}

fn update(commands: Commands, next_state: ResMut<NextState<State>>) {
    let mut session_builder = SessionBuilder::<GameConfig>::new()
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

    transition_to_game(
        commands,
        next_state,
        Session::SyncTest(session),
        LocalPlayers((0..NUM_PLAYERS).collect()),
    );
}

fn cleanup() {}

pub fn transition_to_menu(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::Menu);
}
