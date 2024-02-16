pub mod game;
pub mod menu;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;

use crate::game::AddGameAppExt;
use crate::menu::menu::AddMainMenuAppExt;
use crate::menu::menu_local::AddLocalMenuAppExt;
use crate::menu::menu_online::AddOnlineMenuAppExt;

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
struct FontAssets {}

#[derive(Resource, AssetCollection)]
struct AudioAssets {}

#[derive(Resource, AssetCollection)]
struct TextureAssets {}

fn main() {
    let mut app = App::new();

    app.add_state::<State>()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin);

    app.add_game()
        .add_main_menu()
        .add_local_menu()
        .add_online_menu()
        .add_loading_state(
            LoadingState::new(State::Load)
                .continue_to_state(State::MenuMain)
                .load_collection::<FontAssets>()
                .load_collection::<AudioAssets>()
                .load_collection::<TextureAssets>(),
        );

    app.run();
}
