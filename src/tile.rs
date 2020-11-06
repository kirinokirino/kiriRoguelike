use macroquad::Color;

/// Coordinates in the game world.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    x: i16,
    y: i16,
}

impl Into<(i16, i16)> for Position {
    fn into(self) -> (i16, i16) {
        (self.x, self.y)
    }
}

impl From<(i16, i16)> for Position {
    fn from(tuple: (i16, i16)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {} ", self.x, self.y)
    }
}

/// Available tiles.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Debug,
    Wall,
    Grass,
    Pengu,
    Door,
    Chest,
    Coin,
    Cat,
}

/// Tile is the visual component of game world entities.
#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: Position,
    pub brightness: Brightness,
}

impl Tile {
    /// Creates a new Tile with maximum brightness.
    pub fn new(tile_type: TileType, position: Position) -> Self {
        Self {
            tile_type,
            position,
            brightness: Brightness::default(),
        }
    }
}

/// A newtype for the brightness level, max is 255.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Brightness(u8);

impl Default for Brightness {
    fn default() -> Self {
        Self(255)
    }
}

impl From<u8> for Brightness {
    fn from(val: u8) -> Self {
        Self(val)
    }
}

impl From<Brightness> for u8 {
    fn from(val: Brightness) -> Self {
        val.0
    }
}

impl From<Brightness> for Color {
    fn from(val: Brightness) -> Self {
        Self([val.into(), val.into(), val.into(), 255])
    }
}
