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
    window::{PresentMode, WindowResolution},
};

mod constants;
mod tile_type;
use constants::*;

mod player;
use player::*;

mod overworld_map;
use overworld_map::*;

// mod worldgen;
// use worldgen::*;

mod events;
mod states;
use states::*;

use events::*;

// mod debug_plugin;
// use debug_plugin::*;

fn main() {
    App::new()
        .add_message::<MoveEvent>()
        .add_message::<MoveLegal>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Void destiny - The roguelike game!".into(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                present_mode: PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        })
        .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        //.add_plugin(DebugPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(OverWorldMapPlugin)
        //.add_plugins(WorldGenPlugin)
        .run();
}


