use macroquad::{draw_texture_ex, load_texture, Color, DrawTextureParams, Rect, Texture2D, Vec2};

use crate::tile::{Tile, TileType};
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

    /// Draws the tile.
    pub fn draw_tile(&self, tile: &Tile) {
        let (atlas_x, atlas_y) = Self::get_atlas_position(tile.tile_type);
        let params = DrawTextureParams {
            dest_size: Some(Vec2::one()),
            source: Some(Rect {
                x: (self.tile_width + 0.3) * atlas_x,
                y: (self.tile_height + 0.3) * atlas_y,
                w: self.tile_width - 1.0,
                h: self.tile_height - 1.0,
            }),
            rotation: std::f32::consts::PI,
        };
        let (x, y) = tile.position.into();
        draw_texture_ex(
            self.texture,
            f32::from(x),
            f32::from(y),
            Color::from(tile.brightness),
            params,
        );
    }

    /// Position of tiletype in atlas.
    const fn get_atlas_position(tile_type: TileType) -> (f32, f32) {
        match tile_type {
            TileType::Debug => (0., 0.),
            TileType::Wall => (1., 0.),
            TileType::Grass => (2., 0.),
            TileType::Pengu => (3., 0.),
            TileType::Door => (0., 1.),
            TileType::Chest => (1., 1.),
            TileType::Coin => (2., 1.),
            TileType::Cat => (3., 1.),
        }
    }
}
