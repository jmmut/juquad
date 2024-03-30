use macroquad::prelude::{
    draw_line, is_mouse_button_down, is_mouse_button_released, mouse_position, Color, MouseButton,
    Rect, Vec2, BLACK, DARKGRAY, GRAY, LIGHTGRAY, WHITE,
};

use crate::draw::draw_rect;
use crate::widgets::anchor::Anchor;
use crate::widgets::text::{DrawText, MeasureText, TextRect};

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InteractionStyle {
    pub at_rest: Color,
    pub hovered: Color,
    pub pressed: Color,
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Style {
    pub bg_color: InteractionStyle,
    pub text_color: InteractionStyle,
    pub border_color: InteractionStyle,
}
impl Style {
    pub const fn new() -> Style {
        Self {
            bg_color: InteractionStyle {
                at_rest: LIGHTGRAY,
                hovered: WHITE,
                pressed: GRAY,
            },
            text_color: InteractionStyle {
                at_rest: BLACK,
                hovered: BLACK,
                pressed: BLACK,
            },
            border_color: InteractionStyle {
                at_rest: DARKGRAY,
                hovered: Color::new(0.88, 0.88, 0.88, 1.00),
                pressed: DARKGRAY,
            },
        }
    }
}
impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}
pub struct Button {
    pub text_rect: TextRect,
    interaction: Interaction,
}

impl Button {
    pub fn new(text: &str, position_pixels: Anchor, font_size: f32) -> Self {
        Self::new_generic(text, position_pixels, font_size, macroquad::prelude::measure_text, macroquad::prelude::draw_text)
    }
    pub fn new_generic(text: &str, position_pixels: Anchor, font_size: f32, measure_text: MeasureText, draw_text: DrawText) -> Self {
        Self {
            text_rect: TextRect::new_generic(text, position_pixels, font_size, measure_text, draw_text),
            interaction: Interaction::None,
        }
    }

    pub fn rect(&self) -> Rect {
        self.text_rect.rect
    }
    pub fn interact(&mut self) -> Interaction {
        self.interaction = if self.text_rect.rect.contains(Vec2::from(mouse_position())) {
            if is_mouse_button_down(MouseButton::Left) {
                Interaction::Pressing
            } else if is_mouse_button_released(MouseButton::Left) {
                Interaction::Clicked
            } else {
                Interaction::Hovered
            }
        } else {
            Interaction::None
        };
        self.interaction
    }
    pub fn interaction(&self) -> Interaction {
        self.interaction
    }
    pub fn render(&self, style: &Style) {
        let (bg_color, text_color) = match self.interaction {
            Interaction::Clicked | Interaction::Pressing => {
                (style.bg_color.pressed, style.text_color.pressed)
            }
            Interaction::Hovered => (style.bg_color.hovered, style.text_color.hovered),
            Interaction::None => (style.bg_color.at_rest, style.text_color.at_rest),
        };
        let rect = self.text_rect.rect;
        draw_rect(rect, bg_color);
        draw_panel_border(rect, self.interaction, &style.border_color);
        self.text_rect.render_text(text_color);
    }
}

pub fn draw_panel_border(rect: Rect, interaction: Interaction, style: &InteractionStyle) {
    draw_windows_95_border(rect, interaction, style);
    // draw_rect_lines(rect, 2.0, BLACK);
}

// I swear I didn't realise what I was doing until I saw it running XD
pub fn draw_windows_95_border(rect: Rect, interaction: Interaction, style: &InteractionStyle) {
    let (border_color_high, border_color_low) = if interaction.is_down() {
        (style.pressed, style.hovered)
    } else {
        (style.hovered, style.pressed)
    };
    let left = rect.x;
    let right = rect.x + rect.w;
    let top = rect.y;
    let bottom = rect.y + rect.h;
    let thickness = 1.0;
    draw_line(left, top, right, top, thickness, border_color_high);
    draw_line(left, top, left, bottom, thickness, border_color_high);
    draw_line(left, bottom, right, bottom, thickness, border_color_low);
    draw_line(right, top, right, bottom, thickness, border_color_low);
}
