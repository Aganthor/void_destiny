use bevy::prelude::*;
use bevy_ecs_tiled::{prelude::*, tiled::world::asset};

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_world_map)
            .add_plugins(TiledPlugin::default());
    }
}

fn setup_world_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
     // Load a map then spawn it
    commands.spawn((
        // Only the [`TiledMap`] component is actually required to spawn a map.
        TiledMap(asset_server.load("maps/terrain_preview.tmx")),
        // But you can add extra components to change the defaults settings and how
        // your map is actually displayed
        TilemapAnchor::Center,
    ));
}