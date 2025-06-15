use crate::draw::{draw_rect, draw_rect_lines};
use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::widgets::anchor::Anchor;
use crate::widgets::text::{MeasureText, TextRect};
use crate::widgets::{interact, Interaction, Style, Widget};
use macroquad::prelude::{
    draw_line, Rect,
};
use macroquad::text::Font;

pub type RenderButton = fn(interaction: Interaction, text_rect: &TextRect, style: &Style);

pub struct Button {
    pub text_rect: TextRect,
    interaction: Interaction,
    input: Box<dyn InputTrait>,
}
impl Widget for Button {
    fn rect(&self) -> Rect {
        Button::rect(self)
    }
    fn rect_mut(&mut self) -> &mut Rect {
        Button::rect_mut(self)
    }
}

impl Button {
    pub fn new(text: &str, position_pixels: Anchor, font_size: f32) -> Self {
        Self::new_from_text_rect(TextRect::new(text, position_pixels, font_size))
    }
    pub fn new_generic(
        text: &str,
        position_pixels: Anchor,
        font_size: f32,
        font: Option<Font>,
        measure_text: MeasureText,
        input: Box<dyn InputTrait>,
    ) -> Self {
        Self::new_from_text_rect_generic(
            TextRect::new_generic(
                text,
                position_pixels,
                font_size,
                font,
                measure_text,
            ),
            input,
        )
    }
    pub fn new_from_text_rect(text_rect: TextRect) -> Self {
        Self::new_from_text_rect_generic(text_rect, Box::new(InputMacroquad))
    }
    pub fn new_from_text_rect_generic(
        text_rect: TextRect,
        input: Box<dyn InputTrait>,
    ) -> Self {
        Self {
            text_rect,
            interaction: Interaction::None,
            input,
        }
    }

    pub fn rect(&self) -> Rect {
        self.text_rect.rect
    }
    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.text_rect.rect
    }
    pub fn interact(&mut self) -> Interaction {
        self.interaction = interact(self.rect(), &self.input);
        self.interaction
    }
    pub fn interaction(&self) -> Interaction {
        self.interaction
    }
    pub fn render_default(&self, style: &Style) {
        self.render(style, render_button);
    }
    pub fn render(&self, style: &Style, render_button: RenderButton) {
        render_button(self.interaction, &self.text_rect, style);
    }
}

pub fn render_button(interaction: Interaction, text_rect: &TextRect, style: &Style) {
    let state_style = style.choose(interaction);
    let rect = text_rect.rect;
    draw_rect(rect, state_style.bg_color);
    draw_panel_border(rect, interaction, &style);
    text_rect.render_text(state_style.text_color);
}

pub fn draw_panel_border(rect: Rect, interaction: Interaction, style: &Style) {
    // draw_windows_95_border(rect, interaction, style);
    draw_rect_lines(rect, 2.0, style.choose(interaction).border_color);
}

// I swear I didn't realise what I was doing until I saw it running XD
pub fn draw_windows_95_border(rect: Rect, interaction: Interaction, style: &Style) {
    let (border_color_high, border_color_low) = if interaction.is_down() {
        (style.pressed.border_color, style.hovered.border_color)
    } else {
        (style.hovered.border_color, style.pressed.border_color)
    };
    let left = rect.x + 1.0;
    let right = rect.x + rect.w;
    let top = rect.y;
    let bottom = rect.y + rect.h - 1.0;
    let thickness = 1.0;
    draw_line(left, top, right, top, thickness, border_color_high);
    draw_line(left, top, left, bottom, thickness, border_color_high);
    draw_line(left, bottom, right, bottom, thickness, border_color_low);
    draw_line(right, top, right, bottom, thickness, border_color_low);
}
