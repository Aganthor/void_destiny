use bevy::{
    prelude::*, 
    render::render_resource::TextureUsages,
};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use simdnoise::*;

use crate::constants::*;


pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup)
            .add_system(set_texture_filters_to_nearest);
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
    let seed = rng.gen();

    let noise = NoiseBuilder::fbm_2d(OVERWORLD_SIZE_WIDTH as usize, OVERWORLD_SIZE_HEIGHT as usize)
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
            let noise_value = noise.get(index as usize).unwrap();
            let mut texture_id = 0;
            if noise_value < &0.1 {
                texture_id = 0;
            } else if noise_value < &0.2 {
                texture_id = 1;
            } else if noise_value < &0.3 {
                texture_id = 2;
            } else if noise_value < &0.5 {
                texture_id = 3;
            } else if noise_value < &0.8 {
                texture_id = 4;
            } else if noise_value < &0.9 {
                texture_id = 5;
            } else if noise_value < &0.95 {
                texture_id = 6;
            } else {
                texture_id = 7;
            }
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex { 0: texture_id },
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

pub fn set_texture_filters_to_nearest(
    mut texture_events: EventReader<AssetEvent<Image>>,
    mut textures: ResMut<Assets<Image>>,
) {
    // quick and dirty, run this for all textures anytime a texture is created.
    for event in texture_events.iter() {
        match event {
            AssetEvent::Created { handle } => {
                if let Some(mut texture) = textures.get_mut(handle) {
                    texture.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
                        | TextureUsages::COPY_SRC
                        | TextureUsages::COPY_DST;
                }
            }
            _ => (),
        }
    }
}