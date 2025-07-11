pub mod body;
pub mod collider;
pub mod controller;

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackOrdered};
use rapier2d::math::Real;
use rapier2d::{crossbeam, prelude::*};

use crate::body::{PhysicsBodyOptions, PhysicsBodyVelocity};
use crate::collider::PhysicsColliderOptions;
use crate::physics::body::PhysicsBody;
use crate::physics::body::PhysicsBodyHandle;
use crate::physics::collider::PhysicsCollider;
use crate::physics::collider::PhysicsColliderHandle;
use crate::physics::controller::PhysicsCharacterController;
use crate::utilities::cmp::cmp_rollback;
use crate::utilities::hash::f32_hasher;
use crate::utilities::maths::*;

#[derive(Copy, Clone, Resource)]
pub struct Scaler {
    pub scale: f32,
}

#[derive(Clone, Resource)]
pub struct Physics {
    pub gravity: Vector<Real>,
    //
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    pub ccd_solver: CCDSolver,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub query_pipeline: QueryPipeline,
    pub island_manager: IslandManager,
    pub impulse_joints: ImpulseJointSet,
    pub multibody_joints: MultibodyJointSet,
    pub integration_parameters: IntegrationParameters,
    //
    pub body_handles_by_entity: HashMap<Entity, RigidBodyHandle>,
    pub collider_handles_by_entity: HashMap<Entity, ColliderHandle>,
}

impl Scaler {
    #[inline(always)]
    pub fn pixels_to_meters<T>(&self, value: T) -> T::Output
    where
        T: std::ops::Div<f32>,
    {
        value / self.scale
    }

    #[inline(always)]
    pub fn meters_to_pixels<T>(&self, value: T) -> T::Output
    where
        T: std::ops::Mul<f32>,
    {
        value * self.scale
    }
}

impl Physics {
    #[allow(clippy::let_unit_value)]
    pub fn step(&mut self) {
        let (collision_event_sender, _) = crossbeam::channel::unbounded();
        let (contact_force_event_sender, _) = crossbeam::channel::unbounded();

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
            &(),
            &ChannelEventCollector::new(collision_event_sender, contact_force_event_sender),
        );
    }

    //

    pub fn insert_body(&mut self, body: RigidBody, collider: Collider) -> (RigidBodyHandle, ColliderHandle) {
        let body_handle = self.bodies.insert(body);
        let collider_handle = self
            .colliders
            .insert_with_parent(collider, body_handle, &mut self.bodies);

        (body_handle, collider_handle)
    }

    pub fn remove_body(&mut self, body_handle: RigidBodyHandle) -> Option<RigidBody> {
        self.bodies.remove(
            body_handle,
            &mut self.island_manager,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            false,
        )
    }

    pub fn remove_collider(&mut self, collider_handle: ColliderHandle) -> Option<Collider> {
        self.colliders.remove(
            collider_handle,
            &mut self.island_manager,
            &mut self.bodies,
            false,
        )
    }

    //

    pub fn move_controller(
        &mut self,
        scaler: &Scaler,
        body_handle: &PhysicsBodyHandle,
        collider_handle: &PhysicsColliderHandle,
        collider_options: Option<&PhysicsColliderOptions>,
        character_controller: &mut PhysicsCharacterController,
    ) {
        let (movement, collisions) = {
            let body = self
                .bodies
                .get(body_handle.handle())
                .expect("Body not found");
            let collider = self
                .colliders
                .get(collider_handle.handle())
                .expect("Collider not found");
            let position = body.position();
            let controller = character_controller.rapier_controller;
            let collider_shape = collider.shape();
            let mut collisions = vec![];
            let mut query_filter = QueryFilter::default().exclude_rigid_body(body_handle.handle());

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
                scaler
                    .pixels_to_meters(character_controller.velocity)
                    .to_physics(),
                query_filter,
                |collision| {
                    collisions.push(collision);
                },
            );

            (movement, collisions)
        };

        let body = self
            .bodies
            .get_mut(body_handle.handle())
            .expect("Body not found");
        let position = body.position();

        body.set_next_kinematic_translation(position.translation.vector + movement.translation);
        character_controller.update_with_movement(movement, collisions);
    }
}

impl Hash for Scaler {
    fn hash<H: std::hash::Hasher>(&self, mut state: &mut H) {
        f32_hasher(self.scale, &mut state);
    }
}

impl Hash for Physics {
    #[cfg(feature = "stable")]
    fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
    #[cfg(not(feature = "stable"))]
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

impl Default for Scaler {
    fn default() -> Self {
        Self { scale: 100.0 }
    }
}

impl Default for Physics {
    fn default() -> Self {
        Self {
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
    scaler: Res<Scaler>,
    mut physics: ResMut<Physics>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));

    for (_, body_handle, collider_handle, collider_options, mut character_controller) in query {
        physics.move_controller(
            &scaler,
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
    scaler: Res<Scaler>,
    physics: Res<Physics>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));

    for (_, body_handle, mut transform) in query {
        if let Some(body) = physics.bodies.get(body_handle.handle()) {
            transform.rotation = ToBevyQuatExt::to_bevy(body.rotation());
            transform.translation = (scaler.meters_to_pixels(body.position().to_bevy())).extend(transform.translation.z);
        }
    }
}

#[allow(clippy::type_complexity)]
fn physics_update_system(
    body_query: Query<(
        &Rollback,
        &PhysicsBody,
        &PhysicsBodyHandle,
        &PhysicsBodyOptions,
    )>,
    collider_query: Query<(
        &Rollback,
        &PhysicsCollider,
        &PhysicsColliderHandle,
        &PhysicsColliderOptions,
    )>,
    velocity_query: Query<(
        &Rollback,
        &PhysicsBody,
        &PhysicsBodyHandle,
        &PhysicsBodyVelocity,
    )>,
    //
    order: Res<RollbackOrdered>,
    scaler: Res<Scaler>,
    mut physics: ResMut<Physics>,
) {
    let mut body_query = body_query.iter().collect::<Vec<_>>();
    let mut collider_query = collider_query.iter().collect::<Vec<_>>();
    let mut velocity_query = velocity_query.iter().collect::<Vec<_>>();

    body_query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));
    collider_query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));
    velocity_query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));

    for (_, body, body_handle, body_options) in body_query {
        body.apply_options(
            &scaler,
            physics
                .bodies
                .get_mut(body_handle.handle())
                .expect("Body not found"),
            body_options,
        );
    }
    for (_, body, body_handle, body_velocity) in velocity_query {
        body.apply_velocity(
            &scaler,
            physics
                .bodies
                .get_mut(body_handle.handle())
                .expect("Body not found"),
            body_velocity,
        );
    }
    for (_, collider, collider_handle, collider_options) in collider_query {
        collider.apply_options(
            &scaler,
            physics
                .colliders
                .get_mut(collider_handle.handle())
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
    scaler: Res<Scaler>,
    mut physics: ResMut<Physics>,
) {
    let mut query = query.iter().collect::<Vec<_>>();
    query.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollback(&order, rollback_a, rollback_b));

    for (e, _, transform, body, collider) in query {
        let body = body.build(&scaler, transform);
        let collider = collider.build(&scaler);
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
        .map(|b| b.handle())
        .collect::<HashSet<_>>();
    let collider_handles = query_collider_handles
        .iter()
        .map(|c| c.handle())
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
fn physics_debug_system(mut gizmos: Gizmos, scaler: Res<Scaler>, physics: Res<Physics>) {
    for (_, collider) in physics.colliders.iter() {
        if let Some(ball) = collider.shape().as_ball() {
            gizmos.circle_2d(
                Isometry2d {
                    translation: scaler.meters_to_pixels(collider.translation().to_bevy()),
                    ..default()
                },
                scaler.meters_to_pixels(ball.radius),
                Color::linear_rgba(1.0, 0.0, 1.0, 0.2),
            );
        }
        if let Some(cuboid) = collider.shape().as_cuboid() {
            gizmos.rect_2d(
                Isometry2d {
                    rotation: ToBevyRot2Ext::to_bevy(collider.rotation()),
                    translation: scaler.meters_to_pixels(collider.translation().to_bevy()),
                },
                scaler.meters_to_pixels(cuboid.half_extents.to_bevy()) * 2.0,
                Color::linear_rgba(0.0, 1.0, 0.0, 0.2),
            );
        }
    }
}

//

pub fn physics_systems() -> ScheduleConfigs<ScheduleSystem> {
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

pub fn physics_debug_systems() -> ScheduleConfigs<ScheduleSystem> {
    physics_debug_system.into_configs()
}
