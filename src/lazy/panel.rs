use crate::draw::draw_rect;
use crate::lazy::{
    draw_debug_widget, Interactable, Renderable, WidgetData, WidgetTrait, DEBUG_WIDGETS,
};
use crate::widgets::Interaction;
use std::any::Any;

pub type Panel = WidgetData<PanelBase>;

#[derive(Default)]
pub struct PanelBase;

impl Panel {
    pub fn interact_t<T: 'static>(&mut self, expected_elems_of_this_type: usize) -> Vec<T> {
        let anys = self.interact();
        let mut typeds = Vec::new();
        for any in anys {
            let typed = any.downcast::<T>();
            if let Ok(typed) = typed {
                typeds.push(*typed);
            }
        }
        assert_eq!(
            typeds.len(), expected_elems_of_this_type,
            "The {} widgets inside this panel didn't provide the {} expected interactions of the given type. \
            Note that this function is doing blind downcasts for convenience because I can't come up with a better set of traits.\n\
            \tHint: Check that the panel contains the desired number of widgets.\n\
            \tHint: Check that the widgets inside the panel are returning the type you expect.",
            self.children().len(), expected_elems_of_this_type
        );
        typeds
    }
}
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
