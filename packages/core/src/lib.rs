pub mod anim;
pub mod clock;
pub mod event;
pub mod input;
pub mod loader;
pub mod physics;
pub mod utilities;

pub mod derive {
    pub use core_derive::RollbackEvent;
}

use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;
use bevy_ggrs::ggrs::Config;
use bevy_ggrs::prelude::*;

use crate::anim::{sprite_sheet_animator_system, SpriteSheetAnimator};
use crate::body::{PhysicsBody, PhysicsBodyHandle, PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::clock::{ttl_system, TimeToLive};
use crate::collider::{PhysicsCollider, PhysicsColliderHandle, PhysicsColliderOptions};
use crate::controller::PhysicsCharacterController;
use crate::physics::*;

pub trait AddCoreAppExt {
    fn add_core<T, M>(&mut self, fps: usize, input_system: impl IntoScheduleConfigs<ScheduleSystem, M>) -> &mut Self
    where
        T: Config;
}

impl AddCoreAppExt for App {
    fn add_core<T, M>(&mut self, fps: usize, input_system: impl IntoScheduleConfigs<ScheduleSystem, M>) -> &mut Self
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
            .rollback_resource_with_copy::<Scaler>()
            .rollback_resource_with_clone::<Physics>()
            .rollback_component_with_copy::<TimeToLive>()
            .rollback_component_with_copy::<PhysicsBody>()
            .rollback_component_with_copy::<PhysicsBodyHandle>()
            .rollback_component_with_copy::<PhysicsBodyOptions>()
            .rollback_component_with_copy::<PhysicsBodyVelocity>()
            .rollback_component_with_copy::<PhysicsCollider>()
            .rollback_component_with_copy::<PhysicsColliderHandle>()
            .rollback_component_with_copy::<PhysicsColliderOptions>()
            .rollback_component_with_copy::<PhysicsCharacterController>()
            .rollback_component_with_clone::<Sprite>()
            .rollback_component_with_clone::<SpriteSheetAnimator>();

        self
    }
}

pub fn core_systems() -> ScheduleConfigs<ScheduleSystem> {
    (
        ttl_system,
        physics_systems(),
        sprite_sheet_animator_system,
    )
        .chain()
        .into_configs()
}
