use bevy::prelude::*;
use bevy_ggrs::ggrs::{PlayerType, SessionBuilder};
use bevy_ggrs::{LocalPlayers, Session};

use crate::game::goto_game;
use crate::GameConfig;
use crate::{GameArgs, State};

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
    game_args: Res<GameArgs>,
    mut next_state: ResMut<NextState<State>>,
) {
    let mut session_builder = SessionBuilder::<GameConfig>::new()
        .with_fps(game_args.fps)
        .expect("Invalid FPS")
        .with_max_prediction_window(game_args.max_prediction)
        .with_num_players(game_args.num_players)
        .with_input_delay(game_args.input_delay)
        .with_check_distance(game_args.check_distance);

    for handle in 0..game_args.num_players {
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
        //
        &game_args,
        Session::SyncTest(session),
        LocalPlayers((0..game_args.num_players).collect()),
    );
}

fn cleanup() {}

pub fn goto_local_menu(next_state: &mut NextState<State>) {
    next_state.set(State::MenuLocal);
}
