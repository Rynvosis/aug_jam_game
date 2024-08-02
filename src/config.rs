use bevy::render::view::RenderLayers;

/// In-game resolution width.
pub const RES_WIDTH: u32 = 64;

/// In-game resolution height.
pub const RES_HEIGHT: u32 = 64;

pub const GRID_SIZE: f32 = 8.0;

/// Default render layers for pixel-perfect rendering.
/// You can skip adding this component, as this is the default.
pub const PIXEL_PERFECT_LAYERS: RenderLayers = RenderLayers::layer(0);

/// Render layers for high-resolution rendering.
pub const HIGH_RES_LAYERS: RenderLayers = RenderLayers::layer(1);
