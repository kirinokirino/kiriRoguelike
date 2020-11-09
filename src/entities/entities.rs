use crate::graphics::layer::LAYER_DIMENSIONS;
use crate::graphics::tile::{Position, Tile, TileType};
use crate::graphics::tile_atlas::TileAtlas;
use crate::world::{World, WorldPosition};

use crate::entities::player::Player;

use std::collections::HashMap;
/// Entities have the capabilites to interact in the game world.
/// Block view, move, block movement, be playable...
#[derive(Debug, Default)]
pub struct Entities {
    pub player: Player,

    loaded_locations: Vec<WorldPosition>,
    entities_store: HashMap<WorldPosition, Vec<Entity>>,
}

impl Entities {
    fn update(&mut self, world: &World) {
        let active_locations = &world.positions_of_layers_in_view;
        for active_location in active_locations.iter() {
            if self.loaded_locations.contains(active_location) {
                // Update
            } else {
                self.load_nearby_entities(&active_locations);
            }
        }

        let mut locations_to_unload: Vec<WorldPosition> = Vec::new();
        for location in &self.loaded_locations {
            if !active_locations.contains(location) {
                locations_to_unload.push(location.clone());
            }
        }

        for location in locations_to_unload.drain(..) {
            self.unload_entites_from(&location);
        }
    }

    pub fn draw(&self, tile_atlas: &TileAtlas) {
        tile_atlas.draw_entity(&self.player.entity);
    }

    fn load_nearby_entities(&mut self, locations_to_load: &Vec<WorldPosition>) {
        self.loaded_locations.clear();
        let mut to_generate: Vec<&WorldPosition> = Vec::new();
        for location in locations_to_load {
            if let Some(entities) = self.entities_store.remove(location) {
                self.load_entities(entities);
                self.loaded_locations.push(location.clone());
            } else {
                to_generate.push(location);
            }
        }

        for location in to_generate.drain(..) {
            self.populate_location(location.clone());
            self.loaded_locations.push(*location);
        }
    }
    fn populate_location(&mut self, location: WorldPosition) {}
    fn unload_entites_from(&mut self, location: &WorldPosition) {}
    fn load_entities(&mut self, entities: Vec<Entity>) {
        /*
        for entity in entities {
            self.positions.push(entity.pos.clone());
        }
        */
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
