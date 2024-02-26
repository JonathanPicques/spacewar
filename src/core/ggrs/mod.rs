use bevy::prelude::*;
use bevy_ggrs::ggrs::Config;
use bevy_ggrs::prelude::*;

use crate::core::levels::LoadedLevels;
use crate::core::physics::PlayerController;
use crate::core::utilities::hash::transform_hasher;

pub trait AddGgrsCoreAppExt {
    fn add_ggrs<T, M>(&mut self, fps: usize, input_system: impl IntoSystemConfigs<M>) -> &mut Self
    where
        T: Config;
}

impl AddGgrsCoreAppExt for App {
    fn add_ggrs<T, M>(&mut self, fps: usize, input_system: impl IntoSystemConfigs<M>) -> &mut Self
    where
        T: Config,
    {
        self.add_plugins(GgrsPlugin::<T>::default())
            .add_systems(ReadInputs, input_system)
            //
            .set_rollback_schedule_fps(fps)
            //
            .checksum_resource_with_hash::<LoadedLevels>()
            //
            .checksum_component::<Transform>(transform_hasher)
            .checksum_component_with_hash::<PlayerController>()
            //
            .rollback_component_with_copy::<PlayerController>()
            .rollback_component_with_clone::<Transform>()
    }
}
