use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::{GgrsTime, Rollback, RollbackOrdered};

use crate::core::utilities::cmp::cmp_rollack;

#[derive(Hash, Copy, Clone)]
pub struct Clock {
    elapsed: Duration,
    duration: Duration,
    finished: bool,
}

#[derive(Hash, Clone, Component)]
pub struct TimeToLive {
    clock: Clock,
}

impl Clock {
    pub fn new(duration: Duration) -> Self {
        Self { duration, finished: false, ..default() }
    }

    pub fn from_secs_f32(secs: f32) -> Self {
        Self {
            duration: Duration::from_secs_f32(secs),
            finished: false,
            ..default()
        }
    }

    pub fn with_finished(mut self, finished: bool) -> Self {
        self.finished = finished;
        self
    }

    #[inline]
    pub fn tick(&mut self, delta: Duration) -> &Self {
        self.elapsed += delta;
        if self.elapsed >= self.duration {
            self.finished = true;
        }
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

impl TimeToLive {
    pub fn from_secs_f32(secs: f32) -> Self {
        Self { clock: Clock::from_secs_f32(secs) }
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self {
            elapsed: default(),
            duration: default(),
            finished: true,
        }
    }
}

impl Default for TimeToLive {
    fn default() -> Self {
        Self { clock: Clock::from_secs_f32(1.0) }
    }
}

pub fn ttl_system(
    mut query: Query<(Entity, &Rollback, &mut TimeToLive)>,
    mut commands: Commands,
    //
    time: Res<Time<GgrsTime>>,
    order: Res<RollbackOrdered>,
) {
    let delta = time.delta_seconds();
    let delta_d = Duration::from_secs_f32(delta);
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(_, rollback_a, ..), (_, rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (e, _, mut ttl) in query {
        if ttl.clock.tick(delta_d).finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}
