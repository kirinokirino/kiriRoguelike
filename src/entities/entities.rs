use crate::graphics::layer::LAYER_DIMENSIONS;
use crate::graphics::tile::{Brightness, Position, Tile, TileType};
use crate::graphics::tile_atlas::TileAtlas;
use crate::world::{Generator, World, WorldPosition};

use crate::entities::player::Player;

use std::collections::HashMap;
/// Entities have the capabilites to interact in the game world.
/// Block view, move, block movement, be playable...
#[derive(Debug, Default)]
pub struct Entities {
    pub player: Player,

    entities: Vec<Entity>,

    loaded_locations: Vec<WorldPosition>,
    entities_store: HashMap<WorldPosition, Vec<Entity>>,
}

impl Entities {
    pub fn input(&mut self, dest: (i8, i8)) {
        self.player.destination.set_destination(dest.0, dest.1);
    }
    pub fn update(&mut self, world: &World, generator: &Generator) {
        let active_locations = &world.positions_of_layers_in_view;
        for active_location in active_locations.iter() {
            if self.loaded_locations.contains(active_location) {
                // Update
            } else {
                self.load_entities_at_location(active_location, generator);
            }
        }

        let mut locations_to_unload: Vec<WorldPosition> = Vec::new();
        for location in &self.loaded_locations {
            if !active_locations.contains(location) {
                locations_to_unload.push(location.clone());
            }
        }

        for location in locations_to_unload.drain(..) {
            self.unload_entites_from_location(&location);
        }
        if !self.player.destination.is_zero() {
            self.player
                .entity
                .add_to_local_position(self.player.destination.as_tuple().into());
            self.player.destination.reset_destination();
        }
    }

    pub fn draw(&self, tile_atlas: &TileAtlas) {
        let player_pos = self.player.entity.get_absolute_position();
        for entity in self.entities.iter() {
            let entity_pos = entity.get_absolute_position();
            let dist = distance(player_pos, entity_pos);
            if dist < self.player.vision_range.into() {
                let brightness = self.player.calc_brightness(dist);
                tile_atlas.draw_entity(entity, brightness);
            }
        }
        tile_atlas.draw_entity(&self.player.entity, 255.into());
    }

    fn populate_location(&mut self, location: WorldPosition, generator: &Generator) {
        let (x, y) = location.into();
        let scaled_x = x * i32::from(LAYER_DIMENSIONS);
        let scaled_y = y * i32::from(LAYER_DIMENSIONS);
        let mut entities = generator.generate_entities(scaled_x as f32, scaled_y as f32);

        for mut entity in entities {
            entity.set_world_position(location.clone());
            self.entities.push(entity);
        }
    }

    fn load_entities_at_location(&mut self, location: &WorldPosition, generator: &Generator) {
        if let Some(entities) = self.entities_store.remove(location) {
            self.load_entities(entities);
        } else {
            self.populate_location(*location, generator);
        }
        self.loaded_locations.push(*location);
    }

    fn unload_entites_from_location(&mut self, location: &WorldPosition) {
        let mut entities_to_store: Vec<Entity> = Vec::new();
        for entity in self.entities.iter() {
            if entity.world_pos == *location {
                entities_to_store.push(entity.clone());
            }
        }
        self.entities.retain(|e| e.world_pos != *location);
        self.entities_store.insert(*location, entities_to_store);
        self.loaded_locations.retain(|e| e != location);
    }

    fn load_entities(&mut self, mut entities: Vec<Entity>) {
        self.entities.extend(entities.drain(..));
    }
}

#[derive(Debug, Clone, Default)]
pub struct Entity {
    pub world_pos: WorldPosition,
    pub pos: Position,
    pub tile: TileType,
}

impl Entity {
    fn new(world_pos: WorldPosition, pos: Position, tile: TileType) -> Self {
        Self {
            world_pos,
            pos,
            tile,
        }
    }

    pub fn set_tile(&mut self, tile: TileType) {
        self.tile = tile;
    }

    pub fn set_local_position(&mut self, pos: Position) {
        self.pos = pos;
    }

    pub fn add_to_local_position(&mut self, to_add: (i16, i16)) {
        self.pos.add_tuple(to_add);
        self.check_world_position();
    }

    pub fn check_world_position(&mut self) {
        let dimensions = LAYER_DIMENSIONS as i16;
        let (mut x, mut y) = self.pos.into();
        let (mut world_x, mut world_y) = self.world_pos.into();
        let mut has_change = false;
        if x >= dimensions {
            world_x += 1;
            x -= dimensions;
            has_change = true;
        } else if x < 0 {
            world_x -= 1;
            x += dimensions;
            has_change = true;
        }
        if y >= dimensions {
            world_y += 1;
            y -= dimensions;
            has_change = true;
        } else if y < 0 {
            world_y -= 1;
            y += dimensions;
            has_change = true;
        }

        if has_change {
            self.set_local_position((x, y).into());
            self.set_world_position(WorldPosition::new(world_x, world_y));
        }
    }

    pub fn set_world_position(&mut self, world_pos: WorldPosition) {
        self.world_pos = world_pos;
    }

    pub fn get_absolute_position(&self) -> (f32, f32) {
        let (local_x, local_y) = self.pos.into();
        let (world_x, world_y) = self.world_pos.into();
        (
            ((world_x * i32::from(LAYER_DIMENSIONS)) + i32::from(local_x)) as f32,
            ((world_y * i32::from(LAYER_DIMENSIONS)) + i32::from(local_y)) as f32,
        )
    }
}
struct Opaque(Position);

pub fn distance(first: (f32, f32), second: (f32, f32)) -> f32 {
    let (x1, y1) = first;
    let (x2, y2) = second;
    let distance_x = (x1 - x2).abs();
    let distance_y = (y1 - y2).abs();
    distance_x.hypot(distance_y).abs()
}
