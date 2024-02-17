pub mod core;
pub mod menu;
pub mod player;

use bevy::prelude::*;
use bevy_ggrs::{ggrs::Config, GgrsApp, GgrsPlugin, ReadInputs};
use bevy_ggrs::{GgrsSchedule, LocalPlayers, Session};
use bevy_matchbox::prelude::*;

use crate::game::core::input::{input_system, GameInput};
use crate::game::player::{player_system, Player};
use crate::State;

pub const FPS: usize = 60;
pub const INPUT_DELAY: usize = 2;
pub const NUM_PLAYERS: usize = 2;
pub const MAX_PREDICTION: usize = 12;

#[derive(Component)]
pub struct Game {}

#[derive(Debug)]
pub struct GameConfig;
impl Config for GameConfig {
    type Input = GameInput;
    type State = u8;
    type Address = PeerId;
}

pub trait AddGameAppExt {
    fn add_game(&mut self) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self) -> &mut Self {
        self.add_plugins(GgrsPlugin::<GameConfig>::default())
            .add_systems(ReadInputs, input_system)
            .set_rollback_schedule_fps(FPS)
            //
            .rollback_component_with_clone::<Transform>()
            //
            .add_systems(OnEnter(State::Game), setup)
            .add_systems(GgrsSchedule, (player_system).chain())
            .add_systems(OnExit(State::Game), cleanup)
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Game {}, Camera2dBundle::default()));

    for handle in 0..NUM_PLAYERS {
        let transform = Transform::from_translation(Vec3::new((handle * 2) as f32, 1.0, 1.0));
        commands.spawn((
            Game {},
            Player { handle },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(2.0, 2.0)),
                    ..Default::default()
                },
                transform,
                ..Default::default()
            },
        ));
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Game>>) {
    commands.remove_resource::<LocalPlayers>();
    commands.remove_resource::<Session<GameConfig>>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn goto_game(mut commands: Commands, mut next_state: ResMut<NextState<State>>, session: Session<GameConfig>, local_players: LocalPlayers) {
    commands.insert_resource(session);
    commands.insert_resource(local_players);
    next_state.set(State::Game);
}
