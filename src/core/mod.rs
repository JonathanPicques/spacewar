pub mod anim;
pub mod clock;
pub mod input;
pub mod loader;
pub mod physics;
pub mod utilities;

use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy_ggrs::ggrs::Config;
use bevy_ggrs::prelude::*;

use crate::core::anim::sprite_sheet_animator_system;
use crate::core::body::{PhysicsBody, PhysicsBodyHandle, PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::clock::{ttl_system, TimeToLive};
use crate::core::collider::{PhysicsCollider, PhysicsColliderHandle, PhysicsColliderOptions};
use crate::core::controller::PhysicsCharacterController;
use crate::core::physics::*;

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
            .checksum_resource_with_hash::<Physics>()
            .checksum_component_with_hash::<TimeToLive>()
            .checksum_component_with_hash::<PhysicsBody>()
            .checksum_component_with_hash::<PhysicsBodyHandle>()
            .checksum_component_with_hash::<PhysicsBodyOptions>()
            .checksum_component_with_hash::<PhysicsBodyVelocity>()
            .checksum_component_with_hash::<PhysicsCollider>()
            .checksum_component_with_hash::<PhysicsColliderHandle>()
            .checksum_component_with_hash::<PhysicsColliderOptions>()
            .checksum_component_with_hash::<PhysicsCharacterController>()
            //
            .rollback_resource_with_clone::<Physics>()
            .rollback_component_with_copy::<TimeToLive>()
            .rollback_component_with_copy::<PhysicsBody>()
            .rollback_component_with_copy::<PhysicsBodyHandle>()
            .rollback_component_with_copy::<PhysicsBodyOptions>()
            .rollback_component_with_copy::<PhysicsBodyVelocity>()
            .rollback_component_with_copy::<PhysicsCollider>()
            .rollback_component_with_copy::<PhysicsColliderHandle>()
            .rollback_component_with_copy::<PhysicsColliderOptions>()
            .rollback_component_with_copy::<PhysicsCharacterController>();

        self
    }
}

pub fn core_systems() -> SystemConfigs {
    (
        ttl_system,
        physics_systems(),
        sprite_sheet_animator_system,
    )
        .chain()
        .into_configs()
}
