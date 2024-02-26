use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::ggrs::Config;
use bevy_matchbox::prelude::*;
use clap::Parser;

use crate::core::input::CoreInput;

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
    #[clap(long, default_value = "2")]
    pub max_prediction: usize,
    #[clap(long, default_value = "0")]
    pub check_distance: usize,
}

#[derive(Resource, AssetCollection)]
pub struct GameAssets {
    #[asset(key = "player")]
    pub player: Handle<TextureAtlas>,

    #[asset(key = "tileset.texture")]
    pub tileset_texture: Handle<Image>,
    #[asset(key = "tileset.project")]
    pub tileset_project: Handle<LdtkProject>,
}

#[derive(Debug)]
pub struct GameConfig;

impl Config for GameConfig {
    type Input = CoreInput;
    type State = u8;
    type Address = PeerId;
}
