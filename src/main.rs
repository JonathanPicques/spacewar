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

    app
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
        .add_plugins(EguiPlugin)
        .add_plugins(DynamicAssetPlugin::new(&["ron"]))
        //
        .insert_resource(Msaa::Off)
        //
        .add_game()
        .run();
}
