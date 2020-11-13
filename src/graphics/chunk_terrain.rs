use crate::coords::{ChunkPosition, LocalPosition, CHUNK_SIZE};
use crate::tile_types::TileType;

/// The background terrain for the chunk.
#[derive(Debug)]
pub struct ChunkTerrain {
    pub origin: (i64, i64),

    positions: Vec<Vec<LocalPosition>>,
    tile_types: Vec<Vec<TileType>>,
}

impl ChunkTerrain {
    pub fn new(
        origin: (i64, i64),
        tile_types: Vec<Vec<(TileType)>>,
        positions: Vec<Vec<(LocalPosition)>>,
    ) -> Self {
        Self {
            origin,
            positions,
            tile_types,
        }
    }
    pub fn get_tile(&self, pos: &LocalPosition) -> &TileType {
        self.tile_types
            .get(pos.y as usize)
            .expect("Tried to get a tile outside the layer!")
            .get(pos.x as usize)
            .expect("Tried to get a tile outside the layer!")
    }
}

impl<'a> IntoIterator for &'a ChunkTerrain {
    type Item = (TileType, LocalPosition);
    type IntoIter = LayerIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        LayerIterator {
            tile_types: &self.tile_types,
            positions: &self.positions,
            index: 0,
        }
    }
}

/// Iterator for the layer. Iterates on the corresponding `TileType`, `Position` and `Brightness`.
pub struct LayerIterator<'a> {
    tile_types: &'a Vec<Vec<TileType>>,
    positions: &'a Vec<Vec<LocalPosition>>,
    index: usize,
}

impl<'a> Iterator for LayerIterator<'a> {
    type Item = (TileType, LocalPosition);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= (CHUNK_SIZE * CHUNK_SIZE).into() {
            None
        } else {
            let (x, y) = (
                self.index / usize::from(CHUNK_SIZE),
                self.index % usize::from(CHUNK_SIZE),
            );

            let tile_type = self.tile_types[x][y];
            let position = self.positions[x][y];

            self.index += 1;
            Some((tile_type, position))
        }
    }
}
