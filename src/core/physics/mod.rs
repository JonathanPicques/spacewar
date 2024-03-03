pub mod body;
pub mod collider;
pub mod controller;

use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackOrdered};
use rapier2d::math::Real;
use rapier2d::prelude::*;

use crate::core::body::{PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::collider::PhysicsColliderOptions;
use crate::core::physics::body::PhysicsBodyHandle;
use crate::core::physics::collider::PhysicsColliderHandle;
use crate::core::utilities::cmp::cmp_rollack;
use crate::core::utilities::maths::*;

pub use crate::core::physics::body::PhysicsBody;
pub use crate::core::physics::collider::PhysicsCollider;
pub use crate::core::physics::controller::PhysicsCharacterController;

#[derive(Clone, Resource)]
pub struct Physics {
    pub scale: f32,
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
            scale: 9.0,
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
        let body_handle = self.bodies.insert(body.build(self, transform));
        let collider_handle = self
            .colliders
            .insert_with_parent(collider.build(), body_handle, &mut self.bodies);

        (body_handle, collider_handle)
    }

    pub(crate) fn move_controller(
        &mut self,
        body_handle: &PhysicsBodyHandle,
        collider_handle: &PhysicsColliderHandle,
        character_controller: &mut PhysicsCharacterController,
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
            let rapier_controller = character_controller.rapier_controller;

            let mut collisions = vec![];
            let movement = rapier_controller.move_shape(
                self.integration_parameters.dt,
                &self.bodies,
                &self.colliders,
                &self.query_pipeline,
                collider_shape,
                position,
                (character_controller.velocity / self.scale).to_physics(),
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
        character_controller.update_with_movement(movement, collisions);
    }
}

//

type Upserted<T> = Or<(Added<T>, Changed<T>)>;

#[allow(clippy::type_complexity)]
fn physics_system(
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

    for (_, body_handle, collider_handle, mut character_controller) in query {
        physics.move_controller(
            body_handle,
            collider_handle,
            &mut character_controller,
        );
    }
    physics.step();
}

#[allow(clippy::type_complexity)]
fn physics_sync_system(
    mut query: Query<(&Rollback, &PhysicsBodyHandle, &mut Transform)>,
    //
    order: Res<RollbackOrdered>,
    physics: Res<Physics>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, body, mut transform) in query {
        if let Some(body) = physics.bodies.get(body.0) {
            transform.translation = (body.position().to_bevy() * physics.scale).extend(transform.translation.z);
        }
    }
}

#[allow(clippy::type_complexity)]
fn physics_update_system(
    body_query: Query<
        (
            &Rollback,
            &PhysicsBody,
            &PhysicsBodyHandle,
            &PhysicsBodyOptions,
        ),
        Upserted<PhysicsBodyOptions>,
    >,
    collider_query: Query<
        (
            &Rollback,
            &PhysicsCollider,
            &PhysicsColliderHandle,
            &PhysicsColliderOptions,
        ),
        Upserted<PhysicsColliderOptions>,
    >,
    velocity_query: Query<(
        &Rollback,
        &PhysicsBody,
        &PhysicsBodyHandle,
        &PhysicsBodyVelocity,
    )>,
    //
    order: Res<RollbackOrdered>,
    mut physics: ResMut<Physics>,
) {
    let scale = physics.scale;
    let mut body_query = body_query.iter().collect::<Vec<_>>();
    let mut collider_query = collider_query.iter().collect::<Vec<_>>();
    let mut velocity_query = velocity_query.iter().collect::<Vec<_>>();

    body_query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));
    collider_query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));
    velocity_query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, body, body_handle, body_options) in body_query {
        body.apply_options(
            physics
                .bodies
                .get_mut(body_handle.0)
                .expect("Body not found"),
            body_options,
            true,
        );
    }
    for (_, body, body_handle, body_velocity) in velocity_query {
        body.apply_velocity(
            physics
                .bodies
                .get_mut(body_handle.0)
                .expect("Body not found"),
            body_velocity,
            scale,
            true,
        );
    }
    for (_, collider, collider_handle, collider_options) in collider_query {
        collider.apply_options(
            physics
                .colliders
                .get_mut(collider_handle.0)
                .expect("Collider not found"),
            collider_options,
        );
    }
}

#[allow(clippy::type_complexity)]
fn physics_create_handles_system(
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

//

#[allow(clippy::type_complexity)]
fn physics_debug_system(mut gizmos: Gizmos, physics: Res<Physics>) {
    for (_, collider) in physics.colliders.iter() {
        if let Some(cuboid) = collider.shape().as_cuboid() {
            gizmos.rect_2d(
                collider.translation().to_bevy() * physics.scale,
                collider.rotation().angle(),
                cuboid.half_extents.to_bevy() * 2.0 * physics.scale,
                Color::GREEN,
            );
        }
    }
}

//

pub fn physics_systems() -> SystemConfigs {
    (
        physics_create_handles_system,
        physics_update_system.after(physics_create_handles_system),
        physics_sync_system.after(physics_update_system),
        physics_system.after(physics_sync_system),
    )
        .into_configs()
}

pub fn physics_debug_systems() -> SystemConfigs {
    physics_debug_system.into_configs()
}
