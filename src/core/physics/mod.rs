pub mod body;
pub mod collider;
pub mod controller;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackOrdered};
use rapier2d::math::Real;
use rapier2d::prelude::*;

use crate::core::body::{PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::core::collider::PhysicsColliderOptions;
use crate::core::physics::body::PhysicsBody;
use crate::core::physics::body::PhysicsBodyHandle;
use crate::core::physics::collider::PhysicsCollider;
use crate::core::physics::collider::PhysicsColliderHandle;
use crate::core::physics::controller::PhysicsCharacterController;
use crate::core::utilities::cmp::cmp_rollack;
use crate::core::utilities::hash::f32_hasher;
use crate::core::utilities::maths::*;

#[derive(Clone, Resource)]
pub struct Physics {
    pub scale: f32,
    pub gravity: Vector<Real>,
    //
    pub(crate) bodies: RigidBodySet,
    pub(crate) colliders: ColliderSet,
    pub(crate) ccd_solver: CCDSolver,
    pub(crate) broad_phase: BroadPhase,
    pub(crate) narrow_phase: NarrowPhase,
    pub(crate) query_pipeline: QueryPipeline,
    pub(crate) island_manager: IslandManager,
    pub(crate) impulse_joints: ImpulseJointSet,
    pub(crate) multibody_joints: MultibodyJointSet,
    pub(crate) integration_parameters: IntegrationParameters,
    //
    pub(crate) body_handles_by_entity: HashMap<Entity, RigidBodyHandle>,
    pub(crate) collider_handles_by_entity: HashMap<Entity, ColliderHandle>,
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

    //

    pub(crate) fn insert_body(&mut self, body: RigidBody, collider: Collider) -> (RigidBodyHandle, ColliderHandle) {
        let body_handle = self.bodies.insert(body);
        let collider_handle = self
            .colliders
            .insert_with_parent(collider, body_handle, &mut self.bodies);

        (body_handle, collider_handle)
    }

    pub(crate) fn remove_body(&mut self, body_handle: RigidBodyHandle) -> Option<RigidBody> {
        self.bodies.remove(
            body_handle,
            &mut self.island_manager,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            false,
        )
    }

    pub(crate) fn remove_collider(&mut self, collider_handle: ColliderHandle) -> Option<Collider> {
        self.colliders.remove(
            collider_handle,
            &mut self.island_manager,
            &mut self.bodies,
            false,
        )
    }

    //

    pub(crate) fn move_controller(
        &mut self,
        body_handle: &PhysicsBodyHandle,
        collider_handle: &PhysicsColliderHandle,
        collider_options: Option<&PhysicsColliderOptions>,
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
            let controller = character_controller.rapier_controller;
            let collider_shape = collider.shape();
            let mut collisions = vec![];
            let mut query_filter = QueryFilter::default().exclude_rigid_body(body_handle.0);

            if let Some(collider_options) = collider_options {
                query_filter = query_filter.groups(collider_options.collision_groups);
            }
            let movement = controller.move_shape(
                self.integration_parameters.dt,
                &self.bodies,
                &self.colliders,
                &self.query_pipeline,
                collider_shape,
                position,
                (character_controller.velocity / self.scale).to_physics(),
                query_filter,
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

impl Hash for Physics {
    fn hash<H: std::hash::Hasher>(&self, mut state: &mut H) {
        for (_, body) in self.bodies.iter() {
            let rotation = body.rotation().angle();
            let translation = body.translation();

            f32_hasher(rotation, &mut state);
            f32_hasher(translation.x, &mut state);
            f32_hasher(translation.y, &mut state);
        }
        for (_, collider) in self.colliders.iter() {
            let rotation = collider.rotation().angle();
            let translation = collider.translation();

            f32_hasher(rotation, &mut state);
            f32_hasher(translation.x, &mut state);
            f32_hasher(translation.y, &mut state);
        }
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            scale: 9.0,
            gravity: vector![0.0, -9.81],
            //
            body_handles_by_entity: default(),
            collider_handles_by_entity: default(),
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
            integration_parameters: default(),
        }
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
        Option<&PhysicsColliderOptions>,
        &mut PhysicsCharacterController,
    )>,
    //
    order: Res<RollbackOrdered>,
    mut physics: ResMut<Physics>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, body_handle, collider_handle, collider_options, mut character_controller) in query {
        physics.move_controller(
            body_handle,
            collider_handle,
            collider_options,
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
    velocity_query: Query<
        (
            &Rollback,
            &PhysicsBody,
            &PhysicsBodyHandle,
            &PhysicsBodyVelocity,
        ),
        Upserted<PhysicsBodyVelocity>,
    >,
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
        let body = body.build(&physics, transform);
        let collider = collider.build();
        let (body_handle, collider_handle) = physics.insert_body(body, collider);

        physics
            .body_handles_by_entity
            .insert(e, body_handle);
        physics
            .collider_handles_by_entity
            .insert(e, collider_handle);
        commands.entity(e).insert((
            PhysicsBodyHandle(body_handle),
            PhysicsColliderHandle(collider_handle),
        ));
    }
}

#[allow(clippy::type_complexity)]
fn physics_remove_handles_system(
    query_body_handles: Query<&PhysicsBodyHandle>,
    query_collider_handles: Query<&PhysicsColliderHandle>,
    //
    mut physics: ResMut<Physics>,
) {
    let body_handles = query_body_handles
        .iter()
        .map(|b| b.0)
        .collect::<HashSet<_>>();
    let collider_handles = query_collider_handles
        .iter()
        .map(|c| c.0)
        .collect::<HashSet<_>>();
    let mut remove_body_handles = vec![];
    let mut remove_collider_handles = vec![];

    for (handle, _) in physics.bodies.iter() {
        if !body_handles.contains(&handle) {
            remove_body_handles.push(handle);
        }
    }
    for (handle, _) in physics.colliders.iter() {
        if !collider_handles.contains(&handle) {
            remove_collider_handles.push(handle);
        }
    }

    for handle in remove_body_handles {
        physics
            .remove_body(handle)
            .expect("Body not removed");
    }
    for handle in remove_collider_handles {
        physics
            .remove_collider(handle)
            .expect("Collider not removed");
    }
}

//

#[allow(clippy::type_complexity)]
fn physics_debug_system(mut gizmos: Gizmos, physics: Res<Physics>) {
    for (_, collider) in physics.colliders.iter() {
        if let Some(ball) = collider.shape().as_ball() {
            gizmos.circle_2d(
                collider.translation().to_bevy() * physics.scale,
                ball.radius,
                Color::GREEN,
            );
        }
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
        physics_remove_handles_system,
        physics_update_system,
        physics_sync_system,
        physics_system,
    )
        .chain()
        .into_configs()
}

pub fn physics_debug_systems() -> SystemConfigs {
    physics_debug_system.into_configs()
}
