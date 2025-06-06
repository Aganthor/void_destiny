use std::convert::From;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    DeepWater = 0,
    Dirt = 1,    
    Forest = 2,
    Grass = 3,
    Mountain = 4,
    Rock = 5,
    Sand = 6,
    Savannah = 7,
    ShallowWater = 8,
    Shore = 9,
    Snow = 10,
    None = 11,
}

impl From<u32> for TileType {
    fn from(tile_type: u32) -> Self {
        match tile_type {
            0 => TileType::DeepWater,
            1 => TileType::Dirt,
            2 => TileType::Forest,            
            3 => TileType::Grass,
            4 => TileType::Mountain,
            5 => TileType::Rock,
            6 => TileType::Sand,
            7 => TileType::Savannah,
            8 => TileType::ShallowWater,
            9 => TileType::Shore,
            10 => TileType::Snow,
            _ => TileType::None,
        }
    }
}

pub fn tile_walkable(tile_index: u32) -> bool {
    let tile_type = TileType::from(tile_index);
    !matches!(tile_type, TileType::DeepWater | TileType::Rock | TileType::ShallowWater | TileType::Mountain | TileType::None)
}

