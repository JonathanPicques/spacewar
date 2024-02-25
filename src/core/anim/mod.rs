use bevy::prelude::*;
use bevy_ggrs::GgrsTime;

#[derive(Default, Component)]
pub struct SpriteSheetAnimation {
    pub timer: Timer,
    pub start: usize,
    pub finish: usize,
}

pub fn sprite_sheet_animation_system(mut query: Query<(&mut TextureAtlasSprite, &mut SpriteSheetAnimation)>, time: Res<Time<GgrsTime>>) {
    for (mut texture_atlas_sprite, mut sprite_sheet_animation) in query.iter_mut() {
        sprite_sheet_animation.timer.tick(time.delta());
        if sprite_sheet_animation.timer.finished() {
            texture_atlas_sprite.index =
                if (texture_atlas_sprite.index < sprite_sheet_animation.start) || (texture_atlas_sprite.index >= sprite_sheet_animation.finish) {
                    sprite_sheet_animation.start
                } else {
                    texture_atlas_sprite.index + 1
                };
            sprite_sheet_animation.timer.reset();
        }
    }
}
