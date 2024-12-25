use macroquad::prelude::Vec2;

pub mod widgets;
pub mod draw;
pub mod fps;
pub mod texture_loader;

pub mod input {
    pub mod input_macroquad;
    pub mod input_trait;
}

/// Represents an absolute position in pixels from the top left screen corner, e.g. (800.0, 600.0).
pub type PixelPosition = Vec2;

/// Represents a difference between 2 PixelPositions, e.g. the size of a Rect.
pub type SizeInPixels = Vec2;
