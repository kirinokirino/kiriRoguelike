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
