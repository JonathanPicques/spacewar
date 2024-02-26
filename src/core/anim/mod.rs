use bevy::prelude::*;
use bevy_ggrs::GgrsTime;

#[derive(Default, Component)]
pub struct SpriteSheetAnimator {
    pub timer: Timer,
    pub animation: Handle<SpriteSheetAnimation>,
}

#[derive(Asset, TypePath)]
pub struct SpriteSheetAnimation {
    pub start: usize,
    pub finish: usize,
}

pub fn sprite_sheet_animator_system(
    mut query: Query<(&mut TextureAtlasSprite, &mut SpriteSheetAnimator)>,
    //
    time: Res<Time<GgrsTime>>,
    animations: Res<Assets<SpriteSheetAnimation>>,
) {
    for (mut sprite, mut animator) in query.iter_mut() {
        let animation = animations
            .get(animator.animation.id())
            .expect("Animation not found");

        animator.timer.tick(time.delta());
        if animator.timer.finished() {
            if (sprite.index < animation.start) || (sprite.index >= animation.finish) {
                sprite.index = animation.start;
            } else {
                sprite.index += 1;
            }

            animator.timer.reset();
        }
    }
}
