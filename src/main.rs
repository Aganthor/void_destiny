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

//use benimator::AnimationPlugin;

mod constants;
use constants::*;

//mod player;
//use player::*;

mod map;
use map::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(BG_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "Void destiny - The roguelike game!".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        //.add_plugin(AnimationPlugin::default())
//        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

