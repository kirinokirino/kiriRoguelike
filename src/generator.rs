use crate::coords::{LocalPosition, CHUNK_SIZE};
use crate::entities::entities::Entity;
use crate::tile_types::TileType;

use simdnoise::NoiseBuilder;

#[derive(Default)]
pub struct Generator {
    seed: i32,
}

impl Generator {
    pub fn new(seed: i32) -> Self {
        Self { seed }
    }

    pub fn generate_chunk_terrain(
        &self,
        x_offset: f32,
        y_offset: f32,
    ) -> (Vec<Vec<LocalPosition>>, Vec<Vec<TileType>>) {
        let noise = NoiseBuilder::gradient_2d_offset(x_offset, 64, y_offset, 64)
            .with_seed(self.seed)
            .with_freq(0.045)
            .generate_scaled(0.0, 255.0);

        let mut positions: Vec<Vec<LocalPosition>> = Vec::with_capacity(CHUNK_SIZE.into());
        let mut tile_types: Vec<Vec<TileType>> = Vec::with_capacity(CHUNK_SIZE.into());
        for y in 0..CHUNK_SIZE.into() {
            let mut tiles_row: Vec<TileType> = Vec::with_capacity(CHUNK_SIZE.into());
            let mut positions_row: Vec<LocalPosition> = Vec::with_capacity(CHUNK_SIZE.into());
            for x in 0..CHUNK_SIZE.into() {
                positions_row.push((x as i16, y as i16).into());
                let number = *noise.get(y * 64 + x).unwrap() as u8;
                let tile_type = match number {
                    0..=209 => TileType::GrassFloor,
                    210..=255 => TileType::StoneFloor,
                };
                tiles_row.push(tile_type);
            }
            positions.push(positions_row);
            tile_types.push(tiles_row);
        }
        (positions, tile_types)
    }

    pub fn generate_entities(&self, x_offset: f32, y_offset: f32) -> Vec<Entity> {
        let noise = NoiseBuilder::gradient_2d_offset(x_offset, 64, y_offset, 64)
            .with_seed(self.seed)
            .with_freq(0.4)
            .generate_scaled(0.0, 255.0);

        let entity_base = Entity {
            world_pos: (0, 0).into(),
            pos: (0, 0).into(),
            tile: TileType::Debug,
            removed: false,
        };

        let mut entities = Vec::new();

        for y in 0..CHUNK_SIZE.into() {
            for x in 0..CHUNK_SIZE.into() {
                let number = *noise.get(y * 64 + x).unwrap() as u8;
                match number {
                    220..=222 => {
                        let mut entity = entity_base.clone();
                        entity.set_tile(TileType::Pond);
                        entity.set_local_position((x as i16, y as i16).into());
                        entities.push(entity);
                    }
                    223..=244 => {
                        let mut entity = entity_base.clone();
                        entity.set_tile(TileType::Stones);
                        entity.set_local_position((x as i16, y as i16).into());
                        entities.push(entity);
                    }
                    245..=252 => {
                        let mut entity = entity_base.clone();
                        entity.set_tile(TileType::Bush);
                        entity.set_local_position((x as i16, y as i16).into());
                        entities.push(entity);
                    }
                    253..=255 => {
                        let mut entity = entity_base.clone();
                        entity.set_tile(TileType::Coin);
                        entity.set_local_position((x as i16, y as i16).into());
                        entities.push(entity);
                    }
                    _ => {}
                }
            }
        }

        entities
    }
}
