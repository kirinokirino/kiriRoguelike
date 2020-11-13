pub const CHUNK_SIZE: u16 = 32;

/// Coordinates in the chunk.
/// x: `i16`,
/// y: `i16`,
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LocalPosition {
    pub x: i16,
    pub y: i16,
}

impl LocalPosition {
    pub fn add_tuple(&mut self, to_add: (i16, i16)) {
        self.x += to_add.0;
        self.y += to_add.1;
    }
}

impl Into<(i16, i16)> for LocalPosition {
    fn into(self) -> (i16, i16) {
        (self.x, self.y)
    }
}

impl From<(i16, i16)> for LocalPosition {
    fn from(tuple: (i16, i16)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
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

/// Sets the world position to a tuple without scaling.
impl ChunkPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Into<(i32, i32)> for ChunkPosition {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(i32, i32)> for ChunkPosition {
    fn from(pos: (i32, i32)) -> Self {
        Self {
            x: pos.0 / i32::from(CHUNK_SIZE),
            y: pos.1 / i32::from(CHUNK_SIZE),
        }
    }
}

impl From<(u16, u16)> for ChunkPosition {
    fn from(pos: (u16, u16)) -> Self {
        Self {
            x: i32::from(pos.0 / CHUNK_SIZE),
            y: i32::from(pos.1 / CHUNK_SIZE),
        }
    }
}
