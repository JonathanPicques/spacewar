use bevy::prelude::*;
use bevy_ggrs::{GgrsTime, Rollback, RollbackOrdered};
use derivative::Derivative;

use crate::core::clock::Clock;
use crate::core::utilities::cmp::cmp_rollack;
use crate::core::utilities::maths::clamp;

#[derive(Clone, Default, Component, Derivative)]
#[derivative(Hash)]
pub struct SpriteSheetAnimator {
    #[cfg_attr(feature = "stable", derivative(Hash = "ignore"))]
    pub clock: Clock,
    pub finished: bool,
    pub animation: Handle<SpriteSheetAnimation>,
}

#[derive(Asset, TypePath)]
pub struct SpriteSheetAnimation {
    pub start: usize,
    pub finish: usize,
    pub repeat: bool,
}

impl SpriteSheetAnimator {
    pub fn finished(&self) -> bool {
        self.finished
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
        if animator.clock.finished() {
            animator.clock.reset();

            match animation.repeat {
                true => {
                    animator.finished = false;

                    if (atlas.index < animation.start) || (atlas.index >= animation.finish) {
                        atlas.index = animation.start;
                    } else {
                        atlas.index += 1;
                    }
                }
                false => {
                    atlas.index = clamp(atlas.index + 1, animation.start, animation.finish);
                    animator.finished = atlas.index == animation.finish;
                }
            };
        }
    }
}
