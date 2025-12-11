use crate::draw::{draw_rect_lines, to_rect};
use crate::lazy::panel::{with_alpha, DEBUGGING_ALPHA, DEBUGGING_THICKNESS};
use crate::lazy::{AsWidget, Style, Widget, WidgetData};
use crate::widgets::text::draw_text;
use crate::SizeInPixels2d;
use macroquad::math::Vec2;
use macroquad::prelude::{measure_text, BLACK, BLUE, ORANGE};

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
static mut FIRST: bool = true;

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
        let pos = self.widget_data.pos();
        let size = self.widget_data.size();
        let rect = to_rect(pos, size);
        draw_rect_lines(
            rect,
            DEBUGGING_THICKNESS,
            with_alpha(BLACK, DEBUGGING_ALPHA),
        );
        let margin = self.widget_data.style.margin.vec2();
        let rect_margin = to_rect(pos - margin, size + margin * 2.0);
        draw_rect_lines(
            rect_margin,
            DEBUGGING_THICKNESS,
            with_alpha(BLUE, DEBUGGING_ALPHA),
        );
        let pad = self.widget_data.style.pad.vec2();
        let rect_pad = to_rect(pos + pad, size - pad * 2.0);
        draw_rect_lines(
            rect_pad,
            DEBUGGING_THICKNESS,
            with_alpha(ORANGE, DEBUGGING_ALPHA),
        );

        // draw_text() draws from the baseline of the text
        // https://en.wikipedia.org/wiki/Baseline_(typography)
        // I don't use self.text_dimensions.offset_y because that changes depending on the letters,
        // so I prefer an approximate distance that makes all buttons at the same baseline
        let approx_height_from_baseline_to_top = 0.85 * self.reference_height;
        let x = rect_pad.x.round();
        let y = (rect_pad.y + approx_height_from_baseline_to_top).round();
        let first = unsafe { FIRST };
        if first {
            println!("drawing text at {}, {}", x, y);
            unsafe {
                FIRST = false;
            }
        }
        draw_text(
            &self.text,
            x,
            y,
            self.widget_data.style.font_size,
            &self.widget_data.style.coloring.at_rest,
            self.widget_data.style.font,
        );
    }
}
//
// impl Widget for Text {
//     fn rect(&self) -> Rect {
//         self.rect
//     }
//
//     fn set_rect(&mut self, rect: Rect) {
//         todo!()
//     }
// }

fn size_text(text: &str, style: &Style) -> SizeInPixels2d {
    // font_size doesn't seem to be in pixels across fonts
    let reference_size = measure_text("Odp", style.font, style.font_size as u16, 1.0);
    let reference_height = reference_size.height;
    let text_dimensions = measure_text(text, style.font, style.font_size as u16, 1.0);

    let size = Vec2::new(text_dimensions.width.round(), reference_height.round());
    size
}
