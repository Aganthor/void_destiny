use bevy::{math::Vec3Swizzles, prelude::*, platform::collections::HashSet};
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::helpers::*;
use noise::{Fbm, NoiseFn, Perlin, OpenSimplex};
use rand::Rng;

use crate::tile_type::GroundTiles;


#[derive(Default)]
pub struct WorldGenIslandPlugin;

impl Plugin for WorldGenIslandPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .add_systems(Startup, startup)
            .add_systems(Startup, spawn_chunk)
            // .add_systems(Update, spawn_chunk_around_camera)
            // .add_systems(Update, despawn_outofrange_chunks)
            .add_systems(Update, camera_movement);
    }
}

const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
// For this example, don't choose too large a chunk size.
const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};

// Configuration
const WIDTH: u32 = TILE_SIZE.x as u32 * 20;
const HEIGHT: u32 = TILE_SIZE.y as u32 * 20;
const SEALEVEL: f64 = 0.15;
const RIVER_THRESHOLD: f64 = 20.0; // Higher = fewer, thicker rivers
const NUM_DROPS: usize = 25000;


fn startup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Projection), With<Camera>>,
) {
    for (mut transform, mut projection) in query.iter_mut() {
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

        let Projection::Orthographic(ortho) = &mut *projection else {
            continue;
        };

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
        transform.translation += time.delta_secs() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }    
}

fn camera_pos_to_chunk_pos(camera_pos: &Vec2) -> IVec2 {
    let camera_pos = camera_pos.as_ivec2();
    let chunk_size: IVec2 = IVec2::new(CHUNK_SIZE.x as i32, CHUNK_SIZE.y as i32);
    let tile_size: IVec2 = IVec2::new(TILE_SIZE.x as i32, TILE_SIZE.y as i32);
    camera_pos / (chunk_size * tile_size)
}

// fn spawn_chunk_around_camera(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     camera_query: Query<&Transform, With<Camera>>,
//     mut chunk_manager: ResMut<ChunkManager>,
//     map_config: Res<OverWorldMapConfig>,
// ) {
//     for transform in camera_query.iter() {
//         let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());
//         for y in (camera_chunk_pos.y - 2)..(camera_chunk_pos.y + 2) {
//             for x in (camera_chunk_pos.x - 2)..(camera_chunk_pos.x + 2) {
//                 if !chunk_manager.spawned_chunks.contains(&IVec2::new(x, y)) {
//                     chunk_manager.spawned_chunks.insert(IVec2::new(x, y));
//                     spawn_chunk(&mut commands, &asset_server, &map_config, IVec2::new(x, y));
//                 }
//             }
//         }
//     }

// }

///
/// Will despawn chunks that are out of range of the camera.
///
// fn despawn_outofrange_chunks(
//     mut commands: Commands,
//     camera_query: Query<&Transform, With<Camera>>,
//     chunks_query: Query<(Entity, &Transform)>,
//     mut chunk_manager: ResMut<ChunkManager>
// ) {
//     const CHUNK_DESPAWN_DISTANCE: f32 = (CHUNK_SIZE.x as f32 * TILE_SIZE.x) * 3.5;

//     for camera_transform in camera_query.iter() {
//         for (entity, chunk_transform) in chunks_query.iter() {
//             let chunk_pos = chunk_transform.translation.xy();
//             let distance = camera_transform.translation.xy().distance(chunk_pos);
//             if distance > CHUNK_DESPAWN_DISTANCE {
//                 let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x as f32)).floor() as i32;
//                 let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y as f32)).floor() as i32;
//                 chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
//                 commands.entity(entity).despawn();
//             }
//         }
//     }
// }


///
/// This function spawns a chunk of the overworld map.
/// 
fn spawn_chunk(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    // let texture_handle = asset_server.load("tiles/overworld_tiles.png");
    let texture_handle = asset_server.load("tiles/grounds_tiles.png");
    let tilemap_entity = commands.spawn_empty().id();
    let tile_map_size = TilemapSize::new(WIDTH, HEIGHT);
    let mut tile_storage = TileStorage::empty(tile_map_size.into());

    // 1. Setup Noise Generators
    let mut rng = rand::rng();
    let seed = rng.random();
    let island_scale = 1.3; // Larger = bigger island    
    let elev_gen = Fbm::<Perlin>::new(seed);
    let moist_gen = Fbm::<Perlin>::new(seed + 1);
    let warp_gen = Fbm::<Perlin>::new(seed + 2);

    // 2. Pre-calculate Elevation and Moisture Maps
    // We store these in Vectors so the river simulation can access them easily.
    let mut elevation_map = vec![0.0; (WIDTH * HEIGHT) as usize];
    let mut moisture_map = vec![0.0; (WIDTH * HEIGHT) as usize];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = (y * WIDTH + x) as usize;
            // Normalized coordinates (unscaled) so we can compute an edge mask
            let nx0 = 2.0 * (x as f64 / WIDTH as f64) - 1.0;
            let ny0 = 2.0 * (y as f64 / HEIGHT as f64) - 1.0;
            // Apply island scale to the coordinates used for noise sampling
            let mut nx = nx0 / island_scale;
            let mut ny = ny0 / island_scale;

            // Elevation with Masking
            // 1. Higher frequency for more crags
            let freq = 3.5; 
            let mut e = (elev_gen.get([nx * freq, ny * freq]) + 1.0) / 2.0;

            // 2. REDISTRIBUTION: This is the "Mountain Maker"
            // Squaring or cubing the value makes the peaks much sharper.
            e = e.powf(2.5); 

            // 3. BETTER MASK: 
            // We calculate a distance-based shelf. 
            // Instead of simple subtraction, we use this to 'force' the edges down
            // while leaving the center mostly untouched.
            // Compute distance-based mask from the unscaled coordinates so
            // the island always falls off to ocean at the image edges.
            let d = (nx0 * nx0 + ny0 * ny0).sqrt();
            let mask = (1.0 - d.powf(2.0)).clamp(0.0, 1.0);

            // Multiply elevation by mask so edges are 0, but center keeps its peak height.
            let final_elev = e * mask; 
            elevation_map[idx] = final_elev;

            // Warped Moisture
            let qx = warp_gen.get([nx * 2.0, ny * 2.0]) * 0.4;
            let qy = warp_gen.get([nx * 2.0 + 5.2, ny * 2.0 + 1.3]) * 0.4;
            let mut m = (moist_gen.get([nx + qx, ny + qy]) + 1.0) / 2.0;
            
            // Terrain coupling: Lower areas near water are naturally wetter
            let height_factor = 1.0 - elevation_map[idx];
            moisture_map[idx] = (m * 0.6 + height_factor * 0.4).clamp(0.0, 1.0);
        }
    }   
    info!("Elevation and Moisture maps generated.");

    // 4. Final Rendering
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = (y * WIDTH + x) as usize;
            let e = elevation_map[idx];
            let m = moisture_map[idx];
            let tile_pos = TilePos { x, y };

            let texture_index = biome(e, m);

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

    // for x in 0..CHUNK_SIZE.x {        
    //     for y in 0..CHUNK_SIZE.y {            
    //         let tile_pos = TilePos { x, y };
    //         let nx: f64 = (chunk_pos.x as f64 * CHUNK_SIZE.x as f64 + x as f64) / OVERWORLD_SIZE_WIDTH as f64 - 0.5;
    //         let ny: f64 = (chunk_pos.y as f64 * CHUNK_SIZE.y as f64 + y as f64) / OVERWORLD_SIZE_HEIGHT as f64 - 0.5;
    //         let mut elevation_value = elevation_noise.get([nx, ny]);
            
    //         elevation_value += 1.0 * elevation_noise.get([1.0 * nx, 1.0 * ny]);
    //         elevation_value += 0.5 * elevation_noise.get([2.0 * nx, 2.0 * ny]);
    //         elevation_value += 0.25 * elevation_noise.get([4.0 * nx, 4.0 * ny]);
    //         elevation_value += 0.13 * elevation_noise.get([8.0 * nx, 8.0 * ny]);
    //         elevation_value += 0.06 * elevation_noise.get([16.0 * nx, 16.0 * ny]);
    //         elevation_value += 0.03 * elevation_noise.get([32.0 * nx, 32.0 * ny]);
    //         elevation_value /= 1.0 + 0.25 + 0.5 + 0.13 + 0.06 + 0.03;
    //         // Normalize the elevation value to be between 0 and 1
    //         elevation_value = (elevation_value + 1.0) / 2.0;
    //         elevation_value = elevation_value.clamp(0.0, 1.0);
    //         elevation_value = elevation_value.powf(map_config.pow_factor);
            
    //         let mut moisture_value = moisture_noise.get([nx, ny]);

    //         // moisture_value += 1.0 * moisture_noise.get([1.0 * nx, 1.0 * ny]);
    //         // moisture_value += 0.5 * moisture_noise.get([2.0 * nx, 2.0 * ny]);
    //         // moisture_value += 0.25 * moisture_noise.get([4.0 * nx, 4.0 * ny]);
    //         // moisture_value += 0.13 * moisture_noise.get([8.0 * nx, 8.0 * ny]);
    //         // moisture_value += 0.06 * moisture_noise.get([16.0 * nx, 16.0 * ny]);
    //         // moisture_value += 0.03 * moisture_noise.get([32.0 * nx, 32.0 * ny]);
    //         // moisture_value /= 1.0 + 0.25 + 0.5 + 0.13 + 0.06 + 0.03;
    //         // // Normalize the moisture value to be between 0 and 1
    //         // let moisture_value = (moisture_value + 1.0) / 2.0;  
    //         // let moisture_value = moisture_value.clamp(0.0, 1.0);

    //         let texture_index = biome(elevation_value, moisture_value);

    //         let tile_entity = commands
    //             .spawn(TileBundle {
    //                 position: tile_pos,
    //                 tilemap_id: TilemapId(tilemap_entity),
    //                 texture_index: TileTextureIndex(texture_index),
    //                 ..Default::default()
    //             })
    //             .id();
    //         commands.entity(tilemap_entity).add_child(tile_entity);
    //         tile_storage.set(&tile_pos, tile_entity);
    //     }
    // }

    // let transform = Transform::from_translation(Vec3::new(
    //     chunk_pos.x as f32 * CHUNK_SIZE.x as f32 * TILE_SIZE.x,
    //     chunk_pos.y as f32 * CHUNK_SIZE.y as f32 * TILE_SIZE.y,
    //     0.0,
    // ));
    let transform = Transform::from_translation(Vec3::ZERO);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size: TILE_SIZE.into(),
        size: tile_map_size,
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
fn biome(e: f64, m: f64) -> u32 {
    if e < SEALEVEL {
        return if e < SEALEVEL - 0.1 { Rgb([20, 50, 100]) } else { Rgb([40, 90, 160]) };
    }

    if e < SEALEVEL + 0.03 { return Rgb([230, 220, 160]); } // Beach

    if e > 0.45 {
        return if m > 0.4 { Rgb([255, 255, 255]) } else { Rgb([100, 100, 100]) }; // Snow vs Rock
    }

    if e > 0.4 {
        if m > 0.6 { return Rgb([34, 139, 34]); }    // Forest
        if m > 0.3 { return Rgb([100, 150, 70]); }   // Shrubland
        return Rgb([180, 160, 120]);                 // Tundra/Barren
    }

    // Lowlands
    if m > 0.7 { return Rgb([0, 80, 40]); }      // Jungle
    if m > 0.4 { return Rgb([60, 160, 60]); }    // Grassland
    if m > 0.15 { return Rgb([160, 180, 90]); }  // Savannah
    Rgb([210, 180, 110])                         // Desertm
}