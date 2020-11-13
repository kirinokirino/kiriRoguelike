use crate::coords::{ChunkPosition, LocalPosition, CHUNK_SIZE};
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
        chunk_pos: ChunkPosition,
    ) -> (Vec<Vec<LocalPosition>>, Vec<Vec<TileType>>) {
        let noise = NoiseBuilder::gradient_2d_offset(
            (chunk_pos.x * i32::from(CHUNK_SIZE)) as f32,
            CHUNK_SIZE.into(),
            (chunk_pos.y * i32::from(CHUNK_SIZE)) as f32,
            CHUNK_SIZE.into(),
        )
        .with_seed(self.seed)
        .with_freq(0.045)
        .generate_scaled(0.0, 255.0);

        let mut positions: Vec<Vec<LocalPosition>> = Vec::with_capacity(CHUNK_SIZE.into());
        let mut tile_types: Vec<Vec<TileType>> = Vec::with_capacity(CHUNK_SIZE.into());
        for y in 0..CHUNK_SIZE {
            let mut tiles_row: Vec<TileType> = Vec::with_capacity(CHUNK_SIZE.into());
            let mut positions_row: Vec<LocalPosition> = Vec::with_capacity(CHUNK_SIZE.into());
            for x in 0..CHUNK_SIZE {
                positions_row.push(LocalPosition::new(x as i16, y as i16));
                let number = *noise.get((y * CHUNK_SIZE + x) as usize).unwrap() as u8;
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
        let noise = NoiseBuilder::gradient_2d_offset(
            x_offset,
            CHUNK_SIZE.into(),
            y_offset,
            CHUNK_SIZE.into(),
        )
        .with_seed(self.seed)
        .with_freq(0.4)
        .generate_scaled(0.0, 255.0);

        let mut entities = Vec::new();
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let number = *noise.get((y * CHUNK_SIZE + x) as usize).unwrap() as u8;
                let pos = LocalPosition::new(x as i16, y as i16);
                match number {
                    220..=222 => {
                        entities.push(Entity::new_local(pos, TileType::Pond));
                    }
                    223..=244 => {
                        entities.push(Entity::new_local(pos, TileType::Stones));
                    }
                    245..=252 => {
                        entities.push(Entity::new_local(pos, TileType::Bush));
                    }
                    253..=255 => {
                        entities.push(Entity::new_local(pos, TileType::Coin));
                    }
                    _ => {}
                }
            }
        }

        entities
    }
}
