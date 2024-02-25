use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_ggrs::ggrs::Config;
use bevy_matchbox::matchbox_socket::PeerId;
use clap::Parser;

use crate::core::input::CoreInput;

pub const FPS: usize = 60;
pub const INPUT_DELAY: usize = 2;
pub const MAX_PREDICTION: usize = 12;

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
    #[clap(long, short = 'l', default_value = "false")]
    pub local: bool,
    #[clap(long, short = 'n', default_value = "2")]
    pub num_players: usize,
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
