use bevy_ggrs::ggrs::Config;
use bevy_matchbox::prelude::*;

use crate::game::core::input::CoreInput;

pub mod input;
pub mod loader;

#[derive(Debug)]
pub struct CoreConfig;
impl Config for CoreConfig {
    type Input = CoreInput;
    type State = u8;
    type Address = PeerId;
}
