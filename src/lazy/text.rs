use crate::lazy::{
    add_contour, draw_debug_widget, Interactable, Renderable, Style, WidgetData, WidgetTrait,
    DEBUG_WIDGETS,
};
use crate::widgets::text::{draw_text_v, MeasureText};
use crate::widgets::Interaction;
use crate::SizeInPixels2d;
use macroquad::math::Vec2;
use macroquad::prelude::vec2;

pub type Text = WidgetData<TextBase>;
pub type RenderText = fn(widget: &Text, interaction: Interaction);

pub struct TextBase {
    pub text: String,
    pub reference_height: f32,
    pub render_text: RenderText,
}
impl Text {
    pub fn new(style: &Style, text: &str) -> Self {
        Self::new_generic(style, text, macroquad::text::measure_text, render_text)
    }
    pub fn new_generic(
        style: &Style,
        text: &str,
        measure_text: MeasureText,
        render_text: RenderText,
    ) -> Self {
        let mut size = size_text(text, &style, measure_text);
        let reference_height = size.y;
        size += 2.0 * style.pad.vec2();
        let custom = TextBase {
            text: text.to_string(),
            reference_height,
            render_text,
        };
        Self {
            pos: Default::default(),
            size: Some(size),
            style: style.clone(),
            custom,
            children: Vec::new(),
        }
    }
    pub fn render(&self) {
        self.render_interactive(Interaction::None)
    }
}
impl Renderable for Text {
    fn render_interactive(&self, interaction: Interaction) {
        (self.custom.render_text)(self, interaction);
    }
}
impl Interactable for Text {}

pub fn size_text(text: &str, style: &Style, measure_text: MeasureText) -> SizeInPixels2d {
    // font_size doesn't seem to be in pixels across fonts
    let reference_size = measure_text("Odp", style.font.as_ref(), style.font_size as u16, 1.0);
    let reference_height = reference_size.height;
    let text_dimensions = measure_text(text, style.font.as_ref(), style.font_size as u16, 1.0);

    let size = Vec2::new(text_dimensions.width.round(), reference_height.round());
    size
}
pub fn render_text(widget: &Text, interaction: Interaction) {
    let reference_height = widget.custom.reference_height;
    let text = &widget.custom.text;
    let rect_pad = add_contour(widget.rect(), -widget.style.pad.vec2());
    if unsafe { DEBUG_WIDGETS } {
        draw_debug_widget(widget);
    }

    // draw_text() draws from the baseline of the text
    // https://en.wikipedia.org/wiki/Baseline_(typography)
    // I don't use self.text_dimensions.offset_y because that changes depending on the letters,
    // so I prefer an approximate distance that makes all buttons at the same baseline
    let approx_height_from_baseline_to_top = 0.85 * reference_height;
    let x = rect_pad.x.round();
    let y = (rect_pad.y + approx_height_from_baseline_to_top).round();
    // print_debug_pos(x, y);
    draw_text_v(
        text,
        vec2(x, y),
        widget.style.font_size,
        &widget.style.coloring.choose(interaction),
        widget.style.font.as_ref(),
    );
}

static mut FIRST: bool = true;

#[allow(unused)]
fn print_debug_pos(x: f32, y: f32) {
    let first = unsafe { FIRST };
    if first {
        println!("drawing text at {}, {}", x, y);
        unsafe {
            FIRST = false;
        }
    }
}
