use crate::graphics::layer::{Layer, LAYER_DIMENSIONS};
use crate::graphics::tile::{Position, TileType};
use crate::graphics::tile_atlas::TileAtlas;

use crate::entities::entities::Entity;
use crate::entities::player::Player;

use crate::generator::Generator;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct WorldPosition {
    pub x: i32,
    pub y: i32,
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
}

impl World {
    /// Updates and possibly generates the layers that should be in view.
    pub fn update(&mut self, player_chunk: &WorldPosition, generator: &Generator) {
        self.set_visible_layers(player_chunk, generator);
    }

    /// Draws every layer that is in view.
    pub fn draw(&self, tile_atlas: &TileAtlas, player: &Player) {
        let layers = self.get_visible_layers(player);
        for layer in layers {
            tile_atlas.draw_layer(layer, player);
        }
    }

    /// Generates the layer at position and adds it to the world.
    /// (Or updates it the position is already in the world).
    fn gen_layer(&mut self, pos: WorldPosition, generator: &Generator) {
        let x = pos.x * i32::from(LAYER_DIMENSIONS);
        let y = pos.y * i32::from(LAYER_DIMENSIONS);
        self.positions_of_layers_in_view.push(pos);
        let origin = (i64::from(x), i64::from(y));
        self.layers.insert(pos, Self::new_layer(generator, origin));
    }

    /// Utility function to create a new `Layer`
    fn new_layer(gen: &Generator, origin: (i64, i64)) -> Layer {
        let (positions, tile_types) = gen.generate_layer_tiles(origin.0 as f32, origin.1 as f32);
        Layer::new(origin, tile_types, positions)
    }

    /// Returns all the `Layers` that should be in view.
    fn get_visible_layers(&self, player: &Player) -> Vec<&Layer> {
        self.positions_of_layers_in_view
            .iter()
            .map(|p| {
                self.layers
                    .get(p)
                    .expect("Tried to get layer that wasn't generated!")
            })
            .collect()
    }

    fn get_layer(&self, world_pos: &WorldPosition) -> Option<&Layer> {
        self.layers.get(world_pos)
    }

    pub fn get_tile(&self, world_pos: &WorldPosition, pos: &Position) -> Option<&TileType> {
        self.get_layer(world_pos)
            .and_then(|layer| Some(layer.get_tile(pos)))
    }

    /// Updates layers in view to the square around the player position in the world,
    /// generating new layers if necessary.
    fn set_visible_layers(&mut self, player_position: &WorldPosition, generator: &Generator) {
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
            self.gen_layer(layer.clone(), generator);
        }

        self.positions_of_layers_in_view = needed_positions;
    }
}

impl Default for World {
    fn default() -> Self {
        let positions_of_layers_in_view: Vec<WorldPosition> = Vec::with_capacity(9);
        let layers: HashMap<WorldPosition, Layer> = HashMap::new();
        Self {
            positions_of_layers_in_view,
            layers,
        }
    }
}
