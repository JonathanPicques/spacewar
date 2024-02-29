use bevy::prelude::*;
use rapier2d::prelude::*;

#[derive(Clone, Component)]
pub enum PhysicsBody {
    Fixed,
    Dynamic,
    KinematicPositionBased,
    KinematicVelocityBased,
}

#[derive(Clone, Component)]
pub struct PhysicsBodyHandle(pub(crate) RigidBodyHandle);

impl PhysicsBody {
    pub(crate) fn build(&self, transform: &Transform) -> RigidBody {
        let rotation = transform.rotation.to_euler(EulerRot::ZYX).0;
        let translation = vector![transform.translation.x, transform.translation.y];
        match self {
            PhysicsBody::Fixed => RigidBodyBuilder::fixed()
                .rotation(rotation)
                .translation(translation)
                .build(),
            PhysicsBody::Dynamic => RigidBodyBuilder::dynamic()
                .rotation(rotation)
                .translation(translation)
                .build(),
            PhysicsBody::KinematicPositionBased => RigidBodyBuilder::kinematic_position_based()
                .rotation(rotation)
                .translation(translation)
                .build(),
            PhysicsBody::KinematicVelocityBased => RigidBodyBuilder::kinematic_velocity_based()
                .rotation(rotation)
                .translation(translation)
                .build(),
        }
    }
}
