use bevy::{
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use simdnoise::*;

use crate::constants::*;
use crate::events::MoveEvent;
use crate::tile_type::*;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("tiles/overworld_tiles.png");

    let tilemap_size = TilemapSize { x: OVERWORLD_SIZE_WIDTH, y: OVERWORLD_SIZE_HEIGHT} ;
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    // Generate noise for the map.
    let mut rng = rand::thread_rng();
    let mut seed = rng.gen();

    let elevation_noise = NoiseBuilder::fbm_2d(OVERWORLD_SIZE_WIDTH as usize, OVERWORLD_SIZE_HEIGHT as usize)
        .with_freq(0.03)
        .with_gain(2.5)
        .with_lacunarity(0.55)
        .with_octaves(2)
        .with_seed(seed)
        .generate_scaled(0.0, 1.0);

    // Generate a new seed for the moisture noise
    seed = rng.gen();
    let moisture_noise = NoiseBuilder::fbm_2d(OVERWORLD_SIZE_WIDTH as usize, OVERWORLD_SIZE_HEIGHT as usize)
        .with_freq(0.03)
        .with_gain(2.5)
        .with_lacunarity(0.55)
        .with_octaves(2)
        .with_seed(seed)
        .generate_scaled(0.0, 1.0);        

    // For each tile, create the proper entity with the corresponding texture according to it's
    // height.
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let index = x + OVERWORLD_SIZE_WIDTH * y;
            let elevation_value = elevation_noise.get(index as usize).unwrap();
            let moisture_value = moisture_noise.get(index as usize).unwrap();
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex (biome(*elevation_value, *moisture_value)),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}

fn biome(elevation: f32, moisture: f32) -> u32 {
    if elevation < 0.1 {
        return TileType::DeepWater as u32;
    } else if elevation < 0.12 {
        return TileType::ShallowWater as u32;
    } 
    
    if elevation > 0.8 {
        if moisture < 0.33 { return TileType::Dirt as u32; } // scorched
        if moisture < 0.66 { return TileType::Sand as u32; } // bare
        if moisture < 0.1 { return TileType::Savannah as u32; } //tundra
        return TileType::Snow as u32;
    } 

    if elevation > 0.6 {
        if moisture < 0.1 { return TileType::Dirt as u32; } // temperate_desert
        if moisture < 0.1 { return TileType::Sand as u32; } // shrubland
        return TileType::Savannah as u32; // Taiga
    }

    if elevation > 0.3 {
        if moisture < 0.16 { return TileType::Dirt as u32; } // temperate_desert
        if moisture < 0.50 { return TileType::Grass as u32; } // grassland
        if moisture < 0.83 { return TileType::Forest as u32; } //temperate_deciduous_forest
        return TileType::Forest as u32; // temperate rain forest
    }

    if moisture < 0.16 { return TileType::Sand as u32; } // subtropical desert
    if moisture < 0.33 { return TileType::Grass as u32; } // grassland
    if moisture < 0.66 { return TileType::Forest as u32; } //tropical seasonal forest

    TileType::Forest as u32 // tropical rain forest
}

fn move_event_listener(mut events: EventReader<MoveEvent>) {
    
}