

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

// impl From<&u32> for TileType {
//     fn from(tile_type: &str) -> Self {
//         match tile_type {
//             "deep_water.png" => TileType::DeepWater,
//             "dirt.png" => TileType::Dirt,
//             "grass.png" => TileType::Grass,
//             "forest.png" => TileType::Forest,
//             "rock.png" => TileType::Rock,
//             "sand.png" => TileType::Sand,
//             "savannah.png" => TileType::Savannah,
//             "shallow_water.png" => TileType::ShallowWater,
//             "shore.png" => TileType::Shore,
//             "snow.png" => TileType::Snow,
//             "mountain.png" => TileType::Mountain,
//             _ => TileType::None,
//         }
//     }
// }