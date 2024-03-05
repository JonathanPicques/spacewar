use bevy::prelude::*;
use rapier2d::prelude::*;

use crate::core::utilities::maths::*;
use crate::core::Physics;

#[derive(Clone, Component, Debug)]
pub enum PhysicsBody {
    Fixed,
    Dynamic,
    KinematicPositionBased,
    KinematicVelocityBased,
}

#[derive(Clone, Component, Debug)]
pub struct PhysicsBodyOptions {
    pub ccd: bool,
    pub gravity_scale: f32,
    pub linear_damping: f32,
    pub angular_damping: f32,
    pub additional_mass: f32,
}

#[derive(Clone, Component, Debug)]
pub struct PhysicsBodyVelocity {
    pub linear_velocity: Option<Vec2>,
    pub angular_velocity: Option<f32>,
}

#[derive(Clone, Component, Debug)]
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
        body.enable_ccd(options.ccd);
        body.set_gravity_scale(options.gravity_scale, wake_up);
        body.set_linear_damping(options.linear_damping);
        body.set_angular_damping(options.angular_damping);
        body.set_additional_mass(options.additional_mass, wake_up);
    }

    pub(crate) fn apply_velocity(&self, body: &mut RigidBody, velocity: &PhysicsBodyVelocity, scale: f32, wake_up: bool) {
        if let Some(linvel) = velocity.linear_velocity {
            body.set_linvel((linvel / scale).to_physics(), wake_up);
        }
        if let Some(angvel) = velocity.angular_velocity {
            body.set_angvel(angvel, wake_up);
        }
    }
}

impl PhysicsBodyOptions {
    pub fn from_gravity_scale(gravity_scale: f32) -> Self {
        Self { gravity_scale, ..default() }
    }
}

impl PhysicsBodyVelocity {
    pub fn from_linear_velocity(linear_velocity: Vec2) -> Self {
        Self {
            linear_velocity: Some(linear_velocity),
            ..default()
        }
    }

    pub fn from_angular_velocity(angular_velocity: f32) -> Self {
        Self {
            angular_velocity: Some(angular_velocity),
            ..default()
        }
    }
}

impl Default for PhysicsBodyOptions {
    fn default() -> Self {
        Self {
            ccd: false,
            gravity_scale: 1.0,
            linear_damping: default(),
            angular_damping: default(),
            additional_mass: default(),
        }
    }
}

impl Default for PhysicsBodyVelocity {
    fn default() -> Self {
        Self {
            linear_velocity: Some(Vec2::ZERO),
            angular_velocity: Some(0.0),
        }
    }
}
