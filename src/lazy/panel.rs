use crate::draw::draw_rect;
use crate::lazy::{draw_debug_widget, AsWidget, Widget, WidgetData};

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
        let widget = &self.widget_data;
        draw_rect(widget.rect(), self.style().coloring.at_rest.bg_color);
        draw_debug_widget(widget);
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
