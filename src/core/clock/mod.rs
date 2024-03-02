use std::time::Duration;

use bevy::prelude::*;

#[derive(Hash, Clone, Default)]
pub struct Clock {
    elapsed: Duration,
    duration: Duration,
    finished: bool,
}

impl Clock {
    pub fn new(duration: Duration) -> Self {
        Self { duration, ..default() }
    }

    pub fn new_finished(duration: Duration) -> Self {
        Self { duration, finished: true, ..default() }
    }

    #[inline]
    pub fn tick(&mut self, delta: Duration) -> &Self {
        self.elapsed += delta;
        self.finished = self.elapsed >= self.duration;
        self
    }

    #[inline]
    pub fn reset(&mut self) {
        self.elapsed = default();
        self.finished = self.elapsed >= self.duration;
    }

    #[inline]
    pub fn finished(&self) -> bool {
        self.finished
    }
}
