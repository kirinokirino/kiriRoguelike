use crate::graphics::tile::{Position, TileType};

/// The width of the `Layer`.
pub const LAYER_DIMENSIONS: u16 = 32;

/// The struct of arrays for tiles of the background map.
#[derive(Debug)]
pub struct Layer {
    pub origin: (i64, i64),

    positions: Vec<Vec<Position>>,
    tile_types: Vec<Vec<TileType>>,
}

impl Layer {
    pub fn new(
        origin: (i64, i64),
        tile_types: Vec<Vec<(TileType)>>,
        positions: Vec<Vec<(Position)>>,
    ) -> Self {
        Self {
            origin,
            positions,
            tile_types,
        }
    }
    pub fn get_tile(&self, pos: &Position) -> &TileType {
        self.tile_types
            .get(pos.y as usize)
            .expect("Tried to get a tile outside the layer!")
            .get(pos.x as usize)
            .expect("Tried to get a tile outside the layer!")
    }
}

impl<'a> IntoIterator for &'a Layer {
    type Item = (TileType, Position);
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
    positions: &'a Vec<Vec<Position>>,
    index: usize,
}

impl<'a> Iterator for LayerIterator<'a> {
    type Item = (TileType, Position);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= (LAYER_DIMENSIONS * LAYER_DIMENSIONS).into() {
            None
        } else {
            let (x, y) = (
                self.index / usize::from(LAYER_DIMENSIONS),
                self.index % usize::from(LAYER_DIMENSIONS),
            );

            let tile_type = self.tile_types[x][y];
            let position = self.positions[x][y];

            self.index += 1;
            Some((tile_type, position))
        }
    }
}
