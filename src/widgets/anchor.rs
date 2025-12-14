use crate::{PositionInPixels2d, SizeInPixels2d};
use macroquad::math::{Rect, Vec2};
use macroquad::prelude::vec2;

/// An Anchor helps you define positions for rectangles.
/// All f32 values are in pixels units, e.g. (800.0, 600.0). Top left is (0.0, 0.0).
#[derive(Copy, Clone, Debug)]
pub struct Anchor {
    horizontal: Horizontal,
    vertical: Vertical,
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Horizontal {
    Left,
    Center,
    Right,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Vertical {
    Top,
    Center,
    Bottom,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Layout {
    Horizontal {
        direction: Horizontal,
        alignment: Vertical,
    },
    Vertical {
        direction: Vertical,
        alignment: Horizontal,
    },
}
// pub struct Spot2d {
//     horizontal: Horizontal,
//     vertical: Vertical,
// }
// pub struct Spot2d {
//     horizontal: Spot,
//     vertical: Spot,
// }
pub enum Spot {
    Beginning,
    Center,
    End,
}
pub enum Sense {
    Forwards,
    Backwards,
}
pub enum Direction {
    Horizontal,
    Vertical,
}
// pub enum Layout {
//     Horizontal {
//         direction: Direction,
//         alignment: Vertical,
//     },
//     Vertical {
//         direction: Direction,
//         alignment: Horizontal,
//     },
// }
pub struct LayoutV2 {
    direction: Direction,
    sense: Sense,
    alignment: Spot,
}

use Horizontal as H;
use Vertical as V;

impl Anchor {
    pub fn new(horizontal: Horizontal, vertical: Vertical, x: f32, y: f32) -> Self {
        Self {
            horizontal,
            vertical,
            x,
            y,
        }
    }
    pub fn new_v(horizontal: Horizontal, vertical: Vertical, position: PositionInPixels2d) -> Self {
        Self {
            horizontal,
            vertical,
            x: position.x,
            y: position.y,
        }
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

    pub fn center_v(position: PositionInPixels2d) -> Self {
        Self::center(position.x, position.y)
    }
    pub fn center_left_v(position: PositionInPixels2d) -> Self {
        Self::center_left(position.x, position.y)
    }
    pub fn center_right_v(position: PositionInPixels2d) -> Self {
        Self::center_right(position.x, position.y)
    }
    pub fn top_left_v(position: PositionInPixels2d) -> Self {
        Self::top_left(position.x, position.y)
    }
    pub fn top_right_v(position: PositionInPixels2d) -> Self {
        Self::top_right(position.x, position.y)
    }
    pub fn top_center_v(position: PositionInPixels2d) -> Self {
        Self::top_center(position.x, position.y)
    }
    pub fn bottom_left_v(position: PositionInPixels2d) -> Self {
        Self::bottom_left(position.x, position.y)
    }
    pub fn bottom_right_v(position: PositionInPixels2d) -> Self {
        Self::bottom_right(position.x, position.y)
    }
    pub fn bottom_center_v(position: PositionInPixels2d) -> Self {
        Self::bottom_center(position.x, position.y)
    }

    pub fn offset(&mut self, x_diff: f32, y_diff: f32) {
        self.x += x_diff;
        self.y += y_diff;
    }
    pub fn offset_v(&mut self, diff: SizeInPixels2d) {
        self.offset(diff.x, diff.y)
    }

    pub fn get_top_left_pixel(&self, size: SizeInPixels2d) -> PositionInPixels2d {
        let x = match self.horizontal {
            Horizontal::Left => self.x,
            Horizontal::Center => self.x - size.x * 0.5,
            Horizontal::Right => self.x - size.x,
        };
        let y = match self.vertical {
            Vertical::Top => self.y,
            Vertical::Center => self.y - size.y * 0.5,
            Vertical::Bottom => self.y - size.y,
        };
        Vec2::new(x, y)
    }
    pub fn get_rect(&self, size: SizeInPixels2d) -> Rect {
        let pos = self.get_top_left_pixel(size);
        Rect::new(pos.x, pos.y, size.x, size.y)
    }

    pub fn next_to(other: Rect, layout: Layout, margin: f32) -> Anchor {
        match layout {
            Layout::Horizontal {
                direction,
                alignment,
            } => {
                if direction == Horizontal::Left {
                    Self::leftwards(other, alignment, margin)
                } else {
                    Self::rightwards(other, alignment, margin)
                }
            }
            Layout::Vertical {
                direction,
                alignment,
            } => {
                if direction == Vertical::Top {
                    Self::above(other, alignment, margin)
                } else {
                    Self::below(other, alignment, margin)
                }
            }
        }
    }

    pub fn below(other: Rect, alignment: Horizontal, pad_y: f32) -> Anchor {
        Self::below_v(other, alignment, vec2(0.0, pad_y))
    }
    pub fn above(other: Rect, alignment: Horizontal, pad_y: f32) -> Anchor {
        Self::above_v(other, alignment, vec2(0.0, pad_y))
    }
    pub fn rightwards(other: Rect, alignment: Vertical, pad_x: f32) -> Anchor {
        Self::rightwards_v(other, alignment, vec2(pad_x, 0.0))
    }
    pub fn leftwards(other: Rect, alignment: Vertical, pad_x: f32) -> Anchor {
        Self::leftwards_v(other, alignment, vec2(pad_x, 0.0))
    }
    pub fn below_v(other: Rect, alignment: Horizontal, pad: SizeInPixels2d) -> Anchor {
        let x = alignment.x_pad(other, pad);
        Anchor::new(alignment, Vertical::Top, x, other.bottom() + pad.y)
    }
    pub fn above_v(other: Rect, alignment: Horizontal, pad: SizeInPixels2d) -> Anchor {
        let x = alignment.x_pad(other, pad);
        Anchor::new(alignment, Vertical::Bottom, x, other.top() - pad.y)
    }
    pub fn rightwards_v(other: Rect, alignment: Vertical, pad: SizeInPixels2d) -> Anchor {
        let y = alignment.y_pad(other, pad);
        Anchor::new(Horizontal::Left, alignment, other.right() + pad.x, y)
    }
    pub fn leftwards_v(other: Rect, alignment: Vertical, pad: SizeInPixels2d) -> Anchor {
        let y = alignment.y_pad(other, pad);
        Anchor::new(Horizontal::Right, alignment, other.left() - pad.x, y)
    }
    // wrong: next_to is crossed, one dimension is inside the other outside
    // pub fn next_to_concrete(other: Rect, horiz: Horizontal, vert: Vertical, margin: SizeInPixels2d) -> Anchor {
    //     let x = horiz.x_margin(other, margin);
    //     let y = vert.y_margin(other, margin);
    //     Anchor::new(horiz, vert, x, y)
    // }

    pub fn inside(other: Rect, layout: Layout, pad: SizeInPixels2d) -> Anchor {
        match layout {
            Layout::Horizontal {
                direction,
                alignment,
            } => {
                // if direction == Horizontal::Left {
                //     Self::from_right(other, alignment, pad)
                // } else {
                //     Self::from_left(other, alignment, pad)
                // }
                Self::inside_concrete(other, direction.opposite(), alignment, pad)
            }
            Layout::Vertical {
                direction,
                alignment,
            } => {
                // if direction == Vertical::Top {
                //     Self::from_bottom(other, alignment, pad)
                // } else {
                //     Self::from_top(other, alignment, pad)
                // }
                Self::inside_concrete(other, alignment, direction.opposite(), pad)
            }
        }
    }

    pub fn from_top(other: Rect, alignment: Horizontal, pad: SizeInPixels2d) -> Anchor {
        Self::inside_concrete(other, alignment, Vertical::Top, pad)
    }
    pub fn from_bottom(other: Rect, alignment: Horizontal, pad: SizeInPixels2d) -> Anchor {
        Self::inside_concrete(other, alignment, Vertical::Bottom, pad)
    }
    pub fn from_left(other: Rect, alignment: Vertical, pad: SizeInPixels2d) -> Anchor {
        Self::inside_concrete(other, Horizontal::Left, alignment, pad)
    }
    pub fn from_right(other: Rect, alignment: Vertical, pad: SizeInPixels2d) -> Anchor {
        Self::inside_concrete(other, Horizontal::Right, alignment, pad)
    }
    pub fn inside_concrete(
        other: Rect,
        horiz: Horizontal,
        vert: Vertical,
        pad: SizeInPixels2d,
    ) -> Anchor {
        let x = horiz.x_pad(other, pad);
        let y = vert.y_pad(other, pad);
        Anchor::new(horiz, vert, x, y)
    }
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::top_left(0.0, 0.0)
    }
}
impl Layout {
    pub fn vertical(direction: Vertical, alignment: Horizontal) -> Self {
        Self::Vertical {
            direction,
            alignment,
        }
    }
    pub fn horizontal(direction: Horizontal, alignment: Vertical) -> Self {
        Self::Horizontal {
            direction,
            alignment,
        }
    }
    pub fn parallel(&self, v: Vec2) -> f32 {
        v[self.parallel_index()]
    }
    pub fn parallel_index(&self) -> usize {
        match self {
            Layout::Horizontal { .. } => 0,
            Layout::Vertical { .. } => 1,
        }
    }
    pub fn perpendicular(&self, v: Vec2) -> f32 {
        v[self.perpendicular_index()]
    }
    pub fn perpendicular_index(&self) -> usize {
        match self {
            Layout::Horizontal { .. } => 1,
            Layout::Vertical { .. } => 0,
        }
    }
    pub fn get_horiz(&self) -> Horizontal {
        match self {
            Layout::Horizontal { direction, .. } => *direction,
            Layout::Vertical { alignment, .. } => *alignment,
        }
    }
    pub fn get_vert(&self) -> Vertical {
        match self {
            Layout::Vertical { direction, .. } => *direction,
            Layout::Horizontal { alignment, .. } => *alignment,
        }
    }
    #[must_use]
    pub fn transpose(
        self,
        transform_h: fn(horiz: Horizontal) -> Vertical,
        transform_v: fn(vert: Vertical) -> Horizontal,
    ) -> Self {
        let horiz: Horizontal = transform_v(self.get_vert());
        let vert: Vertical = transform_h(self.get_horiz());
        self.opposite(horiz, vert)
        // match self {
        //     Layout::Horizontal {
        //         direction,
        //         alignment,
        //     } => {
        //         let direction = transform_h(direction);
        //         let alignment = transform_v(alignment);
        //         Layout::Vertical {
        //             direction,
        //             alignment,
        //         }
        //     }
        //     Layout::Vertical {
        //         direction,
        //         alignment,
        //     } => {
        //         let direction = transform_v(direction);
        //         let alignment = transform_h(alignment);
        //         Layout::Horizontal {
        //             direction,
        //             alignment,
        //         }
        //     }
        // }
    }
    #[must_use]
    pub fn opposite(self, horiz: Horizontal, vert: Vertical) -> Self {
        match self {
            Layout::Horizontal { .. } => Layout::Vertical {
                direction: vert,
                alignment: horiz,
            },
            Layout::Vertical { .. } => Layout::Horizontal {
                direction: horiz,
                alignment: vert,
            },
        }
    }
    pub fn get_alignment(&self) -> Spot {
        match self {
            Layout::Horizontal { alignment, .. } => (*alignment).into(),
            Layout::Vertical { alignment, .. } => (*alignment).into(),
        }
    }
    #[must_use]
    pub fn with_alignment(self, alignment: Spot) -> Self {
        match self {
            Layout::Horizontal { direction, .. } => Layout::Horizontal {
                alignment: alignment.into(),
                direction,
            },
            Layout::Vertical { direction, .. } => Layout::Vertical {
                alignment: alignment.into(),
                direction,
            },
        }
    }
    #[must_use]
    pub fn map_alignment(self, alignment_f: fn(Spot) -> Spot) -> Self {
        match self {
            Layout::Horizontal {
                alignment,
                direction,
            } => Layout::Horizontal {
                direction,
                alignment: alignment_f(alignment.into()).into(),
            },
            Layout::Vertical {
                alignment,
                direction,
            } => Layout::Vertical {
                direction,
                alignment: alignment_f(alignment.into()).into(),
            },
        }
    }
    pub fn get_direction(&self) -> Spot {
        match self {
            Layout::Horizontal { direction, .. } => (*direction).into(),
            Layout::Vertical { direction, .. } => (*direction).into(),
        }
    }
    #[must_use]
    pub fn with_direction(self, direction: Spot) -> Self {
        match self {
            Layout::Horizontal { alignment, .. } => Layout::Horizontal {
                direction: direction.into(),
                alignment,
            },
            Layout::Vertical { alignment, .. } => Layout::Vertical {
                direction: direction.into(),
                alignment,
            },
        }
    }
    #[must_use]
    pub fn map_direction(self, direction_f: fn(Spot) -> Spot) -> Self {
        match self {
            Layout::Horizontal {
                alignment,
                direction,
            } => Layout::Horizontal {
                direction: direction_f(direction.into()).into(),
                alignment,
            },
            Layout::Vertical {
                alignment,
                direction,
            } => Layout::Vertical {
                direction: direction_f(direction.into()).into(),
                alignment,
            },
        }
    }
    #[must_use]
    pub fn map_h_v(
        self,
        transform_h: fn(horiz: Horizontal) -> Horizontal,
        transform_v: fn(vert: Vertical) -> Vertical,
    ) -> Self {
        match self {
            Layout::Horizontal {
                direction,
                alignment,
            } => {
                let direction = transform_h(direction);
                let alignment = transform_v(alignment);
                Layout::Horizontal {
                    direction,
                    alignment,
                }
            }
            Layout::Vertical {
                direction,
                alignment,
            } => {
                let direction = transform_v(direction);
                let alignment = transform_h(alignment);
                Layout::Vertical {
                    direction,
                    alignment,
                }
            }
        }
    }
}
impl Horizontal {
    pub fn x_pad(self, other: Rect, pad: SizeInPixels2d) -> f32 {
        match self {
            Self::Left => other.left() + pad.x,
            Self::Center => other.center().x,
            Self::Right => other.right() - pad.x,
        }
    }
    pub fn x_margin(self, other: Rect, margin: SizeInPixels2d) -> f32 {
        self.x_pad(other, -margin)
    }
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Center => Self::Center,
            Self::Right => Self::Left,
        }
    }
    #[must_use]
    pub fn next(self) -> Horizontal {
        match self {
            Horizontal::Left => Horizontal::Center,
            Horizontal::Center => Horizontal::Right,
            Horizontal::Right => Horizontal::Left,
        }
    }
    #[must_use]
    pub fn rotate(self) -> Vertical {
        match self {
            Horizontal::Left => Vertical::Bottom,
            Horizontal::Center => Vertical::Center,
            Horizontal::Right => Vertical::Top,
        }
    }
}

impl Vertical {
    pub fn y_pad(self, other: Rect, pad: SizeInPixels2d) -> f32 {
        match self {
            Self::Top => other.top() + pad.y,
            Self::Center => other.center().y,
            Self::Bottom => other.bottom() - pad.y,
        }
    }
    pub fn y_margin(self, other: Rect, margin: SizeInPixels2d) -> f32 {
        self.y_pad(other, -margin)
    }
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Center => Self::Center,
            Self::Bottom => Self::Top,
        }
    }
    #[must_use]
    pub fn next(self) -> Vertical {
        match self {
            Vertical::Top => Vertical::Center,
            Vertical::Center => Vertical::Bottom,
            Vertical::Bottom => Vertical::Top,
        }
    }
    #[must_use]
    pub fn rotate(self) -> Horizontal {
        match self {
            Self::Top => Horizontal::Left,
            Self::Center => Horizontal::Center,
            Self::Bottom => Horizontal::Right,
        }
    }
}
impl From<Horizontal> for Spot {
    fn from(value: Horizontal) -> Self {
        match value {
            Horizontal::Left => Spot::Beginning,
            Horizontal::Center => Spot::Center,
            Horizontal::Right => Spot::End,
        }
    }
}
impl From<Vertical> for Spot {
    fn from(value: Vertical) -> Self {
        match value {
            Vertical::Top => Spot::Beginning,
            Vertical::Center => Spot::Center,
            Vertical::Bottom => Spot::End,
        }
    }
}
impl From<Spot> for Horizontal {
    fn from(value: Spot) -> Self {
        match value {
            Spot::Beginning => Horizontal::Left,
            Spot::Center => Horizontal::Center,
            Spot::End => Horizontal::Right,
        }
    }
}
impl From<Spot> for Vertical {
    fn from(value: Spot) -> Self {
        match value {
            Spot::Beginning => Vertical::Top,
            Spot::Center => Vertical::Center,
            Spot::End => Vertical::Bottom,
        }
    }
}
impl Spot {
    #[must_use]
    pub fn opposite(self) -> Self {
        match self {
            Self::Beginning => Self::End,
            Self::Center => Self::Center,
            Self::End => Self::Beginning,
        }
    }
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Beginning => Self::Center,
            Self::Center => Self::End,
            Self::End => Self::Beginning,
        }
    }
}
