use crate::lazy::{
    add_contour, draw_debug_widget, AsWidget, Renderable, RenderableWidget, Style, Widget,
    WidgetData,
};
use crate::widgets::text::draw_text_v;
use crate::widgets::Interaction;
use crate::SizeInPixels2d;
use macroquad::math::Vec2;
use macroquad::prelude::{measure_text, vec2};

pub struct Text {
    pub widget_data: WidgetData,
    pub text: String,
    pub reference_height: f32,
}
impl AsWidget for Text {
    fn widget(&self) -> &dyn Widget {
        &self.widget_data
    }
    fn widget_mut(&mut self) -> &mut dyn Widget {
        &mut self.widget_data
    }
}
impl Text {
    pub fn new(text: &str, mut widget_data: WidgetData) -> Self {
        let mut size = size_text(text, widget_data.style());
        let reference_height = size.y;
        size += 2.0 * widget_data.style.pad.vec2();
        widget_data.set_size(size);
        Self {
            widget_data,
            text: text.to_string(),
            reference_height,
        }
    }
    pub fn render(&self) {
        self.render_interactive(Interaction::None)
    }
}
impl Renderable for Text {
    fn render_interactive(&self, interaction: Interaction) {
        render_text(
            &self.widget_data,
            self.reference_height,
            &self.text,
            interaction,
        );
    }
}
impl RenderableWidget for Text {}

fn size_text(text: &str, style: &Style) -> SizeInPixels2d {
    // font_size doesn't seem to be in pixels across fonts
    let reference_size = measure_text("Odp", style.font, style.font_size as u16, 1.0);
    let reference_height = reference_size.height;
    let text_dimensions = measure_text(text, style.font, style.font_size as u16, 1.0);

    let size = Vec2::new(text_dimensions.width.round(), reference_height.round());
    size
}
fn render_text(widget: &WidgetData, reference_height: f32, text: &str, interaction: Interaction) {
    let rect_pad = add_contour(widget.rect(), -widget.style.pad.vec2());
    draw_debug_widget(widget);

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
        widget.style.font,
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
