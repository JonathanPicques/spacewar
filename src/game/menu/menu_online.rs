use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_ggrs::ggrs::{PlayerType, SessionBuilder};
use bevy_ggrs::{LocalPlayers, Session};
use bevy_matchbox::matchbox_socket::{PeerState, SingleChannel};
use bevy_matchbox::MatchboxSocket;

use crate::game::conf::{GameArgs, GameConfig, State, FPS, INPUT_DELAY, MAX_PREDICTION};
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

fn setup(mut commands: Commands, args: Res<GameArgs>) {
    let room_url = format!(
        "ws://127.0.0.1:3536/lobby?next={}",
        args.num_players
    );

    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn update(
    commands: Commands,
    args: Res<GameArgs>,
    mut ctx: EguiContexts,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    next_state: ResMut<NextState<State>>,
) {
    for (peer, new_state) in socket.update_peers() {
        match new_state {
            PeerState::Connected => info!("peer {peer} connected"),
            PeerState::Disconnected => info!("peer {peer} disconnected"),
        }
    }

    egui::panel::CentralPanel::default().show(ctx.ctx_mut(), |ui| {
        ui.label(format!(
            "Waiting for {} other players...",
            args.num_players - socket.players().len()
        ));
        for player in socket.players().iter() {
            match player {
                PlayerType::Local => {}
                PlayerType::Remote(peer_id) => {
                    ui.label(format!("remote player {}", peer_id));
                }
                PlayerType::Spectator(peer_id) => {
                    ui.label(format!("spectator player {}", peer_id));
                }
            }
        }
    });

    if socket.players().len() >= args.num_players {
        let mut session_builder = SessionBuilder::<GameConfig>::new()
            .with_fps(FPS)
            .expect("Invalid FPS")
            .with_max_prediction_window(MAX_PREDICTION)
            .expect("Invalid max prediction window")
            .with_num_players(args.num_players)
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

pub fn goto_online_menu(next_state: &mut NextState<State>) {
    next_state.set(State::MenuOnline);
}
