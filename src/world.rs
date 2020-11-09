use crate::graphics::layer::{Layer, LAYER_DIMENSIONS};
use crate::graphics::tile::{Tile, TileType};
use crate::graphics::tile_atlas::TileAtlas;

use std::collections::HashMap;

use simdnoise::NoiseBuilder;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct WorldPosition {
    x: i32,
    y: i32,
}

/// Sets the world position to a tuple without scaling.
impl WorldPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Into<(i32, i32)> for WorldPosition {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(i32, i32)> for WorldPosition {
    fn from(pos: (i32, i32)) -> Self {
        Self {
            x: pos.0 / i32::from(LAYER_DIMENSIONS),
            y: pos.1 / i32::from(LAYER_DIMENSIONS),
        }
    }
}

impl From<(u16, u16)> for WorldPosition {
    fn from(pos: (u16, u16)) -> Self {
        Self {
            x: i32::from(pos.0 / LAYER_DIMENSIONS),
            y: i32::from(pos.1 / LAYER_DIMENSIONS),
        }
    }
}

/// Handles most of the generating and drawing the terrain.
pub struct World {
    pub positions_of_layers_in_view: Vec<WorldPosition>,
    layers: HashMap<WorldPosition, Layer>,

    generator: Generator,
}

impl World {
    /// Updates and possibly generates the layers that should be in view.
    pub fn update(&mut self, player: &WorldPosition) {
        self.set_visible_layers(player);
    }

    /// Draws every layer that is in view.
    pub fn draw(&self, tile_atlas: &TileAtlas) {
        let layers = self.get_visible_layers();
        for layer in layers {
            tile_atlas.draw_layer(layer);
        }
    }

    /// Generates the layer at position and adds it to the world.
    /// (Or updates it the position is already in the world).
    fn gen_layer(&mut self, pos: WorldPosition) {
        let x = pos.x * i32::from(LAYER_DIMENSIONS);
        let y = pos.y * i32::from(LAYER_DIMENSIONS);
        self.positions_of_layers_in_view.push(pos);
        let origin = (i64::from(x), i64::from(y));
        self.layers
            .insert(pos, Self::new_layer(&self.generator, origin));
    }

    /// Utility function to create a new `Layer`
    fn new_layer(gen: &Generator, origin: (i64, i64)) -> Layer {
        let tiles = gen.generate_layer_tiles(origin.0 as f32, origin.1 as f32);
        Layer::new(origin, &tiles)
    }

    /// Returns all the `Layers` that should be in view.
    fn get_visible_layers(&self) -> Vec<&Layer> {
        self.positions_of_layers_in_view
            .iter()
            .map(|p| {
                self.layers
                    .get(p)
                    .expect("Tried to get layer that wasn't generated!")
            })
            .collect()
    }

    /// Updates layers in view to the square around the player position in the world,
    /// generating new layers if necessary.
    fn set_visible_layers(&mut self, player_position: &WorldPosition) {
        let needed_positions: Vec<WorldPosition> = vec![
            WorldPosition::new(player_position.x - 1, player_position.y - 1),
            WorldPosition::new(player_position.x, player_position.y - 1),
            WorldPosition::new(player_position.x + 1, player_position.y - 1),
            WorldPosition::new(player_position.x - 1, player_position.y),
            WorldPosition::new(player_position.x, player_position.y),
            WorldPosition::new(player_position.x + 1, player_position.y),
            WorldPosition::new(player_position.x - 1, player_position.y + 1),
            WorldPosition::new(player_position.x, player_position.y + 1),
            WorldPosition::new(player_position.x + 1, player_position.y + 1),
        ];
        let mut to_generate: Vec<&WorldPosition> = Vec::new();
        for pos in &needed_positions {
            if !self.layers.contains_key(pos) {
                to_generate.push(pos);
            }
        }

        for layer in to_generate {
            self.gen_layer(layer.clone());
        }

        self.positions_of_layers_in_view = needed_positions;
    }
}

impl Default for World {
    fn default() -> Self {
        let positions_of_layers_in_view: Vec<WorldPosition> = Vec::with_capacity(9);
        let layers: HashMap<WorldPosition, Layer> = HashMap::new();
        let generator: Generator = Generator::default();
        Self {
            positions_of_layers_in_view,
            layers,
            generator,
        }
    }
}

#[derive(Default)]
struct Generator {
    seed: i32,
}

impl Generator {
    fn new(seed: i32) -> Self {
        Self { seed }
    }

    fn generate_layer_tiles(&self, x_offset: f32, y_offset: f32) -> Vec<Vec<Tile>> {
        let noise = NoiseBuilder::gradient_2d_offset(x_offset, 64, y_offset, 64)
            .with_seed(self.seed)
            .with_freq(0.015)
            .generate_scaled(0.0, 255.0);

        let tile = Tile {
            tile_type: TileType::Debug,
            position: (0, 0).into(),
            brightness: 255.into(),
        };

        let mut tiles = vec![vec![tile.clone(); LAYER_DIMENSIONS.into()]; LAYER_DIMENSIONS.into()];

        for y in 0..LAYER_DIMENSIONS.into() {
            for x in 0..LAYER_DIMENSIONS.into() {
                tiles[y][x].position = (x as i16, y as i16).into();
                let number = *noise.get(y * 64 + x).unwrap() as u8;
                tiles[y][x].tile_type = match number {
                    0..=127 => TileType::Grass,
                    128..=255 => TileType::Wall,
                }
            }
        }

        tiles
    }
}
