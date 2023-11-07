use macroquad::prelude::{
    draw_line, draw_rectangle, is_mouse_button_down, is_mouse_button_released, mouse_position,
    Color, MouseButton, Rect, Vec2, BLACK, DARKGRAY, GRAY, LIGHTGRAY, WHITE,
};

use crate::widgets::anchor::Anchor;
use crate::widgets::text::TextRect;

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

pub struct Button {
    pub text_rect: TextRect,
    color: Option<Color>,
    interaction: Interaction,
}

impl Button {
    pub fn new(text: &str, position_pixels: Anchor, font_size: f32) -> Self {
        Self {
            text_rect: TextRect::new(text, position_pixels, font_size),
            color: None,
            interaction: Interaction::Pressing,
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
    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.color = Some(color);
        self
    }
    pub fn render(&self) {
        let color = match self.interaction {
            Interaction::Clicked | Interaction::Pressing => GRAY,
            Interaction::Hovered => WHITE,
            Interaction::None => self.color.unwrap_or(LIGHTGRAY),
        };
        let rect = self.text_rect.rect;
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        draw_panel_border(rect, self.interaction);
        self.text_rect.render_text(BLACK);
    }
}

pub fn draw_panel_border(rect: Rect, interaction: Interaction) {
    draw_windows_95_border(rect, interaction);
    // draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, BLACK);
}

// I swear I didn't realise what I was doing until I saw it running XD
pub fn draw_windows_95_border(rect: Rect, interaction: Interaction) {
    let lighter_gray = Color::new(0.88, 0.88, 0.88, 1.00);
    let (border_color_high, border_color_low) = if interaction.is_down() {
        // (BLACK, WHITE)
        (DARKGRAY, lighter_gray)
    } else {
        // (WHITE, BLACK)
        (lighter_gray, DARKGRAY)
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
