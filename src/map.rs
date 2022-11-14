use bevy::{prelude::*, render::render_resource::TextureUsages};
use bevy_ecs_tilemap::prelude::*;
use rand::prelude::*;
use simdnoise::*;
use bevy_tileset::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TerrainTileSet>()
            .add_plugin(TilemapPlugin)
            .add_plugin(TilesetPlugin::default())
            .add_startup_system_to_stage(StartupStage::PreStartup, load_tiles)
            .add_startup_system(setup)
            .add_system(set_texture_filters_to_nearest);
    }
}

#[derive(Default, Resource)]
pub struct TerrainTileSet {
    handle: Option<Handle<Tileset>>,
}

pub fn load_tiles(mut terrain_tileset: ResMut<TerrainTileSet>, asset_server: Res<AssetServer>) {
    terrain_tileset.handle = Some(asset_server.load("terrain_tileset.ron"));

}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    terrain_tileset: Res<TerrainTileSet>,
//    mut map_query: MapQuery,
) {
    let texture_handle = asset_server.load("tiles/terrain_tiles.png");

    // Create map entity and component
    let map_entity = commands.spawn().id();
    let mut map = Map::new(0u16, map_entity);

    let (mut layer_builder, _) = LayerBuilder::new(
        &mut commands,
        LayerSettings::new(
            MapSize(2, 2),
            ChunkSize(8, 8),
            TileSize(32.0, 32.0),
            TextureSize(374.0, 32.0),
        ),
        0u16,
        0u16,
    );

    let mut rng = rand::thread_rng();
    let seed = rng.gen();

    let noise = NoiseBuilder::fbm_2d(16, 16)
        .with_freq(0.03)
        .with_gain(2.5)
        .with_lacunarity(0.55)
        .with_octaves(2)
        .with_seed(seed)
        .generate_scaled(0.0, 1.0);

    // layer_builder.for_each_tiles_mut(|tile_entity, tile_data| {
        
    //     // Tile entity might not be there yet. Create it.
    //     if tile_entity.is_none() {
    //         *tile_entity = Some(commands.spawn().id());
    //     }
    //     commands
    //         .entity(tile_entity.unwrap());
    // });

    layer_builder.set_all(TileBundle::default());

    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    map.add_layer(&mut commands, 0u16, layer_entity);

    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
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