pub mod states;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_egui::EguiPlugin;

use crate::states::game::AddGameAppExt;
use crate::states::menu::AddMenuAppExt;
use crate::states::State;

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

    app.add_loading_state(
        LoadingState::new(State::Load)
            .continue_to_state(State::Menu)
            .load_collection::<FontAssets>()
            .load_collection::<AudioAssets>()
            .load_collection::<TextureAssets>(),
    )
    .add_menu()
    .add_game();

    app.run();
}
