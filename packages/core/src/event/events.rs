use std::hash::Hash;
use std::slice::Iter;

use bevy::prelude::*;

pub trait RollbackEvent: Send + Sync + Hash + Clone + 'static {}

#[derive(Hash, Clone, Resource)]
pub struct RollbackEvents<E>
where
    E: RollbackEvent,
{
    events: Vec<E>,
}

impl<E> RollbackEvents<E>
where
    E: RollbackEvent,
{
    pub fn iter(&self) -> Iter<'_, E> {
        self.events.iter()
    }

    pub fn push(&mut self, value: E) {
        self.events.push(value);
    }

    pub fn clear(&mut self) {
        self.events.clear()
    }
}

impl<E> Default for RollbackEvents<E>
where
    E: RollbackEvent,
{
    fn default() -> Self {
        Self { events: default() }
    }
}
