pub mod body;
pub mod collider;
pub mod controller;

use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackOrdered};
use rapier2d::math::Real;
use rapier2d::prelude::*;

use crate::core::physics::body::PhysicsBodyHandle;
use crate::core::physics::collider::PhysicsColliderHandle;
use crate::core::utilities::sorting::cmp_rollack;

pub use crate::core::physics::body::PhysicsBody;
pub use crate::core::physics::collider::PhysicsCollider;
pub use crate::core::physics::controller::PhysicsCharacterController;

#[derive(Clone, Resource)]
pub struct Physics {
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

impl Default for Physics {
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

impl Physics {
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
            let floor_body = RigidBodyBuilder::fixed().translation(vector![0.0, -30.0]);
            let floor_handle = self.bodies.insert(floor_body);
            let floor_collider = ColliderBuilder::cuboid(width, height);

            self.colliders
                .insert_with_parent(floor_collider, floor_handle, &mut self.bodies);

            let width = 50.0;
            let height = 50.0;
            let box_body = RigidBodyBuilder::fixed().translation(vector![150.0, 10.0]);
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

        let body = self
            .bodies
            .get_mut(body_handle.0)
            .expect("Body not found");
        let position = body.position();

        body.set_next_kinematic_translation(position.translation.vector + movement.translation);
        physics_controller.on_floor = movement.grounded;
        physics_controller.collisions = collisions;
    }
}

pub fn physics_system(
    mut query: Query<(
        &Rollback,
        &PhysicsBodyHandle,
        &PhysicsColliderHandle,
        &mut PhysicsCharacterController,
    )>,
    //
    order: Res<RollbackOrdered>,
    mut physics: ResMut<Physics>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, body_handle, collider_handle, mut controller) in query {
        physics.move_controller(body_handle, collider_handle, &mut controller);
    }
    physics.step();
}

pub fn physics_sync_system(
    mut query: Query<(&Rollback, &PhysicsBodyHandle, &mut Transform)>,
    //
    order: Res<RollbackOrdered>,
    physics: Res<Physics>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, body, mut transform) in query {
        if let Some(body) = physics.bodies.get(body.0) {
            transform.translation.x = body.position().translation.x;
            transform.translation.y = body.position().translation.y;
        }
    }
}

pub fn physics_debug_system(mut gizmos: Gizmos, physics: Res<Physics>) {
    for (_, collider) in physics.colliders.iter() {
        if let Some(cuboid) = collider.shape().as_cuboid() {
            gizmos.rect_2d(
                Vec2::new(collider.translation().x, collider.translation().y),
                collider.rotation().angle(),
                Vec2::new(
                    cuboid.half_extents.x * 2.0,
                    cuboid.half_extents.y * 2.0,
                ),
                Color::GREEN,
            );
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn physics_create_handles_system(
    query: Query<
        (
            Entity,
            &Rollback,
            &Transform,
            &PhysicsBody,
            &PhysicsCollider,
        ),
        (
            Without<PhysicsBodyHandle>,
            Without<PhysicsColliderHandle>,
        ),
    >,
    mut commands: Commands,
    //
    order: Res<RollbackOrdered>,
    mut physics: ResMut<Physics>,
) {
    let mut query = query.iter().collect::<Vec<_>>();
    query.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (e, _, transform, body, collider) in query {
        let (body_handle, collider_handle) = physics.insert_body(body, collider, transform);

        commands.entity(e).insert((
            PhysicsBodyHandle(body_handle),
            PhysicsColliderHandle(collider_handle),
        ));
    }
}
