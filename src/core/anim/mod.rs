use bevy::prelude::*;
use bevy_ggrs::GgrsTime;

#[derive(Default, Component)]
pub struct SpriteSheetAnimation {
    pub timer: Timer,
}

pub fn sprite_sheet_animation_system(
    mut query: Query<(
        &Handle<TextureAtlas>,
        &mut TextureAtlasSprite,
        &mut SpriteSheetAnimation,
    )>,
    time: Res<Time<GgrsTime>>,
    textures: Res<Assets<TextureAtlas>>,
) {
    for (texture_atlas, mut texture_atlas_sprite, mut sprite_sheet_animation) in query.iter_mut() {
        if let Some(texture_atlas) = textures.get(texture_atlas) {
            let nb_frames = texture_atlas.textures.len();

            sprite_sheet_animation.timer.tick(time.delta());
            if sprite_sheet_animation.timer.finished() {
                texture_atlas_sprite.index = (texture_atlas_sprite.index + 1) % nb_frames;
                sprite_sheet_animation.timer.reset();
            }
        }
    }
}
