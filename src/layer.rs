use crate::tile::{Brightness, Position, Tile, TileType};

/// The width of the `Layer`.
pub const LAYER_WIDTH: u16 = 64;
/// The height of the `Layer`.
pub const LAYER_HEIGHT: u16 = 64;

/// The struct of arrays for tiles of the background map.
#[derive(Debug)]
pub struct Layer {
    pub origin: (i64, i64),

    tile_types: Vec<Vec<TileType>>,
    positions: Vec<Vec<Position>>,
    brightnesses: Vec<Vec<Brightness>>,
}

impl Layer {
    pub fn new(origin: (i64, i64), tiles: &[Vec<Tile>]) -> Self {
        debug_assert!(
            tiles.len() + 1 != LAYER_WIDTH.into(),
            "Unable to create the layer! Incorrect width"
        );
        debug_assert!(
            tiles.get(0).unwrap().len() + 1 != LAYER_HEIGHT.into(),
            "Unable to create the layer! Incorrect height"
        );

        let mut tile_types: Vec<Vec<TileType>> =
            vec![vec![TileType::Debug; LAYER_HEIGHT.into()]; LAYER_WIDTH.into()];
        let mut positions: Vec<Vec<Position>> =
            vec![vec![(0, 0).into(); LAYER_HEIGHT.into()]; LAYER_WIDTH.into()];
        let mut brightnesses: Vec<Vec<Brightness>> =
            vec![vec![Brightness::from(0_u8); LAYER_HEIGHT.into()]; LAYER_WIDTH.into()];
        for (x, row) in tiles.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                tile_types[x][y] = tile.tile_type;
                positions[x][y] = tile.position;
                brightnesses[x][y] = tile.brightness;
            }
        }
        Self {
            origin,
            tile_types,
            positions,
            brightnesses,
        }
    }
}

impl<'a> IntoIterator for &'a Layer {
    type Item = (TileType, Position, Brightness);
    type IntoIter = LayerIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        LayerIterator {
            origin: self.origin,
            tile_types: &self.tile_types,
            positions: &self.positions,
            brightnesses: &self.brightnesses,
            index: 0,
        }
    }
}

/// Iterator for the layer. Iterates on the corresponding `TileType`, `Position` and `Brightness`.
pub struct LayerIterator<'a> {
    origin: (i64, i64),
    tile_types: &'a Vec<Vec<TileType>>,
    positions: &'a Vec<Vec<Position>>,
    brightnesses: &'a Vec<Vec<Brightness>>,
    index: usize,
}

impl<'a> Iterator for LayerIterator<'a> {
    type Item = (TileType, Position, Brightness);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= (LAYER_WIDTH * LAYER_HEIGHT).into() {
            None
        } else {
            let (x, y) = (
                self.index / usize::from(LAYER_WIDTH),
                self.index % usize::from(LAYER_WIDTH),
            );

            let tile_type = self.tile_types[x][y];
            let position = self.positions[x][y];
            let brightness = self.brightnesses[x][y];

            self.index += 1;
            Some((tile_type, position, brightness))
        }
    }
}
