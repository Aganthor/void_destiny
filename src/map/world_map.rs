use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledPlugin::default())
            .add_systems(Startup, setup_world_map);
    }
}

fn setup_world_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a map then spawn it
    commands.spawn((
        // Only the [`TiledMap`] component is actually required to spawn a map.
        TiledMap(asset_server.load("tiled_map_assets/overworld_map.tmx")),
        // But you can add extra components to change the defaults settings and how
        // your map is actually displayed
        TilemapAnchor::Center,
    ));
}
