use std::collections::HashSet;
use std::hash::*;

use bevy::prelude::*;
use bevy_ecs_ldtk::{LevelIid, LevelSet};

#[derive(Clone, Default, Resource)]
pub struct LoadedLevels {
    pub levels: HashSet<LevelIid>,
}

impl LoadedLevels {
    pub fn new(level: LevelIid) -> Self {
        Self { levels: HashSet::from([level]) }
    }
}

impl Hash for LoadedLevels {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.levels.len());
    }
}

pub fn load_levels_system(mut query: Query<&mut LevelSet>, loaded_levels: Res<LoadedLevels>) {
    if loaded_levels.is_changed() {
        query.single_mut().iids = loaded_levels.levels.clone();
    }
}
