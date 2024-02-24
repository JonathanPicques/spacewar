use std::collections::HashSet;
use std::hash::*;

use bevy::prelude::*;
use bevy_ecs_ldtk::assets::LdtkProject;
use bevy_ecs_ldtk::ldtk::raw_level_accessor::RawLevelAccessor;
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

pub fn find_levels_around_positions(
    positions: Vec<Vec2>,
    ldtk_projects: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
) -> HashSet<LevelIid> {
    let ldtk_project = ldtk_project_assets
        .get(ldtk_projects.single())
        .expect("LDTk project resource not found");

    positions
        .iter()
        .flat_map(|position| {
            let mut level_iids = vec![];
            for level in ldtk_project.iter_raw_levels() {
                let level_bounds = Rect {
                    min: Vec2::new(
                        level.world_x as f32,
                        level.world_y as f32,
                        //
                    ),
                    max: Vec2::new(
                        (level.world_x + level.px_wid) as f32,
                        (level.world_y + level.px_hei) as f32,
                    ),
                };

                if level_bounds.contains(*position) {
                    level_iids.push(LevelIid::new(level.iid.clone()));
                }
            }
            level_iids
        })
        .collect::<HashSet<_>>()
}
