use bevy::prelude::*;
use rapier2d::prelude::*;

use crate::core::utilities::maths::*;
use crate::core::Physics;

#[derive(Clone, Component)]
pub enum PhysicsBody {
    Fixed,
    Dynamic,
    KinematicPositionBased,
    KinematicVelocityBased,
}

#[derive(Clone, Component)]
pub struct PhysicsBodyOptions {
    pub gravity_scale: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub additional_mass: f32,
}

#[derive(Clone, Component)]
pub(crate) struct PhysicsBodyHandle(pub(crate) RigidBodyHandle);

impl PhysicsBody {
    pub(crate) fn build(&self, physics: &Physics, transform: &Transform) -> RigidBody {
        let rotation = transform.rotation.to_euler(EulerRot::ZYX).0;
        let translation = (transform.translation / physics.scale).to_physics();

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

    pub(crate) fn apply_options(&self, body: &mut RigidBody, options: &PhysicsBodyOptions, wake_up: bool) {
        body.set_gravity_scale(options.gravity_scale, wake_up);
        body.set_linear_damping(options.linear_damping);
        body.set_angular_damping(options.angular_damping);
        body.set_additional_mass(options.additional_mass, wake_up);
    }
}

impl Default for PhysicsBodyOptions {
    fn default() -> Self {
        Self {
            gravity_scale: 1.0,
            linear_damping: default(),
            angular_damping: default(),
            additional_mass: default(),
        }
    }
}
