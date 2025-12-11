use crate::draw::{draw_rect, draw_rect_lines, to_rect};
use crate::lazy::{AsWidget, Widget, WidgetData};
use macroquad::prelude::{Color, SKYBLUE, YELLOW};

pub struct Panel {
    pub widget_data: WidgetData,
}
impl Default for Panel {
    fn default() -> Self {
        Self {
            widget_data: Default::default(),
        }
    }
}
impl Panel {
    pub fn new(widget_data: WidgetData) -> Self {
        Self { widget_data }
    }
    pub fn render(&self) {
        draw_rect(
            self.widget_data.rect(),
            self.style().coloring.at_rest.bg_color,
        );

        let pos = self.widget_data.pos();
        let size = self.widget_data.size();
        let margin = self.widget_data.style.margin.vec2();
        let rect_margin = to_rect(pos - margin, size + margin * 2.0);
        draw_rect_lines(
            rect_margin,
            DEBUGGING_THICKNESS,
            with_alpha(SKYBLUE, DEBUGGING_ALPHA),
        );
        let pad = self.widget_data.style.pad.vec2();
        let rect_pad = to_rect(pos + pad, size - pad * 2.0);
        draw_rect_lines(
            rect_pad,
            DEBUGGING_THICKNESS,
            with_alpha(YELLOW, DEBUGGING_ALPHA),
        );
    }
}
impl AsWidget for Panel {
    fn widget(&self) -> &dyn Widget {
        &self.widget_data
    }
    fn widget_mut(&mut self) -> &mut dyn Widget {
        &mut self.widget_data
    }
}

pub const DEBUGGING_ALPHA: f32 = 0.5;
pub const DEBUGGING_THICKNESS: f32 = 8.0;

pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha)
}
