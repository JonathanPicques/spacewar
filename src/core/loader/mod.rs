use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use serde::{Deserialize, Serialize};

use crate::core::anim::SpriteSheetAnimation;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum Asset {
    Image(ImageAsset),
    TextureAtlasLayout(TextureAtlasLayoutAsset),
    SpriteSheetAnimation(SpriteSheetAnimationAsset),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ImageAsset {
    path: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct TextureAtlasLayoutAsset {
    rows: usize,
    columns: usize,
    tile_size_x: f32,
    tile_size_y: f32,
    //
    offset_x: f32,
    offset_y: f32,
    padding_x: f32,
    padding_y: f32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct SpriteSheetAnimationAsset {
    start: usize,
    finish: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
enum CoreDynamicAsset {
    Asset(Asset),
    Assets(Vec<Asset>),
}

impl DynamicAsset for CoreDynamicAsset {
    fn load(&self, asset_server: &AssetServer) -> Vec<UntypedHandle> {
        let load_asset = |asset: Asset| match asset {
            Asset::Image(ImageAsset { path, .. }) => asset_server.load::<Image>(path).untyped(),
            Asset::TextureAtlasLayout(TextureAtlasLayoutAsset { .. }) => asset_server
                .add(TextureAtlasLayout::new_empty(Vec2::ONE))
                .untyped(),
            Asset::SpriteSheetAnimation(SpriteSheetAnimationAsset { start, finish }) => asset_server
                .add(SpriteSheetAnimation { start, finish })
                .untyped(),
        };

        match self {
            CoreDynamicAsset::Asset(asset) => vec![load_asset(asset.clone())],
            CoreDynamicAsset::Assets(assets) => assets
                .iter()
                .map(|asset| load_asset(asset.clone()))
                .collect(),
        }
    }

    fn build(&self, world: &mut World) -> Result<DynamicAssetType, anyhow::Error> {
        let cell = world.cell();
        let asset_server = cell
            .get_resource::<AssetServer>()
            .expect("Failed to get asset server");
        let mut texture_atlas_layouts = cell
            .get_resource_mut::<Assets<TextureAtlasLayout>>()
            .expect("Failed to get Assets<TextureAtlas>");

        let mut build_asset = |asset: Asset| match asset {
            Asset::Image(ImageAsset { path }) => asset_server.load::<Image>(path).untyped(),
            Asset::TextureAtlasLayout(TextureAtlasLayoutAsset {
                rows,
                columns,
                offset_x,
                offset_y,
                padding_x,
                padding_y,
                tile_size_x,
                tile_size_y,
            }) => texture_atlas_layouts
                .add(TextureAtlasLayout::from_grid(
                    Vec2::new(tile_size_x, tile_size_y),
                    columns,
                    rows,
                    Some(Vec2::new(padding_x, padding_y)),
                    Some(Vec2::new(offset_x, offset_y)),
                ))
                .untyped(),
            Asset::SpriteSheetAnimation(SpriteSheetAnimationAsset { start, finish }) => asset_server
                .add(SpriteSheetAnimation { start, finish })
                .untyped(),
        };
        match self {
            CoreDynamicAsset::Asset(asset) => Ok(DynamicAssetType::Single(build_asset(
                asset.clone(),
            ))),
            CoreDynamicAsset::Assets(assets) => Ok(DynamicAssetType::Collection(
                assets
                    .iter()
                    .map(|asset| build_asset(asset.clone()))
                    .collect(),
            )),
        }
    }
}

#[derive(Asset, Debug, TypePath, PartialEq, Serialize, Deserialize)]
pub struct CoreDynamicAssetCollection(HashMap<String, CoreDynamicAsset>);

impl DynamicAssetCollection for CoreDynamicAssetCollection {
    fn register(&self, dynamic_assets: &mut DynamicAssets) {
        for (key, asset) in self.0.iter() {
            dynamic_assets.register_asset(key, Box::new(asset.clone()));
        }
    }
}
