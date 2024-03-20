use macroquad::math::Rect;
use macroquad::prelude::Vec2;

#[derive(Copy, Clone)]
pub enum Anchor {
    Center { x: f32, y: f32 },
    TopLeft { x: f32, y: f32 },
    TopRight { x: f32, y: f32 },
    BottomLeft { x: f32, y: f32 },
    BottomRight { x: f32, y: f32 },
    // TODO: TopCenter, BottomCenter, CenterLeft, CenterRight
}

impl Anchor {
    pub fn center(x: f32, y: f32) -> Self {
        Anchor::Center { x, y }
    }
    pub fn top_left(x: f32, y: f32) -> Self {
        Anchor::TopLeft { x, y }
    }
    pub fn top_right(x: f32, y: f32) -> Self {
        Anchor::TopRight { x, y }
    }
    pub fn bottom_left(x: f32, y: f32) -> Self {
        Anchor::BottomLeft { x, y }
    }
    pub fn bottom_right(x: f32, y: f32) -> Self {
        Anchor::BottomRight { x, y }
    }
    pub fn center_v(position: Vec2) -> Self {
        Self::center(position.x, position.y)
    }
    pub fn top_left_v(position: Vec2) -> Self {
        Self::top_left(position.x, position.y)
    }
    pub fn top_right_v(position: Vec2) -> Self {
        Self::top_right(position.x, position.y)
    }
    pub fn bottom_left_v(position: Vec2) -> Self {
        Self::bottom_left(position.x, position.y)
    }
    pub fn bottom_right_v(position: Vec2) -> Self {
        Self::bottom_right(position.x, position.y)
    }
    pub fn offset(&mut self, x_diff: f32, y_diff: f32) {
        match self {
            Anchor::Center { x, y }
            | Anchor::TopLeft { x, y }
            | Anchor::TopRight { x, y }
            | Anchor::BottomLeft { x, y }
            | Anchor::BottomRight { x, y } => {
                *x += x_diff;
                *y += y_diff
            }
        }
    }
    pub fn get_top_left_pixel(&self, size: Vec2) -> Vec2 {
        match *self {
            Anchor::Center { x, y } => Vec2::new(x - size.x * 0.5, y - size.y * 0.5),
            Anchor::TopLeft { x, y } => Vec2::new(x, y),
            Anchor::TopRight { x, y } => Vec2::new(x - size.x, y),
            Anchor::BottomLeft { x, y } => Vec2::new(x, y - size.y),
            Anchor::BottomRight { x, y } => Vec2::new(x - size.x, y - size.y),
        }
    }
    pub fn from_below(other: Rect, x_diff: f32, y_diff: f32) -> Anchor {
        Anchor::top_left(other.x + x_diff, other.y + other.h + y_diff)
    }
    pub fn from_right(other: Rect, x_diff: f32, y_diff: f32) -> Anchor {
        Anchor::top_left(other.x + other.w + x_diff, other.y+ y_diff)
    }
}
