use bevy::prelude::*;
use bevy_ggrs::GgrsTime;

use crate::core::clock::Clock;

#[derive(Hash, Clone, Debug, Default, Component)]
pub struct SpriteSheetAnimator {
    pub clock: Clock,
    pub animation: Handle<SpriteSheetAnimation>,
}

#[derive(Asset, Debug, TypePath)]
pub struct SpriteSheetAnimation {
    pub start: usize,
    pub finish: usize,
}

pub fn sprite_sheet_animator_system(
    mut query: Query<(&mut TextureAtlas, &mut SpriteSheetAnimator)>,
    //
    time: Res<Time<GgrsTime>>,
    animations: Res<Assets<SpriteSheetAnimation>>,
) {
    for (mut atlas, mut animator) in query.iter_mut() {
        let animation = animations
            .get(animator.animation.id())
            .expect("Animation not found");

        animator.clock.tick(time.delta());
        if animator.clock.finished() {
            if (atlas.index < animation.start) || (atlas.index >= animation.finish) {
                atlas.index = animation.start;
            } else {
                atlas.index += 1;
            }

            animator.clock.reset();
        }
    }
}
