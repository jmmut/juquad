use crate::draw::draw_rect;
use crate::lazy::{
    draw_debug_widget, Interactable, Renderable, WidgetData, WidgetTrait, DEBUG_WIDGETS,
};
use crate::widgets::Interaction;
use std::any::Any;
use std::marker::PhantomData;

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
        if unsafe { DEBUG_WIDGETS } {
            draw_debug_widget(widget);
        }
        for child in self.children() {
            child.render();
        }
    }
}

impl Interactable for Panel {
    fn interact(&mut self) -> Vec<Box<dyn Any>> {
        let mut interactions = Vec::new();
        for child in self.children_mut() {
            interactions.extend(child.interact());
        }
        interactions
    }
}
