use crate::{PixelPosition, SizeInPixels};
use macroquad::math::{Rect, Vec2};

/// An Anchor helps you define more easily define positions for rectangles.
/// All f32 values are in pixels units, e.g. (800.0, 600.0). Top left is (0.0, 0.0).
#[derive(Copy, Clone)]
pub struct Anchor {
    horizontal: Horizontal,
    vertical: Vertical,
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Horizontal {
    Left,
    Center,
    Right,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Vertical {
    Top,
    Center,
    Bottom,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Layout {
    Horizontal { direction: Horizontal, alignment: Vertical},
    Vertical { direction: Vertical, alignment: Horizontal},
}


use Horizontal as H;
use Vertical as V;

impl Anchor {
    pub fn new(horizontal: Horizontal, vertical: Vertical, x: f32, y: f32) -> Self {
        Self { horizontal, vertical, x, y }
    }
    pub fn new_v(horizontal: Horizontal, vertical: Vertical, position: PixelPosition) -> Self {
        Self { horizontal, vertical, x: position.x, y: position.y }
    }
    pub fn center(x: f32, y: f32) -> Self {
        Self::new(H::Center, V::Center, x, y)
    }
    pub fn center_left(x: f32, y: f32) -> Self {
        Self::new(H::Left, V::Center, x, y)
    }
    pub fn center_right(x: f32, y: f32) -> Self {
        Self::new(H::Right, V::Center, x, y)
    }
    pub fn top_left(x: f32, y: f32) -> Self {
        Self::new(H::Left, V::Top, x, y)
    }
    pub fn top_right(x: f32, y: f32) -> Self {
        Self::new(H::Right, V::Top, x, y)
    }
    pub fn top_center(x: f32, y: f32) -> Self {
        Self::new(H::Center, V::Top, x, y)
    }
    pub fn bottom_left(x: f32, y: f32) -> Self {
        Self::new(H::Left, V::Bottom, x, y)
    }
    pub fn bottom_right(x: f32, y: f32) -> Self {
        Self::new(H::Right, V::Bottom, x, y)
    }
    pub fn bottom_center(x: f32, y: f32) -> Self {
        Self::new(H::Center, V::Bottom, x, y)
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
        self.x += x_diff;
        self.y += y_diff;
    }
    pub fn offset_v(&mut self, diff: SizeInPixels) {
        self.offset(diff.x, diff.y)
    }
    pub fn get_top_left_pixel(&self, size: SizeInPixels) -> PixelPosition {
        let x = match self.horizontal {
            Horizontal::Left => self.x,
            Horizontal::Center => {self.x - size.x * 0.5}
            Horizontal::Right => {self.x - size.x}
        };
        let y = match self.vertical {
            Vertical::Top => self.y,
            Vertical::Center => self.y - size.y * 0.5,
            Vertical::Bottom => self.y - size.y,
        };
        Vec2::new(x, y)
    }
    pub fn get_rect(&self, size: SizeInPixels) -> Rect {
        let pos = self.get_top_left_pixel(size);
        Rect::new(pos.x, pos.y, size.x, size.y)
    }

    pub fn next_to(other: Rect, layout: Layout, pad: f32) -> Anchor {
        match layout {
            Layout::Horizontal{direction, alignment} => {
                if direction == Horizontal::Left {
                    Self::leftwards(other, alignment, pad)
                } else {
                    Self::rightwards(other, alignment, pad)
                }
            }
            Layout::Vertical{direction, alignment} => {
                if direction == Vertical::Top {
                    Self::above(other, alignment, pad)
                } else {
                    Self::below(other, alignment, pad)
                }
            }
        }
    }

    pub fn below(other: Rect, alignment: Horizontal, pad_y: f32) -> Anchor {
        let x = alignment.x(other);
        Anchor::new(alignment, Vertical::Top, x, other.bottom() + pad_y)
    }
    pub fn above(other: Rect, alignment: Horizontal, pad_y: f32) -> Anchor {
        let x = alignment.x(other);
        Anchor::new(alignment, Vertical::Bottom, x, other.top() - pad_y)
    }
    pub fn rightwards(other: Rect, alignment: Vertical, pad_x: f32) -> Anchor {
        let y = alignment.y(other);
        Anchor::new(Horizontal::Left, alignment, other.right() + pad_x, y)
    }
    pub fn leftwards(other: Rect, alignment: Vertical, pad_x: f32) -> Anchor {
        let y = alignment.y(other);
        Anchor::new(Horizontal::Right, alignment, other.left() - pad_x, y)
    }

}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::top_left(0.0, 0.0)
    }
}

impl Horizontal {
    pub fn x(self, other: Rect) -> f32 {
        match self {
            Self::Left => other.left(),
            Self::Center => other.center().x,
            Self::Right => other.right(),
        }
    }
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Center => Self::Center,
            Self::Right => Self::Left,
        }
    }
}

impl Vertical {
    pub fn y(self, other: Rect) -> f32 {
        match self {
            Self::Top => other.left(),
            Self::Center => other.center().y,
            Self::Bottom => other.right(),
        }
    }
    pub fn opposite(self) -> Self {
        match self {
            Self::Top => {Self::Bottom}
            Self::Center => {Self::Center}
            Self::Bottom => {Self::Top}
        }
    }
}
