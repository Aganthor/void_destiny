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

mod overworld_map;
use overworld_map::*;

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
        })
        .set(ImagePlugin::default_nearest()),
        )
        //.add_plugin(DebugPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(OverWorldMapPlugin)
        .add_systems(Startup, setup_camera)
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

