use bevy::{math::Vec4Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use simdnoise::*;
//use bevy_inspector_egui::Inspectable;

use crate::{constants::*, player::Player};
use crate::events::{MoveEvent, MoveLegal};
use crate::tile_type::*;

// #[derive(Component, Inspectable)]
// pub struct TileCollider;


//#[derive(Resource, Inspectable)]
#[derive(Resource)]
pub struct OverWorldMapConfig {
    elevation_seed: i32,
    moisture_seed: i32,
    magnification: f32,
    offset_x: i32,
    offset_y: i32,
}

impl Default for OverWorldMapConfig {
    fn default() -> Self {
        let mut rng = rand::thread_rng();

        OverWorldMapConfig { 
            elevation_seed: rng.gen(),
            moisture_seed: rng.gen(),
            magnification: 7.0,        
            offset_x: 0,
            offset_y: 0,
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


pub struct OverWorldMapPlugin;

impl Plugin for OverWorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(TilemapPlugin)
            .init_resource::<OverWorldMapConfig>()
            .add_system(spawn_chunk)
            .add_system(detect_player_edge)
            //.add_system(change_x_offset)
            .add_system(move_event_listener);
    }
}

fn spawn_chunk(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    map_config: Res<OverWorldMapConfig>,
) {
    let texture_handle = asset_server.load("tiles/overworld_tiles.png");

    let tilemap_size = TilemapSize {
        x: OVERWORLD_SIZE_WIDTH,
        y: OVERWORLD_SIZE_HEIGHT,
    };
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    let elevation_noise = NoiseBuilder::fbm_2d(
        OVERWORLD_SIZE_WIDTH as usize - map_config.offset_x as usize,
        OVERWORLD_SIZE_HEIGHT as usize - map_config.offset_y as usize,
    )
    .with_freq(0.03)
    .with_gain(2.5)
    .with_lacunarity(0.55)
    .with_octaves(2)
    .with_seed(map_config.elevation_seed)
    .generate_scaled(0.0, 1.0);

    println!("elevation_noise size is {}", elevation_noise.len());

    // Generate a new seed for the moisture noise
    let moisture_noise = NoiseBuilder::fbm_2d(
        OVERWORLD_SIZE_WIDTH as usize,
        OVERWORLD_SIZE_HEIGHT as usize,
    )
    .with_freq(0.03)
    .with_gain(3.5)
    .with_lacunarity(0.75)
    .with_octaves(4)
    .with_seed(map_config.moisture_seed)
    .generate_scaled(0.0, 1.0);

    // For each tile, create the proper entity with the corresponding texture according to it's
    // height.
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let index = x + OVERWORLD_SIZE_WIDTH * y;
//            println!("Index is {}", index);
            let elevation_value = elevation_noise.get(index as usize).unwrap();
            let moisture_value = moisture_noise.get(index as usize).unwrap();
            let texture_index = biome(*elevation_value, *moisture_value);

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index),
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

///
/// Simple function to determine the biome depending on elevation and moisture.
/// 
fn biome(elevation: f32, moisture: f32) -> u32 {
    if elevation < 0.1 {
        return TileType::DeepWater as u32;
    } else if elevation < 0.12 {
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

///
/// This method is used to check for event. The player system sends a MoveEvent and this system
/// reads it. It then determines whether the destination tile is walkable or not. It then sends
/// a MoveLegal event.
/// 
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
            if move_event.destination.is_none() {
                return;
            }
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
                            let walkable = tile_walkable(tile_texture.0);
                            if walkable {
                                move_legal.send(MoveLegal { 
                                    legal_move: true,
                                    destination: move_event.destination, 
                                });
                            } else {
                                move_legal.send(MoveLegal {
                                    legal_move: false,
                                    destination: None,
                                 });
                            }
                        }
                    }
                }

                // Is the player about to move the the edge?
                if tile_pos.x == 0 || tile_pos.x == OVERWORLD_SIZE_WIDTH - 1 || tile_pos.y == 0 || tile_pos.y == OVERWORLD_SIZE_HEIGHT - 1 {
                    println!("Edge detected...");
                }
            }
        }
    }
}

pub fn detect_player_edge(
    player_query: Query<&Transform, With<Player>>,
    tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,    
) {
    let player = player_query.single();
    for (map_size, grid_size, map_type, map_transform) in tilemap_q.iter() {
        // Make sure that the destination is correct relative to the map due to any map transformation.
        let dest_in_map_pos: Vec2 = {
            let destination_pos = Vec4::from((player.translation, 1.0));
            let dest_in_map_pos = map_transform.compute_matrix().inverse() * destination_pos;
            dest_in_map_pos.xy()
        };
        // Once we have a world position we can transform it into a possible tile position.
        if let Some(tile_pos) = TilePos::from_world_pos(&dest_in_map_pos, map_size, grid_size, map_type) {
            if tile_pos.x == 0 || tile_pos.x == OVERWORLD_SIZE_WIDTH - 1 || tile_pos.y == 0 || tile_pos.y == OVERWORLD_SIZE_HEIGHT - 1 {
                println!("Edge detected...");
            }
        }
    }
}

fn change_x_offset(
    keyboard_input: Res<Input<KeyCode>>,
    mut map_config: ResMut<OverWorldMapConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::L) {
        map_config.offset_x += 1;
        println!("New x_offset = {}", map_config.offset_x);
    }
}