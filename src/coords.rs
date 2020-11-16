use std::ops::Add;
//                                | What is
//                          TODO: v         this?
use crate::entities::entities::Entity;

pub const CHUNK_SIZE: u16 = 32;

/// Coordinates in the chunk.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LocalPosition {
    pub x: i16,
    pub y: i16,
}

impl LocalPosition {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub fn add_tuple(&mut self, to_add: (i16, i16)) {
        self.x += to_add.0;
        self.y += to_add.1;
    }
}

impl Default for LocalPosition {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::fmt::Display for LocalPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {} ", self.x, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
pub struct ChunkPosition {
    pub x: i32,
    pub y: i32,
}

/// Sets the `ChunkPosition` to a tuple without scaling.
impl ChunkPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
/// Holds both `ChunkPosition` and `LocalPosition`.
pub struct AbsolutePosition {
    pub chunk: ChunkPosition,
    pub local: LocalPosition,
}

impl AbsolutePosition {
    pub fn add_to_local(&self, delta: (i16, i16)) -> Self {
        Entity::get_checked_position(
            self.chunk,
            LocalPosition::new(self.local.x + delta.0, self.local.y + delta.1),
        )
    }
    pub fn new_local(&self, local: LocalPosition) -> Self {
        Entity::get_checked_position(self.chunk, local)
    }
}

impl Into<(f32, f32)> for AbsolutePosition {
    fn into(self) -> (f32, f32) {
        (
            (self.chunk.x * i32::from(CHUNK_SIZE) + i32::from(self.local.x)) as f32,
            (self.chunk.y * i32::from(CHUNK_SIZE) + i32::from(self.local.y)) as f32,
        )
    }
}
