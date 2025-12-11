use crate::input::input_trait::InputTrait;
use crate::widgets::anchor::Anchor;
use macroquad::color::{BLACK, DARKGRAY, GRAY, LIGHTGRAY, WHITE};
use macroquad::prelude::{Color, MouseButton, Rect, Vec2};
use crate::{PositionInPixels2d, SizeInPixels2d};

pub mod anchor;
pub mod anchorer;
pub mod button;
pub mod button_group;
pub mod text;
pub mod texture_button;

pub trait Widget {
    fn rect(&self) -> Rect;
    fn size(&self) -> SizeInPixels2d {
        self.rect().size()
    }
    fn set_rect(&mut self, rect: Rect);
    fn reanchor(&mut self, anchor: Anchor) {
        let new_rect = anchor.get_top_left_pixel(self.size());
        self.set_pos(new_rect);
    }
    fn set_pos(&mut self, position: PositionInPixels2d) {
        let mut rect = self.rect();
        rect.move_to(position);
    }
    fn set_size(&mut self, size: SizeInPixels2d) {
        let mut rect = self.rect();
        rect.w = size.x;
        rect.h = size.y;
    }
}
pub struct StateStyle {
    pub bg_color: Color,
    pub text_color: Color,
    pub border_color: Color,
}
pub struct Style {
    pub at_rest: StateStyle,
    pub hovered: StateStyle,
    pub pressed: StateStyle,
}

impl Style {
    pub const fn new() -> Self {
        Self {
            at_rest: StateStyle {
                bg_color: LIGHTGRAY,
                text_color: BLACK,
                border_color: DARKGRAY,
            },
            hovered: StateStyle {
                bg_color: WHITE,
                text_color: BLACK,
                border_color: LIGHTGRAY,
            },
            pressed: StateStyle {
                bg_color: GRAY,
                text_color: WHITE,
                border_color: DARKGRAY,
            },
        }
    }
    pub fn choose(&self, interaction: Interaction) -> &StateStyle {
        match interaction {
            Interaction::Clicked | Interaction::Pressing => &self.pressed,
            Interaction::Hovered => &self.hovered,
            Interaction::None => &self.at_rest,
        }
    }
}
impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Interaction {
    Pressing,
    Clicked,
    Hovered,
    None,
}

impl Interaction {
    pub fn is_clicked(&self) -> bool {
        *self == Interaction::Clicked
    }

    pub fn is_down(&self) -> bool {
        *self == Interaction::Pressing || *self == Interaction::Clicked
    }

    pub fn is_hovered(&self) -> bool {
        *self == Interaction::Hovered
    }

    pub fn is_hovered_or_clicked(&self) -> bool {
        *self == Interaction::Hovered || *self == Interaction::Clicked
    }
}

pub fn interact(rect: Rect, input: &Box<dyn InputTrait>) -> Interaction {
    if rect.contains(input.mouse_position()) {
        if input.is_mouse_button_down(MouseButton::Left) {
            Interaction::Pressing
        } else if input.is_mouse_button_released(MouseButton::Left) {
            Interaction::Clicked
        } else {
            Interaction::Hovered
        }
    } else {
        Interaction::None
    }
}
