use bevy::prelude::*;
use bevy_ggrs::ggrs::{PlayerType, SessionBuilder};
use bevy_ggrs::{LocalPlayers, Session};
use bevy_matchbox::matchbox_socket::{PeerState, SingleChannel};
use bevy_matchbox::MatchboxSocket;

use crate::game::conf::{GameConfig, State, FPS, INPUT_DELAY, MATCHBOX_ADDRESS, MAX_PREDICTION, NUM_PLAYERS};
use crate::game::goto_game;

pub trait AddOnlineMenuAppExt {
    fn add_online_menu(&mut self) -> &mut Self;
}

impl AddOnlineMenuAppExt for App {
    fn add_online_menu(&mut self) -> &mut Self {
        self.add_systems(OnEnter(State::MenuOnline), setup)
            .add_systems(Update, update.run_if(in_state(State::MenuOnline)))
            .add_systems(OnExit(State::MenuOnline), cleanup)
    }
}

fn setup(mut commands: Commands) {
    commands.insert_resource(MatchboxSocket::new_ggrs(MATCHBOX_ADDRESS));
}

fn update(commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>, next_state: ResMut<NextState<State>>) {
    for (peer, new_state) in socket.update_peers() {
        match new_state {
            PeerState::Connected => info!("peer {peer} connected"),
            PeerState::Disconnected => info!("peer {peer} disconnected"),
        }
    }

    if socket.players().len() >= NUM_PLAYERS {
        let mut session_builder = SessionBuilder::<GameConfig>::new()
            .with_fps(FPS)
            .expect("Invalid FPS")
            .with_max_prediction_window(MAX_PREDICTION)
            .expect("Invalid max prediction window")
            .with_num_players(NUM_PLAYERS)
            .with_input_delay(INPUT_DELAY);

        let mut handles = Vec::new();
        for (i, player_type) in socket.players().iter().enumerate() {
            if *player_type == PlayerType::Local {
                handles.push(i);
            }
            session_builder = session_builder
                .add_player(*player_type, i)
                .expect("Invalid player");
        }

        let channel = socket.take_channel(0).expect("Channel expected");
        let session = session_builder
            .start_p2p_session(channel)
            .expect("P2P Session could not be started");

        goto_game(
            commands,
            next_state,
            Session::P2P(session),
            LocalPlayers(handles),
        );
    }
}

fn cleanup() {}

pub fn goto_online_menu(mut next_state: ResMut<NextState<State>>) {
    next_state.set(State::MenuOnline);
}
