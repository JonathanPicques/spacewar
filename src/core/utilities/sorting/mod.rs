use std::cmp::Ordering;

use bevy_ggrs::{Rollback, RollbackOrdered};

pub fn cmp_rollack(order: &RollbackOrdered, rollback_a: &&Rollback, rollback_b: &&Rollback) -> Ordering {
    order
        .order(**rollback_a)
        .cmp(&order.order(**rollback_b))
}
