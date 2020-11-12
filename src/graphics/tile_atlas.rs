use macroquad::{draw_texture_ex, load_texture, Color, DrawTextureParams, Rect, Texture2D, Vec2};

use crate::graphics::layer::Layer;
use crate::graphics::tile::{Brightness, TileType};

use crate::entities::entities::{distance, Entity};
use crate::entities::player::Player;
/// Is used to split one `Texture2D` into different tiles.
#[derive(Clone, Debug)]
pub struct TileAtlas {
    texture: Texture2D,
    tile_width: f32,
    tile_height: f32,
}

impl TileAtlas {
    /// Initialize the atlas from the texture and tile size.
    pub const fn new(texture: Texture2D, tile_width: f32, tile_height: f32) -> Self {
        Self {
            texture,
            tile_width,
            tile_height,
        }
    }

    /// Initialize the atlas from the texture with 32 pixels tile size.
    pub async fn default() -> Self {
        Self {
            texture: load_texture("assets/Tiles.png").await,
            tile_width: 32.,
            tile_height: 32.,
        }
    }

    /// Draws the provided `&Entity`.
    pub fn draw_entity(&self, entity: &Entity, brightness: Brightness) {
        let params = self.get_texture_params(entity.tile);
        let (x, y) = entity.get_absolute_position();
        draw_texture_ex(
            self.texture,
            f32::from(x),
            f32::from(y),
            Color::from(brightness),
            params,
        );
    }

    /// Draws every tile from the provided `&Layer`.
    pub fn draw_layer(&self, layer: &Layer, player: &Player) {
        for (tile_type, position) in layer {
            let (relative_x, relative_y) = position.into();
            let (x, y) = (
                (layer.origin.0 + i64::from(relative_x) + 1) as f32,
                (layer.origin.1 + i64::from(relative_y)) as f32,
            );

            let dist = distance(player.entity.get_absolute_position(), (x, y));
            if dist < player.vision_range.into() {
                let brightness = player.calc_brightness(dist);
                let params = self.get_texture_params(tile_type);
                #[allow(clippy::cast_precision_loss)]
                draw_texture_ex(
                    self.texture,
                    x as f32,
                    y as f32,
                    Color::from(brightness),
                    params,
                );
            }
        }
    }

    /// Util function to get texture parameters.
    fn get_texture_params(&self, tile_type: TileType) -> DrawTextureParams {
        let (atlas_x, atlas_y) = Self::get_atlas_position(tile_type);
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(-1.0, 1.0)),
            source: Some(Rect {
                x: (self.tile_width + 0.3) * atlas_x,
                y: (self.tile_height + 0.3) * atlas_y,
                w: self.tile_width - 1.0,
                h: self.tile_height - 1.0,
            }),
            rotation: std::f32::consts::PI,
        };
        params
    }

    /// Position of tiletype in atlas.
    const fn get_atlas_position(tile_type: TileType) -> (f32, f32) {
        match tile_type {
            TileType::Debug => (0., 0.),
            TileType::Wall => (1., 0.),
            TileType::GrassFloor => (2., 0.),
            TileType::Pengu => (3., 0.),
            TileType::Door => (0., 1.),
            TileType::Chest => (1., 1.),
            TileType::Coin => (2., 1.),
            TileType::Cat => (3., 1.),
            TileType::StoneFloor => (0., 2.),
            TileType::Bush => (1., 2.),
            TileType::Stones => (2., 2.),
            TileType::Pond => (3., 2.),
        }
    }
}
