use bevy::prelude::*;
use rapier2d::math::Real;
use rapier2d::prelude::*;

use crate::core::physics::body::PhysicsBody;
use crate::core::physics::collider::PhysicsCollider;

#[derive(Clone, Resource)]
pub struct PhysicsContext {
    pub gravity: Vector<Real>,
    //
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub ccd_solver: CCDSolver,
    pub broad_phase: BroadPhase,
    pub narrow_phase: NarrowPhase,
    pub query_pipeline: QueryPipeline,
    pub island_manager: IslandManager,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub integration_parameters: IntegrationParameters,
}

impl Default for PhysicsContext {
    fn default() -> Self {
        Self {
            gravity: vector![0.0, -9.81],
            integration_parameters: default(),
            //
            bodies: default(),
            colliders: default(),
            ccd_solver: default(),
            broad_phase: default(),
            narrow_phase: default(),
            query_pipeline: default(),
            island_manager: default(),
            impulse_joints: default(),
            multibody_joints: default(),
        }
    }
}

impl PhysicsContext {
    pub(crate) fn step(&mut self) {
        let event_handler = ();
        let physics_hooks = ();

        PhysicsPipeline::new().step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &physics_hooks,
            &event_handler,
        );
    }

    pub(crate) fn insert_body(&mut self, body: &PhysicsBody, collider: &PhysicsCollider) -> (RigidBodyHandle, ColliderHandle) {
        let body_handle = self.bodies.insert(body.build());
        let collider_handle = self
            .colliders
            .insert_with_parent(collider.build(), body_handle, &mut self.bodies);

        (body_handle, collider_handle)
    }
}
