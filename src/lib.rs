use macroquad::prelude::Vec2;

pub mod draw;
pub mod fps;
pub mod lazy;
pub mod resource_loader;
pub mod texture_loader;
pub mod widgets;

pub mod elm {
    pub mod button;
    pub mod container;
    pub mod slider;
    pub mod style;
    pub mod text;
    pub mod widget;
}

pub mod input {
    pub mod input_macroquad;
    pub mod input_trait;
}

/// Represents an absolute position in pixels^2 from the top left screen corner, e.g. (800.0, 600.0).
pub type PositionInPixels2d = Vec2;

/// Represents a difference between 2 PixelPosition2, e.g. the size of a Rect.
pub type SizeInPixels2d = Vec2;
