/// All the available tile types.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TileType {
    Debug,
    WoodenWall,
    GrassFloor,
    Pengu,
    Door,
    Chest,
    Coin,
    Cat,
    StoneFloor,
    Bush,
    GrassStones,
    Pond,
    SandFloor,
    StoneWall,
    StoneEngraving,
    SandStones,
    WhiteFlower,
    MushroomOrange,
    MushroomBrown,
    TreeStomp,
    VioletFlower,
    MushroomRed,
    Placeholder,
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
