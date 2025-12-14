use crate::input::input_trait::InputTrait;
use crate::widgets::anchor::Anchor;
use crate::{PositionInPixels2d, SizeInPixels2d};
use macroquad::color::BLACK;
use macroquad::color_u8;
use macroquad::prelude::{Color, MouseButton, Rect};

pub mod anchor;
pub mod anchorer;
pub mod button;
pub mod button_group;
pub mod text;
pub mod texture_button;

pub const fn from_hexes<const N: usize>(hexes: &[u32]) -> [Color; N] {
    let mut colors = [BLACK; N];
    let mut i = 0;
    while i < colors.len() {
        colors[i] = from_hex(hexes[i]);
        i += 1;
    }
    colors
}

const fn from_hex(hex: u32) -> Color {
    color_u8!(hex / 0x10000, hex / 0x100 % 0x100, hex % 0x100, 255)
}

pub const WHITE_BLUE_BACKGROUND: Color = from_hex(0xF5F6FB);
pub const LIGHT_BLUE_BACKGROUND: Color = from_hex(0xA7B5DD);
pub const MID_BLUE_BACKGROUND: Color = from_hex(0x5D78B2);
pub const DARK_BLUE_BACKGROUND: Color = from_hex(0x284168);
pub const BLACK_BLUE_BACKGROUND: Color = from_hex(0x030913);

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
        self.set_rect(rect);
    }
    fn set_size(&mut self, size: SizeInPixels2d) {
        let mut rect = self.rect();
        rect.w = size.x;
        rect.h = size.y;
        self.set_rect(rect)
    }
}
#[derive(Copy, Clone)]
pub struct StateStyle {
    pub bg_color: Color,
    pub text_color: Color,
    pub border_color: Color,
}

#[derive(Copy, Clone)]
pub struct Style {
    pub at_rest: StateStyle,
    pub hovered: StateStyle,
    pub pressed: StateStyle,
}

impl Style {
    pub const fn new() -> Self {
        Self {
            at_rest: StateStyle {
                bg_color: LIGHT_BLUE_BACKGROUND,
                text_color: BLACK_BLUE_BACKGROUND,
                border_color: DARK_BLUE_BACKGROUND,
            },
            hovered: StateStyle {
                bg_color: WHITE_BLUE_BACKGROUND,
                text_color: BLACK_BLUE_BACKGROUND,
                border_color: LIGHT_BLUE_BACKGROUND,
            },
            pressed: StateStyle {
                bg_color: MID_BLUE_BACKGROUND,
                text_color: WHITE_BLUE_BACKGROUND,
                border_color: DARK_BLUE_BACKGROUND,
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
