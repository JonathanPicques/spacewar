use std::time::Duration;

use bevy::prelude::*;

#[derive(Default, Component)]
pub struct SpriteSheetAnimation {
    pub timer: Timer,
}

pub fn anim_system(
    mut query: Query<(
        &Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut SpriteSheetAnimation,
    )>,
    textures: Res<Assets<TextureAtlas>>,
) {
    for (texture_atlas, mut texture_atlas_sprite, mut sprite_sheet_animation) in query.iter_mut() {
        if let Some(texture_atlas) = textures.get(texture_atlas) {
            let nb_frames = texture_atlas.textures.len();

            sprite_sheet_animation
                .timer
                .tick(Duration::from_millis(100));
            if sprite_sheet_animation.timer.finished() {
                texture_atlas_sprite.index = (texture_atlas_sprite.index + 1) % nb_frames;
                sprite_sheet_animation.timer.reset();
            }
        }
    }
}
