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

/// Holds both `ChunkPosition` and `LocalPosition`.
pub struct AbsolutePosition {
    pub chunk: ChunkPosition,
    pub local: LocalPosition,
}
