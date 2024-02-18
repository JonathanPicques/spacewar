pub mod game;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_egui::EguiPlugin;

use crate::game::core::loader::GameDynamicAssetCollection;
use crate::game::menu::menu::AddMainMenuAppExt;
use crate::game::menu::menu_local::AddLocalMenuAppExt;
use crate::game::menu::menu_online::AddOnlineMenuAppExt;
use crate::game::AddGameAppExt;

#[derive(Eq, Hash, Clone, Debug, States, Default, PartialEq)]
pub enum State {
    #[default]
    Load,
    //
    MenuMain,
    MenuLocal,
    MenuOnline,
    //
    Game,
}

#[derive(Resource, AssetCollection)]
pub struct FontAssets {}

#[derive(Resource, AssetCollection)]
pub struct AudioAssets {}

#[derive(Resource, AssetCollection)]
pub struct TextureAssets {
    #[asset(key = "tileset_texture")]
    pub tileset_texture: Handle<Image>,
    #[asset(key = "tileset_project")]
    pub tileset_project: Handle<LdtkProject>,
}

type DynamicAssetPlugin = RonAssetPlugin<GameDynamicAssetCollection>;

fn main() {
    let mut app = App::new();

    app.add_state::<State>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(DynamicAssetPlugin::new(&["ron"]));

    app.add_game()
        .add_main_menu()
        .add_local_menu()
        .add_online_menu()
        .add_loading_state(
            LoadingState::new(State::Load)
                .continue_to_state(State::MenuMain)
                .with_dynamic_assets_file::<GameDynamicAssetCollection>("assets.ron")
                .register_dynamic_asset_collection::<GameDynamicAssetCollection>()
                //
                .load_collection::<FontAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );

    app.run();
}
