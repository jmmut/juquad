use crate::{PixelPosition, SizeInPixels};
use macroquad::math::Rect;

/// An Anchor helps you define more easily define positions for rectangles.
/// All f32 values are in pixels units, e.g. (800.0, 600.0). Top left is (0.0, 0.0).
#[derive(Copy, Clone)]
pub enum Anchor {
    Center { x: f32, y: f32 },
    CenterLeft { x: f32, y: f32 },
    CenterRight { x: f32, y: f32 },
    TopLeft { x: f32, y: f32 },
    TopRight { x: f32, y: f32 },
    TopCenter { x: f32, y: f32 },
    BottomLeft { x: f32, y: f32 },
    BottomRight { x: f32, y: f32 },
    BottomCenter { x: f32, y: f32 },
}

impl Anchor {
    pub fn center(x: f32, y: f32) -> Self {
        Anchor::Center { x, y }
    }
    pub fn center_left(x: f32, y: f32) -> Self {
        Anchor::CenterLeft { x, y }
    }
    pub fn center_right(x: f32, y: f32) -> Self {
        Anchor::CenterRight { x, y }
    }
    pub fn top_left(x: f32, y: f32) -> Self {
        Anchor::TopLeft { x, y }
    }
    pub fn top_right(x: f32, y: f32) -> Self {
        Anchor::TopRight { x, y }
    }
    pub fn top_center(x: f32, y: f32) -> Self {
        Anchor::TopCenter { x, y }
    }
    pub fn bottom_left(x: f32, y: f32) -> Self {
        Anchor::BottomLeft { x, y }
    }
    pub fn bottom_right(x: f32, y: f32) -> Self {
        Anchor::BottomRight { x, y }
    }
    pub fn bottom_center(x: f32, y: f32) -> Self {
        Anchor::BottomCenter { x, y }
    }
    pub fn center_v(position: PixelPosition) -> Self {
        Self::center(position.x, position.y)
    }
    pub fn top_left_v(position: PixelPosition) -> Self {
        Self::top_left(position.x, position.y)
    }
    pub fn top_right_v(position: PixelPosition) -> Self {
        Self::top_right(position.x, position.y)
    }
    pub fn bottom_left_v(position: PixelPosition) -> Self {
        Self::bottom_left(position.x, position.y)
    }
    pub fn bottom_right_v(position: PixelPosition) -> Self {
        Self::bottom_right(position.x, position.y)
    }
    pub fn offset(&mut self, x_diff: f32, y_diff: f32) {
        match self {
            Anchor::Center { x, y }
            | Anchor::CenterLeft { x, y }
            | Anchor::CenterRight { x, y }
            | Anchor::TopLeft { x, y }
            | Anchor::TopRight { x, y }
            | Anchor::TopCenter { x, y }
            | Anchor::BottomLeft { x, y }
            | Anchor::BottomRight { x, y }
            | Anchor::BottomCenter { x, y } => {
                *x += x_diff;
                *y += y_diff
            }
        }
    }
    pub fn offset_v(&mut self, diff: SizeInPixels) {
        self.offset(diff.x, diff.y)
    }
    pub fn get_top_left_pixel(&self, size: SizeInPixels) -> PixelPosition {
        match *self {
            Anchor::Center { x, y } => PixelPosition::new(x - size.x * 0.5, y - size.y * 0.5),
            Anchor::CenterLeft { x, y } => PixelPosition::new(x, y - size.y * 0.5),
            Anchor::CenterRight { x, y } => PixelPosition::new(x - size.x, y - size.y * 0.5),
            Anchor::TopLeft { x, y } => PixelPosition::new(x, y),
            Anchor::TopRight { x, y } => PixelPosition::new(x - size.x, y),
            Anchor::TopCenter { x, y } => PixelPosition::new(x - size.x * 0.5, y),
            Anchor::BottomLeft { x, y } => PixelPosition::new(x, y - size.y),
            Anchor::BottomRight { x, y } => PixelPosition::new(x - size.x, y - size.y),
            Anchor::BottomCenter { x, y } => PixelPosition::new(x - size.x * 0.5, y - size.y),
        }
    }
    pub fn get_rect(&self, size: SizeInPixels) -> Rect {
        let pos = self.get_top_left_pixel(size);
        Rect::new(pos.x, pos.y, size.x, size.y)
    }
    pub fn from_below(other: Rect, x_diff: f32, y_diff: f32) -> Anchor {
        Anchor::top_left(other.x + x_diff, other.y + other.h + y_diff)
    }
    pub fn from_right(other: Rect, x_diff: f32, y_diff: f32) -> Anchor {
        Anchor::top_left(other.x + other.w + x_diff, other.y + y_diff)
    }

    pub fn center_below(other: Rect, x_diff: f32, y_diff: f32) -> Anchor {
        Anchor::top_center(other.x + other.w * 0.5 + x_diff, other.y + other.h + y_diff)
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::top_left(0.0, 0.0)
    }
}
