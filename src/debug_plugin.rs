use bevy::prelude::*;
use bevy_ecs_tilemap::tiles::TileBundle;
use bevy_inspector_egui::quick::WorldInspectorPlugin;


use crate::player::Player;
use crate::map::MapSeed;//{MapSeed, TileCollider};

pub struct DebugPlugin;

// impl Plugin for DebugPlugin {
//     fn build(&self, app: &mut App) {
//         if cfg!(debug_assertions) {
//             app.add_plugin(WorldInspectorPlugin::new())
//                 //.register_inspectable::<Player>()
//                 //.register_inspectable::<MapSeed>();
//                 //.register_inspectable::<TileBundle>()
//                 //.register_inspectable::<TileCollider>();
//         }
//     }
// }
