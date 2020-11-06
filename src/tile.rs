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
    pub brightness: u8,
}

impl Tile {
    pub fn new(tile_type: TileType, position: Position) -> Self {
        Self {
            tile_type,
            position,
            brightness: 255,
        }
    }
}
