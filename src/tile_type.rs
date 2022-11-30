use std::convert::From;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    DeepWater,
    Dirt,
    Grass,
    Forest,
    Rock,
    Sand,
    Savannah,
    ShallowWater,
    Shore,
    Snow,
    Mountain,
    None,
}

impl From<u32> for TileType {
    fn from(tile_type: u32) -> Self {
        match tile_type {
            0 => TileType::DeepWater,
            1 => TileType::Dirt,
            2 => TileType::Grass,
            3 => TileType::Forest,
            4 => TileType::Rock,
            5 => TileType::Sand,
            6 => TileType::Savannah,
            7 => TileType::ShallowWater,
            8 => TileType::Shore,
            9 => TileType::Snow,
            10 => TileType::Mountain,
            _ => TileType::None,
        }
    }
}

pub fn tile_walkable(tile_index: u32) -> bool {
    let tile_type = TileType::from(tile_index);
    !matches!(tile_type, TileType::DeepWater | TileType::Rock | TileType::ShallowWater | TileType::Mountain | TileType::None)
}

