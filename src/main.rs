pub mod game;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
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
    #[asset(key = "tiles")]
    pub tiles: Handle<Image>,
    #[asset(key = "sprites", collection(typed))]
    pub sprites: Vec<Handle<TextureAtlas>>,
}

fn main() {
    let mut app = App::new();

    app.add_state::<State>()
        .add_plugins((
            DefaultPlugins,
            RonAssetPlugin::<GameDynamicAssetCollection>::new(&["ron"]),
        ))
        .add_plugins(EguiPlugin);

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

    app.add_systems(OnEnter(State::MenuMain), list_loaded_assets);

    app.run();
}

fn list_loaded_assets(texture_assets: Res<TextureAssets>) {
    println!("{:?}", texture_assets.tiles);
    println!("{:?}", texture_assets.sprites);
}
