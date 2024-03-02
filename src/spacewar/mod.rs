pub mod game;
pub mod menu;

use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_egui::EguiPlugin;
use bevy_ggrs::ggrs::Config;
use bevy_matchbox::matchbox_socket::PeerId;
use clap::Parser;

use crate::core::anim::SpriteSheetAnimation;
use crate::core::input::CoreInput;
use crate::core::loader::CoreDynamicAssetCollection;
use crate::spacewar::game::AddGameAppExt;
use crate::spacewar::menu::menu_local::AddLocalMenuAppExt;
use crate::spacewar::menu::menu_main::AddMainMenuAppExt;
use crate::spacewar::menu::menu_online::AddOnlineMenuAppExt;

type DynamicAssetPlugin = RonAssetPlugin<CoreDynamicAssetCollection>;

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

#[derive(Parser, Resource)]
pub struct GameArgs {
    #[clap(long, default_value = "60")]
    pub fps: usize,
    #[clap(long, default_value = "false")]
    pub local: bool,
    #[clap(long, default_value = "2")]
    pub num_players: usize,
    #[clap(long, default_value = "2")]
    pub input_delay: usize,
    #[clap(long, default_value = "0")]
    pub check_distance: usize,
    #[clap(long, default_value = "2")]
    pub max_prediction: usize,
    #[clap(long, default_value = "false")]
    pub randomize_input: bool,
    #[clap(long, default_value = "ws://127.0.0.1:3536")]
    pub matchbox_address: String,
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(key = "player_texture")]
    pub player_texture: Handle<Image>,
    #[asset(key = "player_texture_atlas_layout")]
    pub player_texture_atlas_layout: Handle<TextureAtlasLayout>,

    #[asset(key = "player_anim.idle")]
    pub player_idle_anim: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_anim.walk")]
    pub player_walk_anim: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_anim.jump")]
    pub player_jump_anim: Handle<SpriteSheetAnimation>,

    #[asset(key = "bullet")]
    pub bullet: Handle<Image>,
}

#[derive(Debug)]
pub struct GameConfig;

impl Config for GameConfig {
    type Input = CoreInput;
    type State = u8;
    type Address = PeerId;
}

pub fn spacewar() {
    let mut app = App::new();
    let args = GameArgs::parse();
    let args_fps = args.fps;

    app.init_state::<State>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window { prevent_default_event_handling: false, ..default() }),
                    ..default()
                }),
        )
        .insert_resource(Msaa::Off)
        //
        .add_plugins(EguiPlugin)
        .add_plugins(DynamicAssetPlugin::new(&["ron"]))
        .init_asset::<SpriteSheetAnimation>()
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
