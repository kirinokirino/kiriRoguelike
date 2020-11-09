use crate::entities::entities::Entity;
use crate::graphics::tile::TileType;

#[derive(Debug, Clone)]
pub struct Player {
    pub entity: Entity,
    pub destination: Destination,
}

impl Default for Player {
    fn default() -> Self {
        let mut entity = Entity::default();
        entity.set_tile(TileType::Pengu);
        Self {
            entity,
            destination: Destination::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Destination {
    x: i8,
    y: i8,
}

impl Destination {
    pub fn set_destination(&mut self, right: i8, up: i8) {
        self.x += right;
        self.y += up;
    }

    pub fn reset_destination(&mut self) {
        self.x = 0;
        self.y = 0;
    }

    pub fn as_tuple(&self) -> (i16, i16) {
        (self.x.into(), self.y.into())
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}
