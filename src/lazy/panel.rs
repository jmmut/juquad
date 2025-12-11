use crate::draw::draw_rect;
use crate::lazy::{AsWidget, Widget, WidgetData};

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
        )
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
