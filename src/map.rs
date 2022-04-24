use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TilemapPlugin)
            .add_startup_system(setup)
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
) {
    let texture_handle = asset_server.load("assets/tiles/terrain_tiles.png");
}

