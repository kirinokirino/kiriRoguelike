use crate::entities::entities::Entity;
use crate::graphics::tile::TileType;

#[derive(Debug, Clone)]
pub struct Player {
    pub entity: Entity,
}

impl Default for Player {
    fn default() -> Self {
        let mut entity = Entity::default();
        entity.set_tile(TileType::Pengu);
        Self { entity }
    }
}
