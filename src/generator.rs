use crate::coords::{ChunkPosition, LocalPosition, CHUNK_SIZE};
use crate::entities::entities::Entity;
use crate::graphics::chunk_terrain::ChunkTerrain;
use crate::tile_types::TileType;

use simdnoise::{GradientSettings, NoiseBuilder};

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
                    0..=19 => TileType::StoneFloor,
                    20..=209 => TileType::GrassFloor,
                    210..=255 => TileType::SandFloor,
                };
                tiles_row.push(tile_type);
            }
            positions.push(positions_row);
            tile_types.push(tiles_row);
        }
        (positions, tile_types)
    }

    pub fn generate_entities(
        &self,
        x_offset: f32,
        y_offset: f32,
        chunk_terrain: &ChunkTerrain,
    ) -> Vec<Entity> {
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
                let ground_tile = chunk_terrain.get_tile(&LocalPosition {
                    x: x as i16,
                    y: y as i16,
                });
                let pos = LocalPosition::new(x as i16, y as i16);
                let number = *noise.get((y * CHUNK_SIZE + x) as usize).unwrap() as u8;
                let entity = match ground_tile {
                    TileType::SandFloor => self.sand_entity(number, pos),
                    TileType::GrassFloor => self.grass_entity(number, pos),
                    TileType::StoneFloor => self.stone_entity(number, pos),
                    _ => None,
                };
                if let Some(entity) = entity {
                    entities.push(entity)
                }
            }
        }
        entities
    }

    fn grass_entity(&self, number: u8, pos: LocalPosition) -> Option<Entity> {
        match number {
            221..=225 => Some(Entity::new_local(pos, TileType::MushroomBrown)),
            225..=228 => Some(Entity::new_local(pos, TileType::MushroomOrange)),
            228..=230 => Some(Entity::new_local(pos, TileType::MushroomRed)),
            230..=234 => Some(Entity::new_local(pos, TileType::VioletFlower)),
            234..=239 => Some(Entity::new_local(pos, TileType::WhiteFlower)),
            239..=239 => Some(Entity::new_local(pos, TileType::Pond)),
            239..=241 => Some(Entity::new_local(pos, TileType::TreeStomp)),
            241..=244 => Some(Entity::new_local(pos, TileType::GrassStones)),
            245..=252 => Some(Entity::new_local(pos, TileType::Bush)),
            253..=255 => Some(Entity::new_local(pos, TileType::Coin)),
            _ => None,
        }
    }

    fn sand_entity(&self, number: u8, pos: LocalPosition) -> Option<Entity> {
        match number {
            235..=252 => Some(Entity::new_local(pos, TileType::SandStones)),
            253..=255 => Some(Entity::new_local(pos, TileType::Coin)),
            _ => None,
        }
    }

    fn stone_entity(&self, number: u8, pos: LocalPosition) -> Option<Entity> {
        match number {
            245..=250 => Some(Entity::new_local(pos, TileType::Placeholder)),
            250..=252 => Some(Entity::new_local(pos, TileType::Pond)),
            253..=255 => Some(Entity::new_local(pos, TileType::Coin)),
            _ => None,
        }
    }
}
