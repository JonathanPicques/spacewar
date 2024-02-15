use bevy::{prelude::*, utils::HashMap};
use bevy_ggrs::ggrs::InputStatus;
use bevy_ggrs::{ggrs::Config, GgrsApp, GgrsPlugin, ReadInputs};
use bevy_ggrs::{GgrsSchedule, LocalInputs, LocalPlayers, PlayerInputs, Session};
use bevy_matchbox::prelude::*;

use crate::states::State;

pub const FPS: usize = 60;
pub const INPUT_DELAY: usize = 2;
pub const NUM_PLAYERS: usize = 2;
pub const MAX_PREDICTION: usize = 12;

const INPUT_UP: u8 = 1 << 0;
const INPUT_DOWN: u8 = 1 << 1;
const INPUT_LEFT: u8 = 1 << 2;
const INPUT_RIGHT: u8 = 1 << 3;

#[derive(Component)]
pub struct Game {}

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Debug)]
pub struct GameConfig;
impl Config for GameConfig {
    type Input = u8;
    type State = u8;
    type Address = PeerId;
}

pub trait AddGameAppExt {
    fn add_game(&mut self) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self) -> &mut Self {
        self.add_plugins(GgrsPlugin::<GameConfig>::default())
            .add_systems(ReadInputs, read_inputs)
            .set_rollback_schedule_fps(FPS)
            //
            .rollback_component_with_clone::<Transform>()
            //
            .add_systems(OnEnter(State::Game), setup)
            .add_systems(GgrsSchedule, (update).chain())
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

fn update(mut query: Query<(&Player, &mut Transform)>, inputs: Res<PlayerInputs<GameConfig>>) {
    for (player, mut transform) in query.iter_mut() {
        let input = match inputs[player.handle].1 {
            InputStatus::Confirmed => inputs[player.handle].0,
            InputStatus::Predicted => inputs[player.handle].0,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };

        if input & INPUT_UP != 0 {
            transform.translation.y += 1.0;
        }
        if input & INPUT_DOWN != 0 {
            transform.translation.y -= 1.0;
        }
        if input & INPUT_LEFT != 0 {
            transform.translation.x -= 1.0;
        }
        if input & INPUT_RIGHT != 0 {
            transform.translation.x += 1.0;
        }
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Game>>) {
    commands.remove_resource::<LocalPlayers>();
    commands.remove_resource::<Session<GameConfig>>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn read_inputs(mut commands: Commands, local_players: Res<LocalPlayers>, keyboard_input: Res<Input<KeyCode>>) {
    let mut local_inputs = HashMap::new();

    for handle in &local_players.0 {
        let mut input: u8 = 0;

        if *handle == 0 {
            if keyboard_input.pressed(KeyCode::Z) {
                input |= INPUT_UP;
            }
            if keyboard_input.pressed(KeyCode::Q) {
                input |= INPUT_LEFT;
            }
            if keyboard_input.pressed(KeyCode::S) {
                input |= INPUT_DOWN;
            }
            if keyboard_input.pressed(KeyCode::D) {
                input |= INPUT_RIGHT;
            }
        } else {
            if keyboard_input.pressed(KeyCode::Up) {
                input |= INPUT_UP;
            }
            if keyboard_input.pressed(KeyCode::Left) {
                input |= INPUT_LEFT;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                input |= INPUT_DOWN;
            }
            if keyboard_input.pressed(KeyCode::Right) {
                input |= INPUT_RIGHT;
            }
        }

        local_inputs.insert(*handle, input);
    }

    commands.insert_resource(LocalInputs::<GameConfig>(local_inputs));
}

pub fn transition_to_game(mut commands: Commands, mut next_state: ResMut<NextState<State>>, session: Session<GameConfig>, local_players: LocalPlayers) {
    commands.insert_resource(session);
    commands.insert_resource(local_players);
    next_state.set(State::Game);
}
