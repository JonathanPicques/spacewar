use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_ggrs::ggrs::Config;
use bevy_matchbox::matchbox_socket::PeerId;

use crate::core::input::CoreInput;

pub const FPS: usize = 60;
pub const INPUT_DELAY: usize = 2;
pub const NUM_PLAYERS: usize = 2;
pub const MAX_PREDICTION: usize = 12;
pub const MATCHBOX_ADDRESS: &str = "ws://127.0.0.1:3536/lobby?next=2";

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
pub struct Assets {
    #[asset(key = "tileset_texture")]
    pub tileset_texture: Handle<Image>,
    #[asset(key = "tileset_project")]
    pub tileset_project: Handle<LdtkProject>,
}

#[derive(Debug)]
pub struct GameConfig;

impl Config for GameConfig {
    type Input = CoreInput;
    type State = u8;
    type Address = PeerId;
}
