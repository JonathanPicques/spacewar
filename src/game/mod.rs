pub mod conf;
pub mod menu;
pub mod player;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};
use bevy_ggrs::{GgrsApp, GgrsPlugin, ReadInputs};
use bevy_ggrs::{GgrsSchedule, LocalPlayers, Session};
use clap::Parser;

use crate::core::loader::CoreDynamicAssetCollection;
use crate::core::utilities::ArgsPlugin;
use crate::game::conf::{Assets, GameConfig, State, FPS, INPUT_DELAY, MAX_PREDICTION, NUM_PLAYERS};
use crate::game::menu::menu::AddMainMenuAppExt;
use crate::game::menu::menu_local::AddLocalMenuAppExt;
use crate::game::menu::menu_online::AddOnlineMenuAppExt;
use crate::game::player::input::input_system;
use crate::game::player::{player_system, Player};

#[derive(Copy, Clone, Component)]
pub struct Game {}

#[derive(Parser, Resource)]
pub struct GameArgs {
    #[clap(long, short = 'l', default_value = "false")]
    local: bool,
}

pub trait AddGameAppExt {
    fn add_game(&mut self) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self) -> &mut Self {
        self.add_state::<State>()
            //
            .add_main_menu()
            .add_local_menu()
            .add_online_menu()
            //
            .add_plugins(LdtkPlugin)
            .add_plugins(ArgsPlugin::<GameArgs>::default())
            .add_plugins(GgrsPlugin::<GameConfig>::default())
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
            //
            .add_loading_state(
                LoadingState::new(State::Load)
                    .continue_to_state(State::MenuMain)
                    .with_dynamic_assets_file::<CoreDynamicAssetCollection>("assets.ron")
                    .register_dynamic_asset_collection::<CoreDynamicAssetCollection>()
                    //
                    .load_collection::<Assets>(),
            )
    }
}

fn setup(mut commands: Commands, texture_assets: Res<Assets>) {
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
