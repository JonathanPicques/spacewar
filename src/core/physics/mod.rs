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
    mut query: Query<
        (
            &Rollback,
            &PhysicsBody,
            &PhysicsCollider,
            &mut Transform,
            &mut PhysicsCharacterController,
        ),
        (
            With<PhysicsBodyHandle>,
            With<PhysicsColliderHandle>,
        ),
    >,
    //
    order: Res<RollbackOrdered>,
    mut physics_context: ResMut<PhysicsContext>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, _, _, mut transform, mut controller) in query {
        transform.translation += controller.velocity.extend(0.0);
        controller.on_floor = false;

        if transform.translation.y <= -0.0 {
            transform.translation.y = -0.0;
            controller.on_floor = true;
            controller.velocity.y = 0.0;
        }
    }
    physics_context.step();
}

pub fn physics_create_handles_system(
    query: Query<
        (Entity, &Rollback, &PhysicsBody, &PhysicsCollider),
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

    for (e, _, body, collider) in query {
        let (body_handle, collider_handle) = physics_context.insert_body(body, collider);

        commands.entity(e).insert((
            PhysicsBodyHandle(body_handle),
            PhysicsColliderHandle(collider_handle),
        ));
    }
}
