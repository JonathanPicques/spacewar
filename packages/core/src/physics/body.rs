use bevy::prelude::*;
use derivative::Derivative;
use rapier2d::prelude::*;

use crate::utilities::maths::*;
use crate::Scaler;

#[derive(Hash, Copy, Clone, Component)]
pub enum PhysicsBody {
    Fixed,
    Dynamic,
    KinematicPositionBased,
    KinematicVelocityBased,
}

#[derive(Clone, Copy, Component, Derivative)]
#[derivative(Hash)]
pub struct PhysicsBodyOptions {
    pub ccd: bool,
    pub sleep: Option<bool>,
    #[derivative(Hash = "ignore")]
    pub gravity_scale: f32,
    #[derivative(Hash = "ignore")]
    pub linear_damping: f32,
    #[derivative(Hash = "ignore")]
    pub angular_damping: f32,
    #[derivative(Hash = "ignore")]
    pub additional_mass: f32,
}

#[derive(Copy, Clone, Component, Derivative)]
#[derivative(Hash)]
pub struct PhysicsBodyVelocity {
    #[derivative(Hash = "ignore")]
    pub linear_velocity: Option<Vec2>,
    #[derivative(Hash = "ignore")]
    pub angular_velocity: Option<f32>,
}

#[derive(Hash, Copy, Clone, Component)]
pub struct PhysicsBodyHandle(pub(crate) RigidBodyHandle);

impl PhysicsBody {
    pub(crate) fn build(&self, scaler: &Scaler, transform: &Transform) -> RigidBody {
        let rotation = transform.rotation.to_euler(EulerRot::ZYX).0;
        let translation = scaler
            .pixels_to_meters(transform.translation)
            .to_physics();

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

    pub(crate) fn apply_options(&self, scaler: &Scaler, body: &mut RigidBody, options: &PhysicsBodyOptions) {
        let wake_up = true;

        match options.sleep {
            None => (),
            Some(true) => body.sleep(),
            Some(false) => body.wake_up(false),
        }
        body.enable_ccd(options.ccd);
        body.set_gravity_scale(options.gravity_scale, wake_up);
        body.set_linear_damping(scaler.pixels_to_meters(options.linear_damping));
        body.set_angular_damping(options.angular_damping);
        body.set_additional_mass(
            scaler.pixels_to_meters(options.additional_mass),
            wake_up,
        );
    }

    pub(crate) fn apply_velocity(&self, scaler: &Scaler, body: &mut RigidBody, velocity: &PhysicsBodyVelocity) {
        let wake_up = true;

        if let Some(linvel) = velocity.linear_velocity {
            body.set_linvel(
                scaler.pixels_to_meters(linvel).to_physics(),
                wake_up,
            );
        }
        if let Some(angvel) = velocity.angular_velocity {
            body.set_angvel(angvel, wake_up);
        }
    }
}

impl PhysicsBodyHandle {
    #[inline(always)]
    pub fn handle(&self) -> RigidBodyHandle {
        self.0
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
            sleep: default(),
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
