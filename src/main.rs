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
    window::PresentMode, core_pipeline::clear_color::ClearColorConfig,
};

use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod constants;
mod tile_type;
use constants::*;

mod player;
use player::*;

mod map;
use map::*;

mod events;
use events::*;

mod debug_plugin;
use debug_plugin::*;

fn main() {
    App::new()
        .add_event::<MoveEvent>()
        .add_event::<MoveLegal>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Void destiny - The roguelike game!".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        //.add_plugin(ImagePlugin::default_nearest())
        //.add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d { 
            clear_color: ClearColorConfig::Custom(BG_COLOR)
        },
        ..Default::default()
    });
}

