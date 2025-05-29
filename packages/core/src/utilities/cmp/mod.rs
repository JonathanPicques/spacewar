use std::cmp::Ordering;

use bevy_ggrs::{Rollback, RollbackOrdered};

/// Compares two [`Rollback`]s components with the order resource managed by [`bevy_ggrs`].
pub fn cmp_rollback(order: &RollbackOrdered, rollback_a: &Rollback, rollback_b: &Rollback) -> Ordering {
    order
        .order(*rollback_a)
        .cmp(&order.order(*rollback_b))
}
