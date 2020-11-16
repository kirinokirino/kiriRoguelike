use crate::coords::{AbsolutePosition, ChunkPosition, LocalPosition, CHUNK_SIZE};
use crate::entities::player::Player;
use crate::generator::Generator;
use crate::graphics::chunk_terrain::ChunkTerrain;
use crate::graphics::tile_atlas::TileAtlas;
use crate::tile_types::TileType;
use crate::world::World;

use std::collections::HashMap;
/// Entities have the capabilites to interact in the game world.
/// Block view, move, block movement, be playable...
#[derive(Debug, Default)]
pub struct Entities {
    pub player: Player,

    entities: Vec<Entity>,

    loaded_locations: Vec<ChunkPosition>,
    entities_store: HashMap<ChunkPosition, Vec<Entity>>,
}

impl Entities {
    pub fn input(&mut self, dest: (i8, i8)) {
        self.player.destination.set_destination(dest.0, dest.1);
    }
    pub fn update(&mut self, world: &World, generator: &Generator) {
        let active_locations = &world.positions_of_chunks_in_view;
        for active_location in active_locations.iter() {
            if !self.loaded_locations.contains(active_location) {
                self.load_entities_at_location(
                    active_location,
                    world
                        .get_chunk(active_location)
                        .expect("This chunk should be available by now!"),
                    generator,
                );
            }
        }

        let mut locations_to_unload: Vec<ChunkPosition> = Vec::new();
        for location in &self.loaded_locations {
            if !active_locations.contains(location) {
                locations_to_unload.push(location.clone());
            }
        }

        for location in locations_to_unload.drain(..) {
            self.unload_entites_from_location(&location);
        }
        self.clean_up();
        if !self.player.destination.is_zero() {
            let future_pos = self.player.calc_future_pos();
            let collider = self
                .entities
                .iter_mut()
                .find(|e| e.pos == future_pos.1 && e.chunk_pos == future_pos.0);

            let allowed_to_move = match collider {
                Some(collider) => {
                    if Entity::is_blocking(&collider).unwrap() {
                        false
                    } else {
                        collider.collide(&mut self.player);
                        true
                    }
                }
                None => true,
            };

            if allowed_to_move {
                self.player
                    .entity
                    .add_to_local_position(self.player.destination.as_tuple().into());
            }
            self.player.destination.reset_destination();
        }
    }

    pub fn draw(&self, tile_atlas: &TileAtlas) {
        let player_pos = self.player.entity.get_absolute_position_f32();
        for entity in self.entities.iter() {
            let entity_pos = entity.get_absolute_position_f32();
            let dist = distance(player_pos, entity_pos);
            if dist < self.player.vision_range.into() {
                let brightness = self.player.calc_brightness(dist);
                tile_atlas.draw_entity(entity, brightness);
            }
        }
        tile_atlas.draw_entity(&self.player.entity, 255.into());
    }

    fn populate_location(
        &mut self,
        location: ChunkPosition,
        terrain: &ChunkTerrain,
        generator: &Generator,
    ) {
        let scaled_x = location.x * i32::from(CHUNK_SIZE);
        let scaled_y = location.y * i32::from(CHUNK_SIZE);
        let entities = generator.generate_entities(scaled_x as f32, scaled_y as f32, terrain);

        for mut entity in entities {
            if entity.tile == TileType::Placeholder {
                // The center of the hut is `entity.pos`
                // We pass this center (and terrain?) to the hut generator and get a
                // vec of entities back.
                for mut hut_e in self.create_hut(entity.pos) {
                    hut_e.set_chunk_position(location.clone());
                    self.entities.push(hut_e);
                }
            } else {
                entity.set_chunk_position(location.clone());
                self.entities.push(entity);
            }
        }
    }

    fn load_entities_at_location(
        &mut self,
        location: &ChunkPosition,
        chunk_terrain: &ChunkTerrain,
        generator: &Generator,
    ) {
        if let Some(entities) = self.entities_store.remove(location) {
            self.load_entities(entities);
        } else {
            self.populate_location(*location, chunk_terrain, generator);
        }
        self.loaded_locations.push(*location);
    }

    fn unload_entites_from_location(&mut self, location: &ChunkPosition) {
        let mut entities_to_store: Vec<Entity> = Vec::new();
        for entity in self.entities.iter() {
            if entity.chunk_pos == *location {
                entities_to_store.push(entity.clone());
            }
        }
        self.entities.retain(|e| e.chunk_pos != *location);
        self.entities_store.insert(*location, entities_to_store);
        self.loaded_locations.retain(|e| e != location);
    }

    fn load_entities(&mut self, mut entities: Vec<Entity>) {
        self.entities.extend(entities.drain(..));
    }

    fn clean_up(&mut self) {
        self.entities.retain(|e| !e.removed);
    }

    //fn clear_the_ground(&mut self, top_left: &AbsolutePosition, down_right: &AbsolutePosition) {}

    pub fn get_mut_entity_at_pos(&mut self, position: &AbsolutePosition) -> Option<&mut Entity> {
        self.entities
            .iter_mut()
            .find(|e| e.pos == position.local && e.chunk_pos == position.chunk)
    }

    pub fn create_hut(&mut self, top_left: LocalPosition) -> Vec<Entity> {
        let size = 5;
        let blueprint = [
            2, 1, 1, 1, 2, 1, 0, 0, 0, 1, 1, 0, 3, 0, 1, 1, 0, 0, 0, 1, 2, 1, 0, 1, 2,
        ];

        let mut res: Vec<Entity> = Vec::with_capacity(19);
        for height in (0..size) {
            for width in (0..size) {
                let mut base_entity = Entity::default();
                let pos = LocalPosition::new(top_left.x + width, top_left.y + height);
                match blueprint[height as usize * size as usize + width as usize] {
                    0 => (),
                    1 => {
                        base_entity.set_tile(TileType::StoneWall);
                        base_entity.set_local_position(pos);
                        res.push(base_entity);
                    }
                    2 => {
                        base_entity.set_tile(TileType::StoneEngraving);
                        base_entity.set_local_position(pos);
                        res.push(base_entity);
                    }
                    3 => {
                        base_entity.set_tile(TileType::Chest);
                        base_entity.set_local_position(pos);
                        res.push(base_entity);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }
        res
    }
}

#[derive(Debug, Clone, Default)]
pub struct Entity {
    pub chunk_pos: ChunkPosition,
    pub pos: LocalPosition,
    pub tile: TileType,
    pub removed: bool,
}

impl Entity {
    pub fn new_local(pos: LocalPosition, tile: TileType) -> Self {
        Self {
            chunk_pos: ChunkPosition::default(),
            pos,
            tile,
            removed: false,
        }
    }

    pub fn set_tile(&mut self, tile: TileType) {
        self.tile = tile;
    }

    pub fn set_local_position(&mut self, pos: LocalPosition) {
        self.pos = pos;
    }

    pub fn add_to_local_position(&mut self, to_add: (i16, i16)) {
        self.pos.add_tuple(to_add);
        self.set_position(Self::get_checked_position(self.chunk_pos, self.pos));
    }

    pub fn get_checked_position(
        chunk_pos: ChunkPosition,
        pos: LocalPosition,
    ) -> (ChunkPosition, LocalPosition) {
        let dimensions = CHUNK_SIZE as i16;
        let LocalPosition { mut x, mut y } = pos;
        let ChunkPosition {
            x: mut world_x,
            y: mut world_y,
        } = chunk_pos;
        if x >= dimensions {
            world_x += 1;
            x -= dimensions
        } else if x < 0 {
            world_x -= 1;
            x += dimensions;
        }
        if y >= dimensions {
            world_y += 1;
            y -= dimensions;
        } else if y < 0 {
            world_y -= 1;
            y += dimensions;
        }
        (ChunkPosition::new(world_x, world_y), LocalPosition { x, y })
    }

    pub fn set_chunk_position(&mut self, chunk_pos: ChunkPosition) {
        self.chunk_pos = chunk_pos;
    }

    pub fn set_position(&mut self, pos: (ChunkPosition, LocalPosition)) {
        self.set_chunk_position(pos.0);
        self.set_local_position(pos.1);
    }

    pub fn get_absolute_position_f32(&self) -> (f32, f32) {
        let LocalPosition { x, y } = self.pos;
        let ChunkPosition {
            x: world_x,
            y: world_y,
        } = self.chunk_pos;
        (
            ((world_x * i32::from(CHUNK_SIZE)) + i32::from(x)) as f32,
            ((world_y * i32::from(CHUNK_SIZE)) + i32::from(y)) as f32,
        )
    }

    pub fn collide(&mut self, player: &mut Player) {
        if Entity::is_pickup(self).unwrap() {
            self.removed = true;
            if self.tile == TileType::Coin {
                player.score += 1;
            } else if self.tile == TileType::Chest {
                player.score += 5;
            }
        }
    }

    pub const fn is_blocking(entity: &Entity) -> Option<bool> {
        match entity.tile {
            TileType::Debug => Some(false),
            TileType::Placeholder => Some(false),
            TileType::WoodenWall => Some(true),
            TileType::GrassFloor => Some(false),
            TileType::Pengu => Some(true),
            TileType::Door => Some(true),
            TileType::Chest => Some(false),
            TileType::Coin => Some(false),
            TileType::Cat => Some(true),
            TileType::StoneFloor => Some(false),
            TileType::Bush => Some(true),
            TileType::GrassStones => Some(false),
            TileType::Pond => Some(true),
            TileType::SandFloor => Some(false),
            TileType::StoneWall => Some(true),
            TileType::StoneEngraving => Some(true),
            TileType::SandStones => Some(false),
            TileType::WhiteFlower => Some(false),
            TileType::MushroomOrange => Some(false),
            TileType::MushroomBrown => Some(false),
            TileType::TreeStomp => Some(true),
            TileType::VioletFlower => Some(false),
            TileType::MushroomRed => Some(false),
        }
    }

    pub const fn is_pickup(entity: &Entity) -> Option<bool> {
        match entity.tile {
            TileType::Chest => Some(true),
            TileType::Coin => Some(true),
            _ => Some(false),
        }
    }
}

impl std::fmt::Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Entity: {}", self.tile)
    }
}

pub fn distance(first: (f32, f32), second: (f32, f32)) -> f32 {
    let (x1, y1) = first;
    let (x2, y2) = second;
    let distance_x = (x1 - x2).abs();
    let distance_y = (y1 - y2).abs();
    distance_x.hypot(distance_y).abs()
}
