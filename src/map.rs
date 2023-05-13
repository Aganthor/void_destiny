//#[]

use bevy::{math::Vec4Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use simdnoise::*;
//use bevy_inspector_egui::Inspectable;

use crate::constants::*;
use crate::events::{MoveEvent, MoveLegal};
use crate::tile_type::*;

// #[derive(Component, Inspectable)]
// pub struct TileCollider;


//#[derive(Resource, Inspectable)]
#[derive(Resource)]
pub struct MapSeed {
    map_elevation_seed: i32,
    map_moisture_seed: i32,
}

impl Default for MapSeed {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        MapSeed { 
            map_elevation_seed: rng.gen(),
            map_moisture_seed: rng.gen(),
        }
    }
}

// #[derive(Component, Inspectable)]
// struct NoiseSettings {
//     frequency: f32,
//     gain: f32,
//     lacunarity: f32,
//     octaves: f32,
// }

// impl NoiseSettings {
//     fn new() -> Self {
//         NoiseSettings { frequency: (), gain: (), lacunarity: (), octaves: () }
//     }
// }

// #[derive(Bundle, Inspectable)]
// struct Map {
//     ecs_map: TilemapBundle,
//     elevation_noise: NoiseSettings,
//     moisture_noise: NoiseSettings,
//     seeds: MapSeed,
// }


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .init_resource::<MapSeed>()
            .add_startup_system(setup)
            .add_system(move_event_listener);
    }
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    map_seed: Res<MapSeed>,
) {
    let texture_handle = asset_server.load("tiles/overworld_tiles.png");

    let tilemap_size = TilemapSize {
        x: OVERWORLD_SIZE_WIDTH,
        y: OVERWORLD_SIZE_HEIGHT,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let elevation_noise = NoiseBuilder::fbm_2d(
        OVERWORLD_SIZE_WIDTH as usize,
        OVERWORLD_SIZE_HEIGHT as usize,
    )
    .with_freq(0.03)
    .with_gain(2.5)
    .with_lacunarity(0.55)
    .with_octaves(2)
    .with_seed(map_seed.map_elevation_seed)
    .generate_scaled(0.0, 1.0);

    // Generate a new seed for the moisture noise
    let moisture_noise = NoiseBuilder::fbm_2d(
        OVERWORLD_SIZE_WIDTH as usize,
        OVERWORLD_SIZE_HEIGHT as usize,
    )
    .with_freq(0.03)
    .with_gain(3.5)
    .with_lacunarity(0.75)
    .with_octaves(4)
    .with_seed(map_seed.map_moisture_seed)
    .generate_scaled(0.0, 1.0);

    // For each tile, create the proper entity with the corresponding texture according to it's
    // height.
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let index = x + OVERWORLD_SIZE_WIDTH * y;
            let elevation_value = elevation_noise.get(index as usize).unwrap();
            let moisture_value = moisture_noise.get(index as usize).unwrap();
            let texture_index = biome(*elevation_value, *moisture_value);
            // let walkable = tile_walkable(texture_index);

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index),
                    ..Default::default()
                })
                .id();
            
            // if !walkable {
            //     println!("Tile @ {},{} is not walkable and it's index is {}", tile_pos.x, tile_pos.y, texture_index);
            //     commands.entity(tile_entity).insert(TileCollider);
                
            // }
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
        println!("Deepwater...");
        return TileType::DeepWater as u32;
    } else if elevation < 0.12 {
        println!("Shallowwater...");
        return TileType::ShallowWater as u32;
    }

    if elevation > 0.8 {
        if moisture < 0.33 {
            return TileType::Dirt as u32;
        } // scorched
        if moisture < 0.66 {
            return TileType::Sand as u32;
        } // bare
        if moisture < 0.1 {
            return TileType::Savannah as u32;
        } //tundra
        return TileType::Snow as u32;
    }

    if elevation > 0.6 {
        if moisture < 0.1 {
            return TileType::Dirt as u32;
        } // temperate_desert
        if moisture < 0.1 {
            return TileType::Sand as u32;
        } // shrubland
        return TileType::Savannah as u32; // Taiga
    }

    if elevation > 0.3 {
        if moisture < 0.16 {
            return TileType::Dirt as u32;
        } // temperate_desert
        if moisture < 0.50 {
            return TileType::Grass as u32;
        } // grassland
        if moisture < 0.83 {
            return TileType::Forest as u32;
        } //temperate_deciduous_forest
        return TileType::Forest as u32; // temperate rain forest
    }

    if moisture < 0.16 {
        return TileType::Sand as u32;
    } // subtropical desert
    if moisture < 0.33 {
        return TileType::Grass as u32;
    } // grassland
    if moisture < 0.66 {
        return TileType::Forest as u32;
    } //tropical seasonal forest

    TileType::Forest as u32 // tropical rain forest
}

fn move_event_listener(
    mut move_events: EventReader<MoveEvent>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    tile_query: Query<&mut TileTextureIndex>,
    mut move_legal: EventWriter<MoveLegal>,
) {
    for move_event in move_events.iter() {
        for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
            // Make sure that the destination is correct relative to the map due to any map transformation.
            let dest_in_map_pos: Vec2 = {
                let destination_pos = Vec4::from((move_event.destination.unwrap(), 1.0));
                let dest_in_map_pos = map_transform.compute_matrix().inverse() * destination_pos;
                dest_in_map_pos.xy()
            };
            // Once we have a world position we can transform it into a possible tile position.
            if let Some(tile_pos) =
                TilePos::from_world_pos(&dest_in_map_pos, map_size, grid_size, map_type)
            {
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    {
                        if let Ok(tile_texture) = tile_query.get(tile_entity) {
                            println!("{:?}", tile_texture);
                            let walkable = tile_walkable(tile_texture.0);
                            if walkable {
                                info!("Tile is walkable");
                                move_legal.send(MoveLegal { 
                                    legal_move: true,
                                    destination: move_event.destination, 
                                });
                            } else {
                                info!("Tile is not walkable");
                                move_legal.send(MoveLegal {
                                    legal_move: false,
                                    destination: None,
                                 });
                            }
                        }
                    }
                }
            }
        }
    }
}
