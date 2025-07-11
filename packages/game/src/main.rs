pub mod game;
pub mod menu;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_egui::EguiPlugin;
use bevy_ggrs::ggrs::Config;
use bevy_matchbox::matchbox_socket::PeerId;
use clap::Parser;
use rapier2d::geometry::Group;

use core::anim::SpriteSheetAnimation;
use core::input::CoreInput;
use core::loader::CoreDynamicAssetCollection;

use crate::game::AddGameAppExt;
use crate::menu::menu_local::AddLocalMenuAppExt;
use crate::menu::menu_main::AddMainMenuAppExt;
use crate::menu::menu_online::AddOnlineMenuAppExt;

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

pub enum Layer {
    All,
    None,
    //
    Wall,
    Player,
    Projectile,
}

impl From<Layer> for Group {
    fn from(value: Layer) -> Self {
        match value {
            Layer::All => Group::ALL,
            Layer::None => Group::NONE,
            //
            Layer::Wall => Group::GROUP_1,
            Layer::Player => Group::GROUP_2,
            Layer::Projectile => Group::GROUP_3,
        }
    }
}

#[derive(Parser, Resource)]
pub struct GameArgs {
    #[clap(long, default_value = "false")]
    pub local: bool,
    #[clap(long, default_value = "false")]
    pub online: bool,

    #[clap(long, default_value = "60")]
    pub fps: usize,
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
    #[clap(long, default_value = "0")]
    pub desync_detection_interval: u8,
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(key = "bullet")]
    pub bullet: Handle<Image>,
    #[asset(key = "bullet_idle")]
    pub bullet_idle_anim: Handle<SpriteSheetAnimation>,
    #[asset(key = "bullet_atlas_layout")]
    pub bullet_atlas_layout: Handle<TextureAtlasLayout>,

    #[asset(key = "grenade")]
    pub grenade: Handle<Image>,

    #[asset(key = "player")]
    pub player: Handle<Image>,
    #[asset(key = "player_idle")]
    pub player_idle: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_walk")]
    pub player_walk: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_fall")]
    pub player_fall: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_jump")]
    pub player_jump: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_hurt")]
    pub player_hurt: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_shoot")]
    pub player_shoot: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_throw")]
    pub player_throw: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_throw_end")]
    pub player_throw_end: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_dead_start")]
    pub player_dead_start: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_dead_bounce")]
    pub player_dead_bounce: Handle<SpriteSheetAnimation>,
    #[asset(key = "player_atlas_layout")]
    pub player_atlas_layout: Handle<TextureAtlasLayout>,

    #[asset(key = "background_music")]
    pub background_music: Handle<AudioSource>,
}

#[derive(Debug)]
pub struct GameConfig;

impl Config for GameConfig {
    type Input = CoreInput;
    type State = u8;
    type Address = PeerId;
}

fn main() {
    let mut app = App::new();
    let args = GameArgs::parse();
    let args_fps = args.fps;

    app.add_plugins(
        DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoVsync,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
    )
    .init_state::<State>()
    //
    .add_plugins(EguiPlugin { enable_multipass_for_primary_context: false })
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
