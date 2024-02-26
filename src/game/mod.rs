pub mod conf;
pub mod menu;
pub mod player;

use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelIid, LevelSpawnBehavior};
use bevy_egui::EguiPlugin;
use bevy_ggrs::{AddRollbackCommandExtension, GgrsApp, GgrsPlugin, ReadInputs};
use bevy_ggrs::{GgrsSchedule, LocalPlayers, Session};
use clap::Parser;

use crate::core::anim::{sprite_sheet_animation_system, SpriteSheetAnimation};
use crate::core::levels::{load_levels_system, LoadedLevels};
use crate::core::loader::CoreDynamicAssetCollection;
use crate::core::physics::{player_controller_system, PlayerController};
use crate::core::utilities::hash::transform_hasher;
use crate::game::conf::{GameArgs, GameAssets, GameConfig, State};
use crate::game::menu::menu_local::AddLocalMenuAppExt;
use crate::game::menu::menu_main::AddMainMenuAppExt;
use crate::game::menu::menu_online::AddOnlineMenuAppExt;
use crate::game::player::input::input_system;
use crate::game::player::{player_level_follow_system, player_system, Player};

type DynamicAssetPlugin = RonAssetPlugin<CoreDynamicAssetCollection>;

#[derive(Copy, Clone, Component)]
pub struct Game {}

pub trait AddGameAppExt {
    fn add_game(&mut self) -> &mut Self;
}

impl AddGameAppExt for App {
    fn add_game(&mut self) -> &mut Self {
        let args = GameArgs::parse();
        let args_fps = args.fps;

        self
            //
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            fit_canvas_to_parent: true,
                            prevent_default_event_handling: false,
                            ..default()
                        }),
                        ..default()
                    }),
            )
            .insert_resource(Msaa::Off)
            //
            .add_plugins(EguiPlugin)
            .add_plugins(DynamicAssetPlugin::new(&["ron"]))
            //
            .add_plugins(LdtkPlugin)
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation { load_level_neighbors: false },
                ..default()
            })
            .insert_resource(LoadedLevels::new(LevelIid::new(
                "a2a50ff0-66b0-11ec-9cd7-c721746049b9",
            )))
            //
            .insert_resource(args)
            //
            .add_plugins(GgrsPlugin::<GameConfig>::default())
            .add_systems(ReadInputs, input_system)
            .set_rollback_schedule_fps(args_fps)
            .checksum_resource_with_hash::<LoadedLevels>()
            .checksum_component::<Transform>(transform_hasher)
            .checksum_component_with_hash::<Player>()
            .checksum_component_with_hash::<PlayerController>()
            .rollback_resource_with_clone::<LoadedLevels>()
            .rollback_component_with_copy::<PlayerController>()
            .rollback_component_with_clone::<Player>()
            .rollback_component_with_clone::<Transform>()
            //
            .add_state::<State>()
            //
            .add_main_menu()
            .add_local_menu()
            .add_online_menu()
            //
            .add_systems(OnEnter(State::Game), setup)
            .add_systems(
                GgrsSchedule,
                ((
                    player_system,
                    load_levels_system,
                    player_controller_system,
                    player_level_follow_system,
                    sprite_sheet_animation_system,
                )
                    .run_if(in_state(State::Game)))
                .chain(),
            )
            .add_systems(OnExit(State::Game), cleanup)
            //
            .add_loading_state(
                LoadingState::new(State::Load)
                    .continue_to_state(State::MenuMain)
                    .with_dynamic_assets_file::<CoreDynamicAssetCollection>("assets.ron")
                    .register_dynamic_asset_collection::<CoreDynamicAssetCollection>()
                    //
                    .load_collection::<GameAssets>(),
            )
    }
}

fn setup(mut commands: Commands, args: Res<GameArgs>, texture_assets: Res<GameAssets>) {
    let game = Game {};

    commands.spawn((game, Camera2dBundle::default()));
    commands.spawn((
        game,
        LdtkWorldBundle {
            ldtk_handle: texture_assets.tileset_project.clone(),
            ..default()
        },
    ));

    for handle in 0..args.num_players {
        let transform = Transform::from_translation(Vec3::new((handle * 32) as f32, 1.0, 5.0));
        commands
            .spawn((
                game,
                Player { handle },
                PlayerController::default(),
                SpriteSheetBundle {
                    transform,
                    texture_atlas: texture_assets.player.clone(),
                    ..default()
                },
                SpriteSheetAnimation {
                    timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
                    start: 0,
                    finish: 3,
                },
            ))
            .add_rollback();
    }
}

fn cleanup(mut commands: Commands, query: Query<Entity, With<Game>>) {
    commands.remove_resource::<LocalPlayers>();
    commands.remove_resource::<Session<GameConfig>>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn goto_game(commands: &mut Commands, next_state: &mut NextState<State>, session: Session<GameConfig>, local_players: LocalPlayers) {
    commands.insert_resource(session);
    commands.insert_resource(local_players);
    next_state.set(State::Game);
}
