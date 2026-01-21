use bevy::math::UVec2;

// Game window dimensions
pub const WINDOW_WIDTH: u32 = 1024;
pub const WINDOW_HEIGHT: u32 = 768;

// Chunks and Overworld map size
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 4,
    y: CHUNK_SIZE.y * 4,
};
pub const OVERWORLD_SIZE_WIDTH: u32 = 320;
pub const OVERWORLD_SIZE_HEIGHT: u32 = 240;
