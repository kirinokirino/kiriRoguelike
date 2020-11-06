use macroquad::Color;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Into<(i32, i32)> for Position {
    fn into(self) -> (i32, i32) {
        (self.x, self.y)
    }
}

impl From<(i32, i32)> for Position {
    fn from(tuple: (i32, i32)) -> Self {
        Position {
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

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub position: Position,
    pub brightness: Brightness,
}

impl Tile {
    pub fn new(tile_type: TileType, position: Position) -> Self {
        Self {
            tile_type,
            position,
            brightness: Brightness::default(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Brightness(u8);

impl Default for Brightness {
    fn default() -> Self {
        Brightness(255)
    }
}

impl From<u8> for Brightness {
    fn from(val: u8) -> Self {
        Brightness(val)
    }
}

impl From<Brightness> for u8 {
    fn from(val: Brightness) -> Self {
        val.0
    }
}

impl From<Brightness> for Color {
    fn from(val: Brightness) -> Self {
        Color([val.into(), val.into(), val.into(), 255])
    }
}
