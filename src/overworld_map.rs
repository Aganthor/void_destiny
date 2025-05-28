use bevy::{math::Vec4Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::helpers;
use rand::prelude::*;
use noise::{NoiseFn, OpenSimplex, Fbm, MultiFractal};
use std::collections::HashSet;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

use crate::{constants::*, player::Player};
use crate::events::{MoveEvent, MoveLegal};
use crate::{tile_type::*, PlayerCamera};

// #[derive(Component, Inspectable)]
// pub struct TileCollider;

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };


//#[derive(Resource, Inspectable)]
#[derive(Reflect, Resource, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct OverWorldMapConfig {
    elevation_seed: i32,
    moisture_seed: i32,
    magnification: f32,
    frequency: f64,
    octaves: f32,
    lacunarity: f32,
    gain: f32,
    amplitude: f32,
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
            frequency: 1.12,
            octaves: 5.0,
            lacunarity: 0.7,
            gain: 0.5,
            amplitude: 0.5,            
            offset_x: 0,
            offset_y: 0,
        }
    }
}

#[derive(Default, Debug, Resource)]
struct ChunkManager {
    pub spawned_chunks: HashSet<IVec2>,
}

pub struct OverWorldMapPlugin;

impl Plugin for OverWorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilemapPlugin)
            .init_resource::<OverWorldMapConfig>()
            .register_type::<OverWorldMapConfig>()
            .insert_resource(ChunkManager::default())
            .add_plugins(ResourceInspectorPlugin::<OverWorldMapConfig>::default())
//            .add_systems(Startup, setup_camera)
            .add_systems(Update, camera_movement)
            .add_systems(Update, spawn_chunk_around_camera)
            .add_systems(Update, despawn_outofrange_chunks)
            .add_systems(Update, detect_player_edge)
            .add_systems(Update, move_event_listener);
    }
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::KeyZ) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::KeyX) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }    
}

// fn setup_camera(mut commands: Commands) {
//     commands.spawn(Camera2dBundle {
//         camera: Camera { 
//             clear_color: ClearColorConfig::Custom(BG_COLOR),
//             ..Default::default()
//         },
//         ..Default::default()
//     });
// }

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

fn spawn_chunk_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    map_config: Res<OverWorldMapConfig>,
) {
    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
        for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
            for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
                if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
                    chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
                    spawn_chunk(&mut commands, &asset_server, &map_config, IVec2::new(x, y));
                }
            }
        }
    }

}

///
/// Will despawn chunks that are out of range of the camera.
///
fn despawn_outofrange_chunks(
    mut commands: Commands,
    camera_query: Query<&Transform, With<Camera>>,
    chunks_query: Query<(Entity, &Transform)>,
    mut chunk_manager: ResMut<ChunkManager>
) {
    const CHUNK_DESPAWN_DISTANCE: f32 = (CHUNK_SIZE.x as f32 * TILE_SIZE.x) * 3.5;

    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > CHUNK_DESPAWN_DISTANCE {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x as f32)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y as f32)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}


///
/// This function spawns a chunk of the overworld map.
/// 
fn spawn_chunk(
    commands: &mut Commands, 
    asset_server: &AssetServer,
    map_config: &OverWorldMapConfig,
    chunk_pos: IVec2,
) {
    let texture_handle = asset_server.load("tiles/overworld_tiles.png");
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());
    let fbm = Fbm::<OpenSimplex>::new(map_config.elevation_seed as u32)
        .set_octaves(map_config.octaves as usize)
        .set_frequency(map_config.frequency)
        .set_lacunarity(map_config.lacunarity as f64);
    let open_simple_moisture = OpenSimplex::new(map_config.moisture_seed as u32);

    for x in 0..CHUNK_SIZE.x {        
        for y in 0..CHUNK_SIZE.y {            
            let tile_pos = TilePos { x, y };
            let nx: f64 = (chunk_pos.x as f64 * CHUNK_SIZE.x as f64 + x as f64) / OVERWORLD_SIZE_WIDTH as f64 - 0.5;
            let ny: f64 = (chunk_pos.y as f64 * CHUNK_SIZE.y as f64 + y as f64) / OVERWORLD_SIZE_HEIGHT as f64 - 0.5;
            let mut elevation_value = fbm.get([nx, ny]);
            
            elevation_value += 1.0 * fbm.get([1.0 * nx, 1.0 * ny]);
            elevation_value += 0.5 * fbm.get([2.0 * nx, 2.0 * ny]);
            elevation_value += 0.25 * fbm.get([4.0 * nx, 4.0 * ny]);
            elevation_value /= 1.0 + 0.25 + 0.5;
            elevation_value = elevation_value.powf(1.28);
            
            let moisture_value = open_simple_moisture.get([map_config.frequency * nx, map_config.frequency * ny]);
            let texture_index = biome(elevation_value, moisture_value);

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(texture_index),
                    ..Default::default()
                })
                .id();
            commands.entity(tilemap_entity).add_child(tile_entity);
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let transform = Transform::from_translation(Vec3::new(
        chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
        chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
        0.0,
    ));

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: CHUNK_SIZE.into(),
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        transform,
        render_settings: TilemapRenderSettings {
            render_chunk_size: RENDER_CHUNK_SIZE,
            ..Default::default()
        },
        ..Default::default()
    });
}

///
/// Simple function to determine the biome depending on elevation and moisture.
/// 
fn biome(elevation: f64, moisture: f64) -> u32 {
    // if elevation < 0.11 {
    //     TileType::DeepWater as u32
    // } else {
    //     TileType::Grass as u32
    // }
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
    for move_event in move_events.read() {
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
                                    //legal_move: false,
                                    legal_move: true,
                                    //destination: None,
                                    destination: move_event.destination,
                                 });
                            }
                        }
                    }
                }

                // Is the player about to move to the edge?
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
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut map_config: ResMut<OverWorldMapConfig>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyL) {
        map_config.offset_x += 1;
        println!("New x_offset = {}", map_config.offset_x);
    }
}
