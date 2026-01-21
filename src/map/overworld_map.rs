use bevy::{math::Vec4Swizzles, prelude::*};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use noise::{Fbm, MultiFractal, NoiseFn, OpenSimplex, Perlin, Clamp, Blend, RidgedMulti};
use std::collections::HashSet;
use bevy_inspector_egui::{
    bevy_inspector,
    DefaultInspectorConfigPlugin,
    bevy_egui::{EguiContext, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext},
    prelude::*,
};

use crate::{constants::*};
use crate::events::{MoveEvent, MoveLegal};
use crate::{tile_type::*};
use crate::states::GameState;


const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 32.0, y: 32.0 };
// maximum number of chunks that can exist (derived from OVERWORLD_SIZE_* and CHUNK_SIZE)
const MAX_SPAWNED_CHUNKS: usize = ((OVERWORLD_SIZE_WIDTH as usize + CHUNK_SIZE.x as usize - 1) / CHUNK_SIZE.x as usize)
    * ((OVERWORLD_SIZE_HEIGHT as usize + CHUNK_SIZE.y as usize - 1) / CHUNK_SIZE.y as usize);


#[derive(Reflect, Resource, InspectorOptions, Debug, Clone)]
#[reflect(Resource, InspectorOptions)]
pub struct OverWorldMapConfig {
    e_seed: i32,
    m_seed: i32,
    frequency: f64,
    octaves: f32,
    lacunarity: f64,
    persistance: f64,
    amplitude: f32,
    pow_factor: f64,
}

impl Default for OverWorldMapConfig {
    fn default() -> Self {
        let mut rng = rand::rng();

        OverWorldMapConfig { 
            e_seed: rng.random(),
            m_seed: rng.random(),
            frequency: 2.50,
            octaves: 5.0,
            lacunarity: 0.7,
            persistance: 2.0,
            amplitude: 0.5,
            pow_factor: 1.75,
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
            .insert_resource(ChunkManager::default())
            .add_plugins(EguiPlugin::default())
            .add_plugins(DefaultInspectorConfigPlugin)
            .init_resource::<OverWorldMapConfig>()
            .register_type::<OverWorldMapConfig>()
            .add_systems(Update, spawn_chunk_around_camera)
            .add_systems(Update, despawn_outofrange_chunks)
            //.add_systems(Update, camera_movement)
            .add_systems(Update, reset_map.run_if(in_state(GameState::DirtyMap)))
            .add_systems(EguiPrimaryContextPass, inspector_ui)
            .add_systems(Update, move_event_listener);
    }
}

fn inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut ctx = egui_context.clone();
    egui::Window::new("Noise generation configuration").show(ctx.get_mut(), |ui| {
        egui::ScrollArea::both().show(ui, |ui| {

            bevy_inspector::ui_for_resource::<OverWorldMapConfig>(world, ui);

            if ui.add(egui::Button::new("Regenerate map!")).clicked() {
                world.resource_mut::<NextState<GameState>>().set(GameState::DirtyMap);
            }
        });
    });
}

fn reset_map(
    mut commands: Commands,
    chunks_query: Query<(Entity, &Transform), With<TilemapTileSize>>,
    mut chunk_manager: ResMut<ChunkManager>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (entity, _transform) in chunks_query.iter() {
        commands.entity(entity).despawn();
    }
    chunk_manager.spawned_chunks.clear();
    next_state.set(GameState::GameRunning);
    info!("Map has been reset.");
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

fn spawn_chunk_around_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera>>,
    mut chunk_manager: ResMut<ChunkManager>,
    map_config: Res<OverWorldMapConfig>,
) {
    // number of chunks that fit in the overworld grid
    let chunks_x = ((OVERWORLD_SIZE_WIDTH as i32 + CHUNK_SIZE.x as i32 - 1) / CHUNK_SIZE.x as i32);
    let chunks_y = ((OVERWORLD_SIZE_HEIGHT as i32 + CHUNK_SIZE.y as i32 - 1) / CHUNK_SIZE.y as i32);

    for transform in camera_query.iter() {
        let camera_chunk_pos = camera_pos_to_chunk_pos(&transform.translation.xy());

        // clamp spawn window to the finite map (avoid negative/overflow chunk coords)
        let min_x = 0;
        let max_x = chunks_x - 1;
        let min_y = 0;
        let max_y = chunks_y - 1;

        let start_x = (camera_chunk_pos.x - 2).clamp(min_x, max_x);
        let end_x = (camera_chunk_pos.x + 2).clamp(min_x, max_x);
        let start_y = (camera_chunk_pos.y - 2).clamp(min_y, max_y);
        let end_y = (camera_chunk_pos.y + 2).clamp(min_y, max_y);

        for y in start_y..=end_y {
            for x in start_x..=end_x {
                // optional: stop spawning if we've reached the overall maximum
                if chunk_manager.spawned_chunks.len() >= MAX_SPAWNED_CHUNKS {
                    return;
                }

                let pos = IVec2::new(x, y);
                if !chunk_manager.spawned_chunks.contains(&pos) {
                    chunk_manager.spawned_chunks.insert(pos);
                    spawn_chunk(&mut commands, &asset_server, &map_config, pos);
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
    const CHUNK_DESPAWN_DISTANCE: f32 = (CHUNK_SIZE.x as f32 * TILE_SIZE.x) * 6.5;

    for camera_transform in camera_query.iter() {
        for (entity, chunk_transform) in chunks_query.iter() {
            let chunk_pos = chunk_transform.translation.xy();
            let distance = camera_transform.translation.xy().distance(chunk_pos);
            if distance > CHUNK_DESPAWN_DISTANCE {
                let x = (chunk_pos.x / (CHUNK_SIZE.x as f32 * TILE_SIZE.x as f32)).floor() as i32;
                let y = (chunk_pos.y / (CHUNK_SIZE.y as f32 * TILE_SIZE.y as f32)).floor() as i32;
                chunk_manager.spawned_chunks.remove(&IVec2::new(x, y));
                commands.entity(entity).despawn();
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
    let texture_handle = asset_server.load("tiles/grounds_tiles.png");
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(CHUNK_SIZE.into());

    let open_simplex: OpenSimplex = OpenSimplex::new(map_config.e_seed as u32);
    let ridged = RidgedMulti::<OpenSimplex>::new(map_config.e_seed as u32);
    let fbm_main = Fbm::<OpenSimplex>::new(map_config.e_seed as u32)
        .set_octaves(map_config.octaves as usize)
        .set_frequency(map_config.frequency)
        .set_persistence(map_config.persistance)
        .set_lacunarity(map_config.lacunarity);
    let fbm_warp = Fbm::<OpenSimplex>::new(map_config.e_seed as u32)
        .set_octaves(map_config.octaves as usize)
        .set_frequency(map_config.frequency)
        .set_persistence(map_config.persistance)
        .set_lacunarity(map_config.lacunarity);    
    let e_noise: Blend<f64, _, _, _, 2> = Blend::new(open_simplex, ridged, fbm_main);
    let m_noise = OpenSimplex::new(map_config.m_seed as u32);
    let temp_noise = OpenSimplex::new((map_config.m_seed as u32).wrapping_add(12345)); // different seed for temperature

    for x in 0..CHUNK_SIZE.x {        
        for y in 0..CHUNK_SIZE.y {            
            let tile_pos = TilePos { x, y };
            let world_x = chunk_pos.x as f64 * CHUNK_SIZE.x as f64 + x as f64;
            let world_y = chunk_pos.y as f64 * CHUNK_SIZE.y as f64 + y as f64;
            let nx: f64 = world_x / OVERWORLD_SIZE_WIDTH as f64 - 0.5;
            let ny: f64 = world_y / OVERWORLD_SIZE_HEIGHT as f64 - 0.5;

            // Domain-warp for more organic terrain
            let warp_amp = 0.08; // tweakable
            let warp = fbm_warp.get([nx * 2.0, ny * 2.0]) * warp_amp;
            let mut e_value = e_noise.get([nx + warp, ny + warp]);

            // multi-scale detail (kept but normalized)
            e_value += 0.5 * e_noise.get([2.0 * (nx + warp), 2.0 * (ny + warp)]);
            e_value += 0.25 * e_noise.get([4.0 * (nx + warp), 4.0 * (ny + warp)]);
            e_value /= 1.0 + 0.5 + 0.25;
            e_value = normalize_noise(e_value);
            e_value = e_value.powf(map_config.pow_factor);

            // Moisture: base noise, biased by elevation (lowlands wetter) and some temperature influence
            let mut m_value = normalize_noise(m_noise.get([nx * 1.5, ny * 1.5]));
            m_value = m_value * 0.7 + (1.0 - e_value) * 0.3; // mountains drier

            // Temperature: latitude gradient + noise + elevation penalty (higher = colder)
            let lat = 1.0 - (ny + 0.5).abs() * 1.0; // center is warm, poles cold
            let mut t_value = lat.clamp(0.0, 1.0);
            t_value += normalize_noise(temp_noise.get([nx * 2.0, ny * 2.0])) * 0.12;
            t_value -= e_value * 0.5; // elevation cools
            let t_value = t_value.clamp(0.0, 1.0);

            let texture_index = biome(e_value, m_value, t_value);

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

fn normalize_noise(v: f64) -> f64 {
    ((v + 1.0) / 2.0).clamp(0.0, 1.0)
}

fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

///
/// Simple function to determine the biome depending on elevation (e) and moisture (m).
/// 
fn biome(e: f64, m: f64, t: f64) -> u32 {
    // soften water/coast/mountain thresholds
    let water_f = smoothstep(0.18, 0.26, e);
    if water_f > 0.66 { return GroundTiles::DarkShallowWater as u32; }
    if water_f > 0.33 { return GroundTiles::MediumShallowWater as u32; }
    if water_f > 0.0  { return GroundTiles::LightShallowWater as u32; }

    // gentle beach band
    let beach_f = smoothstep(0.26, 0.30, e);
    if beach_f > 0.5 && m < 0.45 { return GroundTiles::LightDirt as u32; }

    // mountains with softened snowline
    let mountain_f = smoothstep(0.80, 0.86, e);
    if mountain_f > 0.8 {
        if t < 0.3 { return GroundTiles::DarkSnowyMountain as u32; }
        return GroundTiles::LightRockSnowyMountain as u32;
    }

    // Biomes based on moisture & temperature (unchanged logic, smoothed where helpful)
    if t < 0.35 && m > 0.45 { return GroundTiles::BrightPineForest as u32; }
    if m < 0.15 && t > 0.6 { return GroundTiles::LightSandyMountain as u32; }
    if m > 0.7 { return GroundTiles::BrightLushForest as u32; }
    if m > 0.45 { return GroundTiles::BrightDeciduousForest as u32; }
    if e > 0.5 { return GroundTiles::MediumGrass as u32; }
    GroundTiles::LightGrass as u32
}

///
/// This method is used to check for event. The player system sends a MoveEvent and this system
/// reads it. It then determines whether the destination tile is walkable or not. It then sends
/// a MoveLegal event.
/// 
fn move_event_listener(
    mut move_events: MessageReader<MoveEvent>,
    tilemap_q: Query<(
        &TilemapSize,
        &TilemapGridSize,
        &TilemapType,
        &TileStorage,
        &Transform,
    )>,
    tile_query: Query<&mut TileTextureIndex>,
    mut move_legal: MessageWriter<MoveLegal>,
) {
    for move_event in move_events.read() {
        for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
            if move_event.destination.is_none() {
                return;
            }
            // Make sure that the destination is correct relative to the map due to any map transformation.
            let dest_in_map_pos: Vec2 = {
                let destination_pos = Vec4::from((move_event.destination.unwrap(), 1.0));
                let dest_in_map_pos = map_transform.to_matrix().inverse() * destination_pos;
                dest_in_map_pos.xy()
            };
            // Once we have a world position we can transform it into a possible tile position.
            if let Some(tile_pos) =
                TilePos::from_world_pos(&dest_in_map_pos, map_size, grid_size, &TILE_SIZE, map_type, &TilemapAnchor::None)
            {
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    {
                        if let Ok(tile_texture) = tile_query.get(tile_entity) {
                            let walkable = tile_walkable(tile_texture.0);
                            if walkable {
                                move_legal.write(MoveLegal { 
                                    legal_move: true,
                                    destination: move_event.destination, 
                                });
                            } else {
                                // move_legal.write(MoveLegal {
                                //     legal_move: false,
                                //     destination: None,
                                //  });
                                move_legal.write(MoveLegal {
                                    legal_move: true,
                                    destination: move_event.destination,
                                });
                            }
                        }
                    }
                }

                // Is the player about to move to the edge?
                // if tile_pos.x == 0 || tile_pos.x == OVERWORLD_SIZE_WIDTH - 1 || tile_pos.y == 0 || tile_pos.y == OVERWORLD_SIZE_HEIGHT - 1 {
                //     println!("Edge detected...");
                // }
            }
        }
    }
}

// pub fn detect_player_edge(
//     player_query: Query<&Transform, With<Player>>,
//     tilemap_q: Query<(&TilemapSize, &TilemapGridSize, &TilemapType, &Transform)>,
// ) {
//     let player = match player_query.single() {
//         Ok(player) => player,
//         Err(_) => return, // No player found
//     };
//     for (map_size, grid_size, map_type, map_transform) in tilemap_q.iter() {
//         // Make sure that the destination is correct relative to the map due to any map transformation.
//         let dest_in_map_pos: Vec2 = {
//             let destination_pos = Vec4::from((player.translation, 1.0));
//             let dest_in_map_pos = map_transform.compute_matrix().inverse() * destination_pos;
//             dest_in_map_pos.xy()
//         };
//         // Once we have a world position, we can transform it into a possible tile position.
//         if let Some(tile_pos) = TilePos::from_world_pos(&dest_in_map_pos, map_size, grid_size, &TILE_SIZE, map_type, &TilemapAnchor::None) {
//             if tile_pos.x == 0 || tile_pos.x == OVERWORLD_SIZE_WIDTH - 1 || tile_pos.y == 0 || tile_pos.y == OVERWORLD_SIZE_HEIGHT - 1 {
//                 println!("Edge detected...");
//             }
//         }
//     }
// }
