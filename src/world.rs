use crate::coords::{ChunkPosition, LocalPosition, CHUNK_SIZE};
use crate::entities::entities::Entity;
use crate::entities::player::Player;
use crate::graphics::chunk_terrain::ChunkTerrain;
use crate::graphics::tile_atlas::TileAtlas;
use crate::tile_types::TileType;

use crate::generator::Generator;
use std::collections::HashMap;

/// Handles most of the generating and drawing the terrain.
pub struct World {
    pub positions_of_layers_in_view: Vec<ChunkPosition>,
    layers: HashMap<ChunkPosition, ChunkTerrain>,
}

impl World {
    /// Updates and possibly generates the layers that should be in view.
    pub fn update(&mut self, player_chunk: &ChunkPosition, generator: &Generator) {
        self.set_visible_layers(player_chunk, generator);
    }

    /// Draws every layer that is in view.
    pub fn draw(&self, tile_atlas: &TileAtlas, player: &Player) {
        let layers = self.get_visible_chunks(player);
        for layer in layers {
            tile_atlas.draw_layer(layer, player);
        }
    }

    /// Generates the layer at position and adds it to the world.
    /// (Or updates it the position is already in the world).
    fn gen_layer(&mut self, pos: ChunkPosition, generator: &Generator) {
        let x = pos.x * i32::from(CHUNK_SIZE);
        let y = pos.y * i32::from(CHUNK_SIZE);
        self.positions_of_layers_in_view.push(pos);
        let origin = (i64::from(x), i64::from(y));
        self.layers
            .insert(pos, Self::new_chunk_terrain(generator, origin));
    }

    /// Utility function to create a new `ChunkTerrain`
    fn new_chunk_terrain(gen: &Generator, origin: (i64, i64)) -> ChunkTerrain {
        let (positions, tile_types) = gen.generate_chunk_terrain(origin.0 as f32, origin.1 as f32);
        ChunkTerrain::new(origin, tile_types, positions)
    }

    /// Returns all the `ChunkTerrain` structures that should be in view.
    fn get_visible_chunks(&self, player: &Player) -> Vec<&ChunkTerrain> {
        self.positions_of_layers_in_view
            .iter()
            .map(|p| {
                self.layers
                    .get(p)
                    .expect("Tried to get layer that wasn't generated!")
            })
            .collect()
    }

    fn get_layer(&self, world_pos: &ChunkPosition) -> Option<&ChunkTerrain> {
        self.layers.get(world_pos)
    }

    pub fn get_tile(&self, world_pos: &ChunkPosition, pos: &LocalPosition) -> Option<&TileType> {
        self.get_layer(world_pos)
            .and_then(|layer| Some(layer.get_tile(pos)))
    }

    /// Updates layers in view to the square around the player position in the world,
    /// generating new layers if necessary.
    fn set_visible_layers(&mut self, player_position: &ChunkPosition, generator: &Generator) {
        let needed_positions: Vec<ChunkPosition> = vec![
            ChunkPosition::new(player_position.x - 1, player_position.y - 1),
            ChunkPosition::new(player_position.x, player_position.y - 1),
            ChunkPosition::new(player_position.x + 1, player_position.y - 1),
            ChunkPosition::new(player_position.x - 1, player_position.y),
            ChunkPosition::new(player_position.x, player_position.y),
            ChunkPosition::new(player_position.x + 1, player_position.y),
            ChunkPosition::new(player_position.x - 1, player_position.y + 1),
            ChunkPosition::new(player_position.x, player_position.y + 1),
            ChunkPosition::new(player_position.x + 1, player_position.y + 1),
        ];
        let mut to_generate: Vec<&ChunkPosition> = Vec::new();
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
        let positions_of_layers_in_view: Vec<ChunkPosition> = Vec::with_capacity(9);
        let layers: HashMap<ChunkPosition, ChunkTerrain> = HashMap::new();
        Self {
            positions_of_layers_in_view,
            layers,
        }
    }
}
