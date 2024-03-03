pub mod anim;
pub mod clock;
pub mod input;
pub mod loader;
pub mod physics;
pub mod utilities;

use bevy::prelude::*;
use bevy_ggrs::ggrs::Config;
use bevy_ggrs::prelude::*;

use crate::core::body::{PhysicsBodyHandle, PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::clock::TimeToLive;
use crate::core::collider::{PhysicsColliderHandle, PhysicsColliderOptions};
use crate::core::physics::*;
use crate::core::utilities::hash::{physics_hasher, transform_hasher};

pub trait AddCoreAppExt {
    fn add_core<T, M>(&mut self, fps: usize, input_system: impl IntoSystemConfigs<M>) -> &mut Self
    where
        T: Config;
}

impl AddCoreAppExt for App {
    fn add_core<T, M>(&mut self, fps: usize, input_system: impl IntoSystemConfigs<M>) -> &mut Self
    where
        T: Config,
    {
        self.add_plugins(GgrsPlugin::<T>::default())
            .add_systems(ReadInputs, input_system)
            .set_rollback_schedule_fps(fps)
            //
            .checksum_resource::<Physics>(physics_hasher)
            .checksum_component::<Transform>(transform_hasher)
            .checksum_component_with_hash::<TimeToLive>()
            //
            .rollback_resource_with_clone::<Physics>()
            .rollback_component_with_clone::<Transform>()
            .rollback_component_with_clone::<TimeToLive>()
            .rollback_component_with_clone::<PhysicsBody>()
            .rollback_component_with_clone::<PhysicsBodyHandle>()
            .rollback_component_with_clone::<PhysicsBodyOptions>()
            .rollback_component_with_clone::<PhysicsBodyVelocity>()
            .rollback_component_with_clone::<PhysicsCollider>()
            .rollback_component_with_clone::<PhysicsColliderHandle>()
            .rollback_component_with_clone::<PhysicsColliderOptions>()
            .rollback_component_with_clone::<PhysicsCharacterController>();

        self
    }
}
