use macroquad::{
    draw_texture_ex, load_texture, vec2, Color, DrawTextureParams, Rect, Texture2D, Vec2,
};
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
            tile_width: 32f32,
            tile_height: 32f32,
        }
    }

    // Draw provided Tiles kind (e.g. `Tiles::Grass`) to the given position.
    /*
    pub fn draw_tile(&self, tile: &Tile, pos: &Position, color: Color) {
        let (atlas_position_x, atlas_position_y) = tile.value();
        let params = DrawTextureParams {
            dest_size: Some(Vec2::one()),
            source: Some(Rect {
                x: (self.tile_width + 0.2) * atlas_position_x,
                y: self.tile_height * atlas_position_y,
                w: self.tile_width - 1.0,
                h: self.tile_height - 1.0,
            }),
            rotation: std::f32::consts::PI,
        };
        draw_texture_ex(self.texture, pos.x as f32, pos.y as f32, color, params);
    }
    */
}
