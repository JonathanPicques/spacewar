use bevy::prelude::*;
use bevy_ggrs::ggrs::{PlayerType, SessionBuilder};
use bevy_ggrs::{LocalPlayers, Session};

use crate::spacewar::game::goto_game;
use crate::spacewar::GameConfig;
use crate::spacewar::{GameArgs, State};

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

fn update(
    mut commands: Commands,
    //
    args: Res<GameArgs>,
    mut next_state: ResMut<NextState<State>>,
) {
    let mut session_builder = SessionBuilder::<GameConfig>::new()
        .with_fps(args.fps)
        .expect("Invalid FPS")
        .with_max_prediction_window(args.max_prediction)
        .expect("Invalid prediction window")
        .with_num_players(args.num_players)
        .with_input_delay(args.input_delay)
        .with_check_distance(args.check_distance);

    for handle in 0..args.num_players {
        session_builder = session_builder
            .add_player(PlayerType::Local, handle)
            .expect("Could not add local player");
    }

    let session = session_builder
        .start_synctest_session()
        .expect("Session could not be started");

    goto_game(
        &mut commands,
        &mut next_state,
        Session::SyncTest(session),
        LocalPlayers((0..args.num_players).collect()),
    );
}

fn cleanup() {}

pub fn goto_local_menu(next_state: &mut NextState<State>) {
    next_state.set(State::MenuLocal);
}
