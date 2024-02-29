pub mod body;
pub mod collider;
pub mod context;
pub mod controller;

use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackOrdered};

use crate::core::physics::body::PhysicsBodyHandle;
use crate::core::physics::collider::PhysicsColliderHandle;
use crate::core::utilities::sorting::cmp_rollack;

pub use crate::core::physics::body::PhysicsBody;
pub use crate::core::physics::collider::PhysicsCollider;
pub use crate::core::physics::context::PhysicsContext;
pub use crate::core::physics::controller::PhysicsCharacterController;

pub fn physics_system(
    mut query: Query<(
        &Rollback,
        &PhysicsBodyHandle,
        &PhysicsColliderHandle,
        &mut PhysicsCharacterController,
    )>,
    //
    order: Res<RollbackOrdered>,
    mut physics_context: ResMut<PhysicsContext>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, body_handle, collider_handle, mut controller) in query {
        physics_context.move_controller(body_handle, collider_handle, &mut controller);
    }
    physics_context.step();
}

pub fn physics_sync_system(
    mut query: Query<(&Rollback, &PhysicsBodyHandle, &mut Transform)>,
    //
    order: Res<RollbackOrdered>,
    physics_context: Res<PhysicsContext>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, body, mut transform) in query {
        if let Some(body) = physics_context.bodies.get(body.0) {
            transform.translation.x = body.position().translation.x;
            transform.translation.y = body.position().translation.y;
        }
    }
}

pub fn physics_debug_system(mut gizmos: Gizmos, physics_context: Res<PhysicsContext>) {
    for (_, collider) in physics_context.colliders.iter() {
        if let Some(cuboid) = collider.shape().as_cuboid() {
            gizmos.rect_2d(
                Transform::from_translation(Vec3::new(
                    collider.translation().x,
                    collider.translation().y,
                    0.0,
                ))
                .translation
                .truncate(),
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
    mut physics_context: ResMut<PhysicsContext>,
) {
    let mut query = query.iter().collect::<Vec<_>>();
    query.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (e, _, transform, body, collider) in query {
        let (body_handle, collider_handle) = physics_context.insert_body(body, collider, transform);

        commands.entity(e).insert((
            PhysicsBodyHandle(body_handle),
            PhysicsColliderHandle(collider_handle),
        ));
    }
}
