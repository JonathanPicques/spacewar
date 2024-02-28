use bevy::prelude::*;
use rapier2d::math::Real;
use rapier2d::prelude::*;

use crate::core::body::PhysicsBodyHandle;
use crate::core::collider::PhysicsColliderHandle;
use crate::core::physics::body::PhysicsBody;
use crate::core::physics::collider::PhysicsCollider;
use crate::core::PhysicsCharacterController;

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
    #[allow(clippy::let_unit_value)]
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

    pub(crate) fn insert_body(&mut self, body: &PhysicsBody, collider: &PhysicsCollider, transform: &Transform) -> (RigidBodyHandle, ColliderHandle) {
        if self.bodies.is_empty() {
            let width = 250.0;
            let height = 10.0;
            let floor_body = RigidBodyBuilder::fixed().translation(vector![0.0, -40.0]);
            let floor_handle = self.bodies.insert(floor_body);
            let floor_collider = ColliderBuilder::cuboid(width, height);

            self.colliders
                .insert_with_parent(floor_collider, floor_handle, &mut self.bodies);

            let width = 50.0;
            let height = 50.0;
            let box_body = RigidBodyBuilder::fixed().translation(vector![150.0, -10.0]);
            let box_handle = self.bodies.insert(box_body);
            let box_collider = ColliderBuilder::cuboid(width, height);

            self.colliders
                .insert_with_parent(box_collider, box_handle, &mut self.bodies);
        }

        let body_handle = self.bodies.insert(body.build(transform));
        let collider_handle = self
            .colliders
            .insert_with_parent(collider.build(), body_handle, &mut self.bodies);

        (body_handle, collider_handle)
    }

    pub(crate) fn move_controller(
        &mut self,
        body_handle: &PhysicsBodyHandle,
        collider_handle: &PhysicsColliderHandle,
        physics_controller: &mut PhysicsCharacterController,
    ) {
        let (movement, collisions) = {
            let body = self
                .bodies
                .get(body_handle.0)
                .expect("Body not found");
            let collider = self
                .colliders
                .get(collider_handle.0)
                .expect("Collider not found");
            let position = body.position();
            let collider_shape = collider.shape();
            let rapier_controller = physics_controller.rapier_controller;

            let mut collisions = vec![];
            let movement = rapier_controller.move_shape(
                self.integration_parameters.dt,
                &self.bodies,
                &self.colliders,
                &self.query_pipeline,
                collider_shape,
                position,
                vector![
                    physics_controller.velocity.x,
                    physics_controller.velocity.y
                ],
                QueryFilter::default().exclude_rigid_body(body_handle.0),
                |collision| {
                    collisions.push(collision);
                },
            );

            (movement, collisions)
        };

        let body = self.bodies.get_mut(body_handle.0).unwrap();
        let position = body.position();

        body.set_next_kinematic_translation(position.translation.vector + movement.translation);
        physics_controller.on_floor = movement.grounded;
        physics_controller.collisions = collisions;
    }
}
