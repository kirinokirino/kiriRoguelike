use crate::layer::{Layer, LAYER_HEIGHT, LAYER_WIDTH};
use crate::tile::{Tile, TileType};

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
            x: pos.0 / i32::from(LAYER_WIDTH),
            y: pos.1 / i32::from(LAYER_HEIGHT),
        }
    }
}

#[derive(Default)]
pub struct World {
    layer_positions: Vec<Option<WorldPosition>>,
    layers: HashMap<WorldPosition, Layer>,

    generator: Generator,
}

impl World {
    pub fn gen_layer(&mut self, pos: WorldPosition) {
        let x = pos.x * i32::from(LAYER_WIDTH);
        let y = pos.y * i32::from(LAYER_HEIGHT);
        let world_position = WorldPosition { x, y };
        self.layer_positions.push(Some(world_position));
        let origin = (i64::from(x), i64::from(y));
        self.layers
            .insert(world_position, Self::new_layer(&self.generator, origin));
    }
    fn new_layer(gen: &Generator, origin: (i64, i64)) -> Layer {
        let tiles = gen.generate_layer_tiles(origin.0 as f32, origin.1 as f32);
        Layer::new(origin, &tiles)
    }

    pub fn get_layers(&self) -> Vec<&Layer> {
        self.layer_positions
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

        let mut tiles = vec![vec![tile.clone(); LAYER_WIDTH.into()]; LAYER_HEIGHT.into()];

        for y in 0..LAYER_HEIGHT.into() {
            for x in 0..LAYER_WIDTH.into() {
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
