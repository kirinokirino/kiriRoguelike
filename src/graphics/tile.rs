use macroquad::Color;
/// Coordinates in the game world, relative to the nearest layer origin.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: i16,
    pub y: i16,
}

impl Position {
    pub fn add_tuple(&mut self, to_add: (i16, i16)) {
        self.x += to_add.0;
        self.y += to_add.1;
    }
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

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
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
    GrassFloor,
    Pengu,
    Door,
    Chest,
    Coin,
    Cat,
    StoneFloor,
    Bush,
    Stones,
    Pond,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Debug
    }
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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
        Self([255, 255, 255, val.into()])
    }
}
