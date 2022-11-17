/// Void destiny is a RPG roguelike game written in rust with bevy!
///
///  Some crates to see / use :
///     - https://github.com/StarArawn/bevy_ecs_tilemap
///     - https://github.com/MrGVSV/bevy_tileset
/// 
/// Some nice sprite : https://toen.itch.io/toens-medieval-strategy
/// 
/// TODO:
/// - Save world map size in a ron file
/// 

use bevy::{
    prelude::*,
    window::PresentMode,
};

mod constants;
mod tile_type;
use constants::*;

mod player;
use player::*;

mod map;
use map::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins.build()
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Void destiny - The roguelike game!".to_string(),
                    width: WINDOW_WIDTH,
                    height: WINDOW_HEIGHT,
                    present_mode: PresentMode::AutoVsync,
                    ..default()
                },
                ..default()
            .set(ImagePlugin::default_nearest())
        }))
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d { clear_color: ClearColor(BG_COLOR) },
        ..Default::default()
    });
}

