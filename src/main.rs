pub mod core;
pub mod game;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::core::loader::CoreDynamicAssetCollection;
use crate::game::AddGameAppExt;

type DynamicAssetPlugin = RonAssetPlugin<CoreDynamicAssetCollection>;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(DynamicAssetPlugin::new(&["ron"]))
        .add_game();

    app.run();
}
