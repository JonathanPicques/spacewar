use std::time::Duration;

use bevy::prelude::*;
use bevy_ggrs::{GgrsTime, Rollback, RollbackOrdered};

use crate::clock::Clock;
use crate::utilities::cmp::cmp_rollack;

#[derive(Hash, Clone, PartialEq)]
enum State {
    Changed,
    Playing,
    Finished,
}

#[derive(Hash, Clone, Component)]
pub struct SpriteSheetAnimator {
    state: State,
    clock: Clock,
    animation: Handle<SpriteSheetAnimation>,
}

#[derive(Asset, TypePath)]
pub struct SpriteSheetAnimation {
    pub speed: f32,
    pub start: usize,
    pub finish: usize,
    pub repeat: bool,
}

impl SpriteSheetAnimator {
    pub fn new(animation: Handle<SpriteSheetAnimation>) -> Self {
        Self { state: State::Changed, clock: default(), animation }
    }
}

impl SpriteSheetAnimator {
    pub fn is_finished(&self) -> bool {
        self.state == State::Finished
    }

    pub fn set_animation(&mut self, animation: Handle<SpriteSheetAnimation>) {
        self.clock.reset();
        self.state = State::Changed;
        self.animation = animation;
    }
}

pub fn sprite_sheet_animator_system(
    mut query: Query<(
        &Rollback,
        &mut TextureAtlas,
        &mut SpriteSheetAnimator,
    )>,
    //
    time: Res<Time<GgrsTime>>,
    order: Res<RollbackOrdered>,
    animations: Res<Assets<SpriteSheetAnimation>>,
) {
    let mut query = query.iter_mut().collect::<Vec<_>>();
    query.sort_by(|(rollback_a, ..), (rollback_b, ..)| cmp_rollack(&order, rollback_a, rollback_b));

    for (_, mut atlas, mut animator) in query {
        let animation = animations
            .get(animator.animation.id())
            .expect("Animation not found");

        animator.clock.tick(time.delta());
        if animator.state == State::Changed {
            atlas.index = animation.start;
            animator
                .clock
                .set_duration(Duration::from_secs_f32(animation.speed));
            animator.clock.reset();
            animator.state = State::Playing;
        }
        if animator.clock.is_finished() {
            animator.clock.reset();

            match animation.repeat {
                true => {
                    if (atlas.index < animation.start) || (atlas.index >= animation.finish) {
                        atlas.index = animation.start;
                    } else {
                        atlas.index += 1;
                    }
                }
                false => {
                    if atlas.index < animation.finish {
                        atlas.index += 1;
                    } else {
                        animator.state = State::Finished;
                    }
                }
            };
        }
    }
}
