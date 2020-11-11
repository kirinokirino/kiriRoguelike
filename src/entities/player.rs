use crate::entities::entities::Entity;
use crate::graphics::tile::{Brightness, TileType};

#[derive(Debug, Clone)]
pub struct Player {
    pub entity: Entity,
    pub destination: Destination,
    pub vision_range: i16,
}

impl Player {
    pub fn calc_brightness(&self, distance: f32) -> Brightness {
        if distance > self.vision_range.into() {
            Brightness::from(0)
        } else {
            Brightness::from((255. - 255. / (f32::from(self.vision_range) / distance)) as u8)
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        let mut entity = Entity::default();
        entity.set_tile(TileType::Pengu);
        Self {
            entity,
            destination: Destination::default(),
            vision_range: 16,
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
