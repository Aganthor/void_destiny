use bevy::color::Srgba;
use bevy::prelude::Color;
use bevy::math::UVec2;

// Game window dimensions
pub const WINDOW_WIDTH: f32 = 1024.0;
pub const WINDOW_HEIGHT: f32 = 768.0;

// Chunks and Overworld map size
pub const CHUNK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
// Max size overwolrd size in chunks.
pub const OVERWORLD_SIZE: UVec2 = UVec2 { x: 64, y: 48 };
// Render chunk sizes are set to 4 render chunks per user specified chunk.
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: CHUNK_SIZE.x * 2,
    y: CHUNK_SIZE.y * 2,
};
pub const OVERWORLD_SIZE_WIDTH: u32 = 32;
pub const OVERWORLD_SIZE_HEIGHT: u32 = 24;

// Colors
pub const BG_COLOR: Color = Color::srgb(38. / 255., 20. / 255., 40. / 255.);