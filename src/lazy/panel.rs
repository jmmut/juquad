use crate::draw::draw_rect;
use crate::lazy::{draw_debug_widget, Renderable, Widget, WidgetData};
use crate::widgets::Interaction;

pub type Panel = WidgetData<PanelBase>;

#[derive(Default)]
pub struct PanelBase;

impl Renderable for Panel {
    fn render_interactive(&self, _interaction: Interaction) {
        self.render()
    }
    fn render(&self) {
        let widget = self;
        draw_rect(widget.rect(), self.style().coloring.at_rest.bg_color);
        draw_debug_widget(widget);
    }
}
