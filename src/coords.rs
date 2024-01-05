pub const CHUNK_SIZE: u16 = 32;

/// Coordinates in the chunk.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
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
        get_checked_position(
            self.chunk,
            LocalPosition::new(self.local.x + delta.0, self.local.y + delta.1),
        )
    }
    pub fn new_local(&self, local: LocalPosition) -> Self {
        get_checked_position(self.chunk, local)
    }
    pub fn get_absolute_position_f32(&self) -> (f32, f32) {
        let LocalPosition { x, y } = self.local;
        let ChunkPosition {
            x: world_x,
            y: world_y,
        } = self.chunk;
        (
            ((world_x * i32::from(CHUNK_SIZE)) + i32::from(x)) as f32,
            ((world_y * i32::from(CHUNK_SIZE)) + i32::from(y)) as f32,
        )
    }
}

impl From<AbsolutePosition> for (f32, f32) {
    fn from(val: AbsolutePosition) -> Self {
        (
            (val.chunk.x * i32::from(CHUNK_SIZE) + i32::from(val.local.x)) as f32,
            (val.chunk.y * i32::from(CHUNK_SIZE) + i32::from(val.local.y)) as f32,
        )
    }
}

pub fn distance(first: (f32, f32), second: (f32, f32)) -> f32 {
    let (x1, y1) = first;
    let (x2, y2) = second;
    let distance_x = (x1 - x2).abs();
    let distance_y = (y1 - y2).abs();
    distance_x.hypot(distance_y).abs()
}

pub fn get_checked_position(chunk_pos: ChunkPosition, pos: LocalPosition) -> AbsolutePosition {
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
    AbsolutePosition {
        local: LocalPosition { x, y },
        chunk: ChunkPosition::new(world_x, world_y),
    }
}
