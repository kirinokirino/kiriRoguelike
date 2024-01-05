use crate::coords::{AbsolutePosition, ChunkPosition};
use crate::entities::player::Player;
use crate::generator::Generator;
use crate::graphics::chunk_terrain::ChunkTerrain;
use crate::graphics::tile_atlas::TileAtlas;
use crate::tile_types::TileType;

use std::collections::HashMap;

/// Handles the chunks, terrain.
pub struct World {
    pub positions_of_chunks_in_view: Vec<ChunkPosition>,
    chunks: HashMap<ChunkPosition, ChunkTerrain>,
}

impl World {
    /// Updates and possibly generates the chunks that should be in view.
    pub fn update(&mut self, player_chunk: &ChunkPosition, generator: &Generator) {
        self.set_visible_layers(player_chunk, generator);
    }

    /// Draws every chunk that is in view.
    pub fn draw(&self, tile_atlas: &TileAtlas, player: &Player) {
        let chunks = self.get_visible_chunks();
        for chunk in chunks {
            tile_atlas.draw_layer(chunk, player);
        }
    }

    /// Generates the chunk at `ChunkPosition` and adds it to the world.
    fn gen_chunk(&mut self, chunk_pos: ChunkPosition, generator: &Generator) {
        self.positions_of_chunks_in_view.push(chunk_pos);
        self.chunks
            .insert(chunk_pos, Self::new_chunk_terrain(generator, chunk_pos));
    }

    /// Utility function to create a new `ChunkTerrain`
    fn new_chunk_terrain(gen: &Generator, chunk_pos: ChunkPosition) -> ChunkTerrain {
        let (positions, tile_types) = gen.generate_chunk_terrain(chunk_pos);
        ChunkTerrain::new(chunk_pos, tile_types, positions)
    }

    /// Returns all the `ChunkTerrain` structures that should be in view.
    fn get_visible_chunks(&self) -> Vec<&ChunkTerrain> {
        self.positions_of_chunks_in_view
            .iter()
            .map(|p| {
                self.chunks
                    .get(p)
                    .expect("Tried to get chunk that wasn't generated!")
            })
            .collect()
    }

    /// Get the option reference to the chunk at `ChunkPosition`.
    pub fn get_chunk(&self, chunk_pos: &ChunkPosition) -> Option<&ChunkTerrain> {
        self.chunks.get(chunk_pos)
    }

    /// Get the option reference to the tile at `AbsolutePosition`
    pub fn get_tile(&self, position: &AbsolutePosition) -> Option<&TileType> {
        self.get_chunk(&position.chunk)
            .map(|chunk| chunk.get_tile(&position.local))
    }

    /// Updates chunks in view to be in the square formation around the
    /// player's `ChunkPosition`, generating new chunks if necessary.
    fn set_visible_layers(&mut self, player_chunk: &ChunkPosition, generator: &Generator) {
        let needed_chunks: Vec<ChunkPosition> = vec![
            ChunkPosition::new(player_chunk.x - 1, player_chunk.y - 1),
            ChunkPosition::new(player_chunk.x, player_chunk.y - 1),
            ChunkPosition::new(player_chunk.x + 1, player_chunk.y - 1),
            ChunkPosition::new(player_chunk.x - 1, player_chunk.y),
            ChunkPosition::new(player_chunk.x, player_chunk.y),
            ChunkPosition::new(player_chunk.x + 1, player_chunk.y),
            ChunkPosition::new(player_chunk.x - 1, player_chunk.y + 1),
            ChunkPosition::new(player_chunk.x, player_chunk.y + 1),
            ChunkPosition::new(player_chunk.x + 1, player_chunk.y + 1),
        ];
        let mut to_generate: Vec<&ChunkPosition> = Vec::new();
        for chunk in &needed_chunks {
            if !self.chunks.contains_key(chunk) {
                to_generate.push(chunk);
            }
        }

        for chunk in to_generate {
            self.gen_chunk(*chunk, generator);
        }

        self.positions_of_chunks_in_view = needed_chunks;
    }
}

impl Default for World {
    fn default() -> Self {
        let positions_of_chunks_in_view: Vec<ChunkPosition> = Vec::with_capacity(9);
        let chunks: HashMap<ChunkPosition, ChunkTerrain> = HashMap::new();
        Self {
            positions_of_chunks_in_view,
            chunks,
        }
    }
}
