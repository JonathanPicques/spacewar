use bevy::prelude::*;
use rapier2d::prelude::*;

#[derive(Clone, Default, Component)]
pub struct PhysicsBody {}

#[derive(Clone, Component)]
pub struct PhysicsBodyHandle(pub(crate) RigidBodyHandle);

impl PhysicsBody {
    pub(crate) fn build(&self) -> RigidBody {
        RigidBodyBuilder::kinematic_position_based()
            .translation(vector![0.0, 0.0])
            .build()
    }
}
