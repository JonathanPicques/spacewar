use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy_ggrs::ggrs::{PlayerType, SessionBuilder};
use bevy_ggrs::{LocalPlayers, Session};
use bevy_matchbox::matchbox_socket::{PeerState, SingleChannel};
use bevy_matchbox::MatchboxSocket;

use crate::spacewar::conf::{GameArgs, GameConfig, State};
use crate::spacewar::game::goto_game;
use crate::spacewar::menu::menu_main::goto_main_menu;

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
        "ws://192.168.1.50:3536/lobby?next={}",
        // "ws://127.0.0.1:3536/lobby?next={}",
        args.num_players
    );

    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

fn update(
    mut commands: Commands,
    mut contexts: EguiContexts,
    //
    args: Res<GameArgs>,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut next_state: ResMut<NextState<State>>,
) {
    for (peer, new_state) in socket.update_peers() {
        match new_state {
            PeerState::Connected => info!("peer {peer} connected"),
            PeerState::Disconnected => info!("peer {peer} disconnected"),
        }
    }

    egui::panel::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        if ui.button("Back").clicked() {
            goto_main_menu(&mut next_state);
            return;
        }
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
            .with_fps(args.fps)
            .expect("Invalid FPS")
            .with_max_prediction_window(args.max_prediction)
            .expect("Invalid max prediction window")
            .with_num_players(args.num_players)
            .with_input_delay(args.input_delay);

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
            &mut commands,
            &mut next_state,
            Session::P2P(session),
            LocalPlayers(handles),
        );
    }
}

fn cleanup(mut commands: Commands) {
    commands.remove_resource::<MatchboxSocket<SingleChannel>>();
}

pub fn goto_online_menu(next_state: &mut NextState<State>) {
    next_state.set(State::MenuOnline);
}
