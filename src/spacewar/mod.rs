pub mod conf;
pub mod game;
pub mod menu;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ecs_ldtk::prelude::*;
use bevy_egui::EguiPlugin;
use clap::Parser;

use crate::core::anim::SpriteSheetAnimation;
use crate::core::loader::CoreDynamicAssetCollection;
use crate::spacewar::conf::{GameArgs, GameAssets, GameConfig, State};
use crate::spacewar::game::AddGameAppExt;
use crate::spacewar::menu::menu_local::AddLocalMenuAppExt;
use crate::spacewar::menu::menu_main::AddMainMenuAppExt;
use crate::spacewar::menu::menu_online::AddOnlineMenuAppExt;

type DynamicAssetPlugin = RonAssetPlugin<CoreDynamicAssetCollection>;

pub fn spacewar() {
    let mut app = App::new();
    let args = GameArgs::parse();
    let args_fps = args.fps;

    app.add_state::<State>()
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
        .init_asset::<SpriteSheetAnimation>()
        //
        .add_plugins(LdtkPlugin)
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation { load_level_neighbors: false },
            ..default()
        })
        //
        .insert_resource(args)
        //
        .add_game(args_fps)
        .add_main_menu()
        .add_local_menu()
        .add_online_menu()
        .add_loading_state(
            LoadingState::new(State::Load)
                .continue_to_state(State::MenuMain)
                .with_dynamic_assets_file::<CoreDynamicAssetCollection>("assets.ron")
                .register_dynamic_asset_collection::<CoreDynamicAssetCollection>()
                //
                .load_collection::<GameAssets>(),
        )
        //
        .run();
}
