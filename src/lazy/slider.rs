use crate::draw::{draw_rect, draw_rect_lines};
use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::lazy::{draw_debug_widget, Renderable, Style, WidgetData, WidgetTrait, DEBUG_WIDGETS};
use crate::widgets::Interaction;
use crate::SizeInPixels2d;
use macroquad::input::MouseButton;
use macroquad::math::vec2;
use macroquad::prelude::Rect;

pub type Slider = WidgetData<SliderBase>;
pub type RenderSlider = fn(widget: &Slider, interaction: Interaction);

pub struct SliderBase {
    pub min: f32,
    pub max: f32,
    pub current: f32,
    pub min_size: SizeInPixels2d,
    pub interaction: Interaction,

    pub input: Box<dyn InputTrait>,
    pub render_slider: RenderSlider,
}

impl Slider {
    pub fn new(style: Style, min: f32, max: f32, current: f32) -> Self {
        let font_size = style.font_size;
        let min_size = vec2(font_size * 15.0, font_size * 2.0);
        Self::new_generic(
            style,
            min,
            max,
            current,
            min_size,
            Box::new(InputMacroquad),
            render_slider,
        )
    }
    pub fn new_generic(
        style: Style,
        min: f32,
        max: f32,
        current: f32,
        min_size: SizeInPixels2d,
        input: Box<dyn InputTrait>,
        render_slider: RenderSlider,
    ) -> Self {
        let custom = SliderBase {
            min,
            max,
            current,
            min_size,
            interaction: Interaction::None,
            input,
            render_slider,
        };
        Self {
            pos: Default::default(),
            size: Some(min_size),
            style,
            custom,
            children: vec![],
        }
    }

    pub fn interact(&mut self) -> f32 {
        let max = self.custom.max;
        let min = self.custom.min;
        let current = self.custom.current;
        let rect = self.rect();
        let input = &self.custom.input;

        let mouse_pos = input.mouse_position();
        let range = max - min;
        let current_coef = (current - min) / range;
        let handle_width = self.handle_width();
        let mouse_coef =
            (mouse_pos - rect.point() - 0.5 * handle_width) / (rect.size() - handle_width);
        let (interaction, render_pos) = if rect.contains(mouse_pos) {
            if input.is_mouse_button_down(MouseButton::Left) {
                (Interaction::Pressing, mouse_coef.x)
            } else if input.is_mouse_button_released(MouseButton::Left) {
                (Interaction::Clicked, mouse_coef.x)
            } else {
                (Interaction::Hovered, current_coef)
            }
        } else {
            if self.custom.interaction.is_down() && input.is_mouse_button_down(MouseButton::Left) {
                (Interaction::Pressing, mouse_coef.x)
            } else {
                (Interaction::None, current_coef)
            }
        };
        self.custom.interaction = interaction;
        let render_pos = Some(render_pos.clamp(min, max));
        self.custom.current = range * render_pos.unwrap() + min;
        self.custom.current
    }
    fn handle_width(&self) -> f32 {
        self.rect().h * 0.62
    }
}

impl Renderable for Slider {
    fn render_interactive(&self, interaction: Interaction) {
        (self.custom.render_slider)(self, interaction);
        // draw_rect_lines(self.rect, 2.0, RED);
    }
}

fn render_slider(slider: &Slider, _interaction: Interaction) {
    let state_style = slider.style.coloring.choose(slider.custom.interaction);
    let center = slider.rect().center();
    let guide_height = slider.rect().h * 0.3;
    let guide_rect = Rect::new(
        slider.rect().x,
        center.y - guide_height * 0.5,
        slider.rect().w,
        guide_height,
    );
    draw_rect(guide_rect, slider.style.coloring.at_rest.bg_color);
    draw_rect_lines(guide_rect, 2.0, slider.style.coloring.at_rest.border_color);

    let width = slider.handle_width();
    let range = slider.custom.max - slider.custom.min;
    let render_pos = (slider.custom.current - slider.custom.min) / range;
    let handle_rect = Rect::new(
        (slider.rect().w - width) * render_pos + slider.rect().x,
        slider.rect().y,
        width,
        slider.rect().h,
    );
    draw_rect(handle_rect, state_style.bg_color);
    draw_rect_lines(handle_rect, 2.0, state_style.border_color);

    if unsafe { DEBUG_WIDGETS } {
        draw_debug_widget(slider);
    }
}
