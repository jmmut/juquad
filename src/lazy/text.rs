use crate::lazy::{AsWidget, Style, Widget, WidgetData};
use crate::widgets::text::draw_text;
use crate::SizeInPixels2d;
use macroquad::math::Vec2;
use macroquad::prelude::measure_text;

pub struct Text {
    pub widget_data: WidgetData,
    pub text: String,
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
        let size = size_text(text, widget_data.style());
        widget_data.set_size(size);
        Self {
            widget_data,
            text: text.to_string(),
        }
    }
    pub fn render(&self) {
        let pos = self.widget_data.pos();
        draw_text(
            &self.text,
            pos.x,
            pos.y,
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
    let pad = Vec2::new(reference_height, reference_height * 0.75);
    let text_dimensions = measure_text(text, style.font, style.font_size as u16, 1.0);

    let size = Vec2::new(
        (text_dimensions.width + pad.x * 2.0).round(),
        (reference_height + pad.y * 2.0).round(),
    );
    size
}
