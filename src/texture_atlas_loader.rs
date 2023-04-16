use std::fs::File;
use std::io::BufReader;

use bevy::asset::Assets;
use bevy::math::{Rect, Vec2};
use bevy::prelude::{AssetServer, Handle, Res, ResMut};
use bevy::sprite::TextureAtlas;
use bevy::utils::HashMap;
use serde::Deserialize;

pub struct TextureAtlasLoader {
    sheet_path: String,
    width: i32,
    height: i32,
    sub_textures: HashMap<String, SubTexture>,
    pub texture_atlas_handle: Option<Handle<TextureAtlas>>,
}

impl TextureAtlasLoader {
    pub fn from_sheet_xml(folder: &str, sheet_name: &str) -> Self {
        let sheet_file = File::open(format!("assets/{}/{}.xml", folder, sheet_name)).unwrap();
        let sheet: TextureAtlasNode =
            quick_xml::de::from_reader(BufReader::new(sheet_file)).unwrap();
        let mut sub_textures = HashMap::new();
        for sub_texture in sheet.sub_textures.into_iter() {
            sub_textures.insert(
                sub_texture.name,
                SubTexture {
                    index: 0,
                    bounds: Rect::new(
                        sub_texture.x as f32,
                        sub_texture.y as f32,
                        (sub_texture.x + sub_texture.width) as f32,
                        (sub_texture.y + sub_texture.height) as f32,
                    ),
                },
            );
        }
        TextureAtlasLoader {
            sheet_path: format!("{}/{}", folder, sheet.image_path),
            width: sheet.width,
            height: sheet.height,
            sub_textures,
            texture_atlas_handle: None,
        }
    }

    pub fn load(
        &mut self,
        asset_server: &Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) {
        let texture_handle = asset_server.load(self.sheet_path.clone());
        let mut texture_atlas = TextureAtlas::new_empty(
            texture_handle,
            Vec2::new(self.width as f32, self.height as f32),
        );
        for (_, sub_texture) in self.sub_textures.iter_mut() {
            sub_texture.index = texture_atlas.add_texture(sub_texture.bounds);
        }
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        self.texture_atlas_handle = Some(texture_atlas_handle);
    }

    pub fn index_of(&self, sprite_name: &str) -> Option<usize> {
        self.sub_textures
            .get(sprite_name)
            .map(|sub_texture| sub_texture.index)
    }
}

#[derive(Debug)]
struct SubTexture {
    index: usize,
    bounds: Rect,
}

#[derive(Debug, Deserialize)]
struct TextureAtlasNode {
    #[serde(rename = "@imagePath")]
    image_path: String,
    #[serde(rename = "@width")]
    width: i32,
    #[serde(rename = "@height")]
    height: i32,
    #[serde(rename = "$value")]
    sub_textures: Vec<SpriteNode>,
}

#[derive(Debug, Deserialize)]
struct SpriteNode {
    #[serde(rename = "@n")]
    name: String,
    #[serde(rename = "@x")]
    x: i32,
    #[serde(rename = "@y")]
    y: i32,
    #[serde(rename = "@w")]
    width: i32,
    #[serde(rename = "@h")]
    height: i32,
}
