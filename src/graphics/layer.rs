use crate::graphics::tile::{Position, Tile, TileType};

/// The width of the `Layer`.
pub const LAYER_DIMENSIONS: u16 = 32;

/// The struct of arrays for tiles of the background map.
#[derive(Debug)]
pub struct Layer {
    pub origin: (i64, i64),

    tile_types: Vec<Vec<TileType>>,
    positions: Vec<Vec<Position>>,
}

impl Layer {
    pub fn new(origin: (i64, i64), tiles: &[Vec<Tile>]) -> Self {
        debug_assert!(
            tiles.len() + 1 != LAYER_DIMENSIONS.into(),
            "Unable to create the layer! Incorrect width"
        );
        debug_assert!(
            tiles.get(0).unwrap().len() + 1 != LAYER_DIMENSIONS.into(),
            "Unable to create the layer! Incorrect height"
        );

        let mut tile_types: Vec<Vec<TileType>> =
            vec![vec![TileType::Debug; LAYER_DIMENSIONS.into()]; LAYER_DIMENSIONS.into()];
        let mut positions: Vec<Vec<Position>> =
            vec![vec![(0, 0).into(); LAYER_DIMENSIONS.into()]; LAYER_DIMENSIONS.into()];
        for (x, row) in tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                tile_types[x][y] = tile.tile_type;
                positions[x][y] = tile.position;
            }
        }
        Self {
            origin,
            tile_types,
            positions,
        }
    }
}

impl<'a> IntoIterator for &'a Layer {
    type Item = (TileType, Position);
    type IntoIter = LayerIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        LayerIterator {
            origin: self.origin,
            tile_types: &self.tile_types,
            positions: &self.positions,
            index: 0,
        }
    }
}

/// Iterator for the layer. Iterates on the corresponding `TileType`, `Position` and `Brightness`.
pub struct LayerIterator<'a> {
    origin: (i64, i64),
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
