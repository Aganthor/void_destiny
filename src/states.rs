use bevy::{dev_tools::states::*, prelude::*};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    GameRunning,
    DirtyMap,
}