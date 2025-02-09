use std::fs;
use std::time::Duration;

use bevy::prelude::*;
use bevy::reflect::erased_serde::private::serde::Deserialize;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::sprite::Anchor;
use bevy::utils::HashMap;

use crate::item::WorldObject;
use crate::{GameState, ImageAssets};
use ron::de::from_str;

pub struct GameAssetsPlugin;

#[derive(Default, Clone, Copy, Debug, Reflect, Deserialize)]
pub struct MyRect {
    pub pos: (f32, f32),
    pub size: (f32, f32),
    pub anchor: Option<(f32, f32)>,
}

impl MyRect {
    pub fn new(pos: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            pos,
            size,
            anchor: None,
        }
    }

    pub fn to_atlas_rect(self) -> bevy::sprite::Rect {
        bevy::sprite::Rect {
            //A tiny amount is clipped off the sides of the rectangle
            //to stop contents of other sprites from bleeding through
            min: Vec2::new(self.pos.0 + 0.15, self.pos.1 + 0.15),
            max: Vec2::new(
                self.pos.0 + self.size.0 - 0.15,
                self.pos.1 + self.size.1 - 0.15,
            ),
        }
    }
}

#[derive(Deserialize)]
pub struct GraphicsDesc {
    map: HashMap<WorldObject, MyRect>,
}

impl Plugin for GameAssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_exit(GameState::Splash)
                .with_system(Self::load_graphics.label("graphics")),
        );
    }
}

pub const PIXEL_SCALE: f32 = 3.;
pub const SOURCE_TILE_SIZE: f32 = 32.;
pub const TILE_SIZE: f32 = SOURCE_TILE_SIZE * PIXEL_SCALE;

pub struct Graphics {
    pub texture_atlas: Handle<TextureAtlas>,
    pub player_index: usize,
    pub box_index: usize,
    pub item_map: HashMap<WorldObject, TextureAtlasSprite>,
    pub image_map: HashMap<WorldObject, Handle<Image>>,
}

fn convert_to_image(
    sprite_desc: MyRect,
    original_image: Handle<Image>,
    assets: &mut ResMut<Assets<Image>>,
) -> Handle<Image> {
    //TODO convert if mismatch
    let original_image = assets.get(original_image).unwrap();
    assert!(original_image.texture_descriptor.format == TextureFormat::Rgba8UnormSrgb);

    let mut data = Vec::default();
    //Every pixel is 4 entries in image.data
    let mut starting_index =
        (sprite_desc.pos.0 + original_image.size().x * sprite_desc.pos.1) as usize;
    for y in 0..sprite_desc.size.1 as usize {
        for x in 0..sprite_desc.size.0 as usize {
            let index = starting_index + x;
            //Copy 1 pixel at index
            data.push(original_image.data[index * 4]);
            data.push(original_image.data[index * 4 + 1]);
            data.push(original_image.data[index * 4 + 2]);
            data.push(original_image.data[index * 4 + 3]);
        }
        starting_index += original_image.size().y as usize;
    }

    let size = Extent3d {
        width: sprite_desc.size.0 as u32,
        height: sprite_desc.size.1 as u32,
        depth_or_array_layers: 1,
    };
    let image = Image::new(
        size,
        TextureDimension::D2,
        data,
        //FIXME
        TextureFormat::Rgba8UnormSrgb,
    );
    assets.add(image)
}

impl GameAssetsPlugin {
    fn load_graphics(
        mut commands: Commands,
        mut image_assets: ResMut<Assets<Image>>,
        sprite_sheet: Res<ImageAssets>,
        mut texture_assets: ResMut<Assets<TextureAtlas>>,
    ) {
        //let image_handle = assets.load("bevy_survival_sprites.png");
        let image_handle = sprite_sheet.sprite_sheet.clone();
        let sprite_desc = fs::read_to_string("assets/sprites_desc.ron").unwrap();

        let sprite_desc: GraphicsDesc = from_str(&sprite_desc).unwrap_or_else(|e| {
            println!("Failed to load config: {}", e);
            std::process::exit(1);
        });

        let mut atlas = TextureAtlas::new_empty(image_handle.clone(), Vec2::splat(256.0));

        let player_index = atlas.add_texture(MyRect::new((0., 0.), (32., 32.)).to_atlas_rect());

        let mut item_map = HashMap::default();
        let mut image_map = HashMap::default();

        for (item, rect) in sprite_desc.map.iter() {
            println!("Found graphic {:?}", item);
            let mut sprite = TextureAtlasSprite::new(atlas.add_texture(rect.to_atlas_rect()));

            //Set the size to be proportional to the source rectangle
            sprite.custom_size = Some(Vec2::new(
                rect.size.0 / SOURCE_TILE_SIZE,
                rect.size.1 / SOURCE_TILE_SIZE,
            ));

            //Position the sprite anchor if one is defined
            if let Some(anchor) = rect.anchor {
                sprite.anchor = Anchor::Custom(Vec2::new(
                    anchor.0 / rect.size.0 - 0.5,
                    0.5 - anchor.1 / rect.size.1,
                ));
            };

            item_map.insert(*item, sprite);
            image_map.insert(
                *item,
                convert_to_image(*rect, image_handle.clone(), &mut image_assets),
            );
        }

        let box_index = atlas.add_texture(MyRect::new((0., 32.), (32., 32.)).to_atlas_rect());

        let atlas_handle = texture_assets.add(atlas);

        commands.insert_resource(Graphics {
            texture_atlas: atlas_handle,
            player_index,
            box_index,
            item_map,
            image_map,
        });
    }
}
