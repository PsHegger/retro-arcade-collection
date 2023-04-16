use bevy::app::{App, Plugin};
use bevy::asset::Handle;
use bevy::prelude::*;
use bevy::sprite::TextureAtlas;

use crate::texture_atlas_loader::TextureAtlasLoader;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpaceShooterSpriteSheet::default());
    }
}

#[derive(Resource)]
pub struct SpaceShooterSpriteSheet(TextureAtlasLoader);

impl SpaceShooterSpriteSheet {
    pub fn atlas_handle(&self) -> Option<Handle<TextureAtlas>> {
        self.0.texture_atlas_handle.clone()
    }

    pub fn index_of(&self, sprite_name: &str) -> Option<usize> {
        self.0.index_of(sprite_name)
    }

    pub fn bounds_of(&self, sprite_name: &str) -> Option<Rect> {
        self.0.bounds_of(sprite_name)
    }

    pub fn load(
        &mut self,
        asset_server: &Res<AssetServer>,
        texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        self.0.load(asset_server, texture_atlases);
    }
}

impl Default for SpaceShooterSpriteSheet {
    fn default() -> Self {
        SpaceShooterSpriteSheet(TextureAtlasLoader::from_sheet_xml(
            "sprites/asteroid",
            "space_shooter_sheet",
        ))
    }
}
