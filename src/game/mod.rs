pub mod conf;
pub mod menu;
pub mod player;

use std::time::Duration;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_ecs_ldtk::{LdtkPlugin, LdtkSettings, LdtkWorldBundle, LevelIid, LevelSpawnBehavior};
use bevy_ggrs::{AddRollbackCommandExtension, GgrsApp, GgrsPlugin, ReadInputs};
use bevy_ggrs::{GgrsSchedule, LocalPlayers, Session};

use crate::core::anim::{sprite_sheet_animation_system, SpriteSheetAnimation};
use crate::core::levels::{load_levels_system, LoadedLevels};
use crate::core::loader::CoreDynamicAssetCollection;
use crate::core::utilities::args::ArgsPlugin;
use crate::core::utilities::hasher::transform_hasher;
use crate::game::conf::{GameArgs, GameAssets, GameConfig, State, FPS, INPUT_DELAY, MAX_PREDICTION, NUM_PLAYERS};
use crate::game::menu::menu_local::AddLocalMenuAppExt;
use crate::game::menu::menu_main::AddMainMenuAppExt;
use crate::game::menu::menu_online::AddOnlineMenuAppExt;
use crate::game::player::input::input_system;
use crate::game::player::{player_level_follow_system, player_system, Player};

#[derive(Copy, Clone, Component)]
pub struct Game {}

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
            .insert_resource(LdtkSettings {
                level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation { load_level_neighbors: false },
                ..default()
            })
            .insert_resource(LoadedLevels::new(LevelIid::new(
                "a2a50ff0-66b0-11ec-9cd7-c721746049b9",
            )))
            //
            .add_plugins(ArgsPlugin::<GameArgs>::default())
            //
            .add_plugins(GgrsPlugin::<GameConfig>::default())
            .add_systems(ReadInputs, input_system)
            .set_rollback_schedule_fps(FPS)
            .checksum_resource_with_hash::<LoadedLevels>()
            .checksum_component::<Transform>(transform_hasher)
            .checksum_component_with_hash::<Player>()
            .rollback_resource_with_clone::<LoadedLevels>()
            .rollback_component_with_clone::<Player>()
            .rollback_component_with_clone::<Transform>()
            //
            .add_systems(OnEnter(State::Game), setup)
            .add_systems(
                GgrsSchedule,
                ((
                    player_system,
                    load_levels_system,
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

fn setup(mut commands: Commands, texture_assets: Res<GameAssets>) {
    let game = Game {};

    commands.spawn((game, Camera2dBundle::default()));
    commands.spawn((
        game,
        LdtkWorldBundle {
            ldtk_handle: texture_assets.tileset_project.clone(),
            ..default()
        },
    ));

    for handle in 0..NUM_PLAYERS {
        let transform = Transform::from_translation(Vec3::new((handle * 32) as f32, 1.0, 5.0));
        commands
            .spawn((
                game,
                Player { handle },
                SpriteSheetBundle {
                    transform,
                    texture_atlas: texture_assets.player_idle.clone(),
                    ..default()
                },
                SpriteSheetAnimation {
                    timer: Timer::new(Duration::from_millis(150), TimerMode::Repeating),
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

pub fn goto_game(mut commands: Commands, mut next_state: ResMut<NextState<State>>, session: Session<GameConfig>, local_players: LocalPlayers) {
    commands.insert_resource(session);
    commands.insert_resource(local_players);
    next_state.set(State::Game);
}
