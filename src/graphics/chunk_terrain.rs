use crate::coords::{AbsolutePosition, ChunkPosition, LocalPosition, CHUNK_SIZE};
use crate::tile_types::TileType;

/// The background terrain for the chunk.
#[derive(Debug)]
pub struct ChunkTerrain {
    chunk: ChunkPosition,

    positions: Vec<Vec<LocalPosition>>,
    tile_types: Vec<Vec<TileType>>,
}

impl ChunkTerrain {
    pub fn new(
        chunk: ChunkPosition,
        tile_types: Vec<Vec<TileType>>,
        positions: Vec<Vec<LocalPosition>>,
    ) -> Self {
        Self {
            chunk,
            positions,
            tile_types,
        }
    }
    pub fn get_tile(&self, pos: &LocalPosition) -> &TileType {
        self.tile_types
            .get(pos.y as usize)
            .expect("Tried to get a tile outside the chunk!")
            .get(pos.x as usize)
            .expect("Tried to get a tile outside the chunk!")
    }
}

impl<'a> IntoIterator for &'a ChunkTerrain {
    type Item = (TileType, AbsolutePosition);
    type IntoIter = ChunkIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        ChunkIterator {
            chunk: self.chunk.clone(),
            tile_types: &self.tile_types,
            positions: &self.positions,
            index: 0,
        }
    }
}

/// Iterator for the chunk. Iterates on the corresponding `TileType`, `Position` and `Brightness`.
pub struct ChunkIterator<'a> {
    chunk: ChunkPosition,
    tile_types: &'a Vec<Vec<TileType>>,
    positions: &'a Vec<Vec<LocalPosition>>,
    index: usize,
}

impl<'a> Iterator for ChunkIterator<'a> {
    type Item = (TileType, AbsolutePosition);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= (CHUNK_SIZE * CHUNK_SIZE).into() {
            None
        } else {
            let (x, y) = (
                self.index / usize::from(CHUNK_SIZE),
                self.index % usize::from(CHUNK_SIZE),
            );

            let tile_type = self.tile_types[x][y];
            let position = AbsolutePosition {
                chunk: self.chunk,
                local: self.positions[x][y],
            };

            self.index += 1;
            Some((tile_type, position))
        }
    }
}
