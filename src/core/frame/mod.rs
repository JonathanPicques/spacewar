use std::hash::{Hash, Hasher};

use bevy::prelude::*;

#[derive(Copy, Clone, Default, Resource)]
pub struct Frame(pub u64);

impl Hash for Frame {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // Not implemented to allow for stable checksums when nothing happens.
        // self.0.hash(state);
    }
}

pub fn frame_system(mut frame: ResMut<Frame>) {
    frame.0 += 1;
}
