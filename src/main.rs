use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_egui::EguiPlugin;

use crate::core::loader::CoreDynamicAssetCollection;
use crate::game::AddGameAppExt;

pub mod core;
pub mod game;

type DynamicAssetPlugin = RonAssetPlugin<CoreDynamicAssetCollection>;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(DynamicAssetPlugin::new(&["ron"]))
        .add_game()
        .run();
}
