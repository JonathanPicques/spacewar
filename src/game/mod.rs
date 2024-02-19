pub mod core;
pub mod menu;
pub mod player;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_ggrs::{GgrsApp, GgrsPlugin, ReadInputs};
use bevy_ggrs::{GgrsSchedule, LocalPlayers, Session};

use crate::game::core::input::input_system;
use crate::game::core::CoreConfig;
use crate::game::player::{player_system, Player};
use crate::{State, TextureAssets};

pub const FPS: usize = 60;
pub const INPUT_DELAY: usize = 2;
pub const NUM_PLAYERS: usize = 2;
pub const MAX_PREDICTION: usize = 12;

#[derive(Copy, Clone, Component)]
pub struct Game {}

pub trait AddGameAppExt {
    fn add_game(&mut self) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self) -> &mut Self {
        self.add_plugins(LdtkPlugin)
            .add_plugins(GgrsPlugin::<CoreConfig>::default())
            .add_systems(ReadInputs, input_system)
            .set_rollback_schedule_fps(FPS)
            //
            .rollback_resource_with_clone::<LevelSelection>()
            //
            .rollback_component_with_clone::<Player>()
            .rollback_component_with_clone::<Transform>()
            //
            .add_systems(OnEnter(State::Game), setup)
            .add_systems(GgrsSchedule, (player_system).chain())
            .add_systems(OnExit(State::Game), cleanup)
            //
            .insert_resource(LevelSelection::index(0))
    }
}

fn setup(mut commands: Commands, texture_assets: Res<TextureAssets>) {
    let game = Game {};

    commands.spawn((game, Camera2dBundle::default()));
    commands.spawn((
        game,
        LdtkWorldBundle {
            ldtk_handle: texture_assets.tileset_project.clone(),
            ..Default::default()
        },
    ));

    for handle in 0..NUM_PLAYERS {
        let transform = Transform::from_translation(Vec3::new((handle * 10) as f32, 1.0, 1.0));
        commands.spawn((
            game,
            Player { handle },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(12.0, 14.0)),
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
    commands.remove_resource::<Session<CoreConfig>>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn goto_game(mut commands: Commands, mut next_state: ResMut<NextState<State>>, session: Session<CoreConfig>, local_players: LocalPlayers) {
    commands.insert_resource(session);
    commands.insert_resource(local_players);
    next_state.set(State::Game);
}
