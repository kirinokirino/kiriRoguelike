use crate::graphics::layer::{Layer, LAYER_DIMENSIONS};
use crate::graphics::tile::{Tile, TileType};

use std::collections::HashMap;

use simdnoise::NoiseBuilder;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct WorldPosition {
    x: i32,
    y: i32,
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

#[derive(Default)]
pub struct World {
    positions_of_layers_in_view: Vec<Option<WorldPosition>>,
    layers: HashMap<WorldPosition, Layer>,

    generator: Generator,
}

impl World {
    pub fn gen_layer(&mut self, pos: WorldPosition) {
        let x = pos.x * i32::from(LAYER_DIMENSIONS);
        let y = pos.y * i32::from(LAYER_DIMENSIONS);
        self.positions_of_layers_in_view.push(Some(pos));
        let origin = (i64::from(x), i64::from(y));
        self.layers
            .insert(pos, Self::new_layer(&self.generator, origin));
    }
    fn new_layer(gen: &Generator, origin: (i64, i64)) -> Layer {
        let tiles = gen.generate_layer_tiles(origin.0 as f32, origin.1 as f32);
        Layer::new(origin, &tiles)
    }

    pub fn get_layers(&self) -> Vec<&Layer> {
        self.positions_of_layers_in_view
            .iter()
            .filter_map(|p| match p {
                Some(position) => self.layers.get(position),
                None => None,
            })
            .collect()
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
