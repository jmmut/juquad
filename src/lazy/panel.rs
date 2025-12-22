use crate::draw::draw_rect;
use crate::lazy::{
    draw_debug_widget, Interactable, Renderable, WidgetData, WidgetTrait, DEBUG_WIDGETS,
};
use crate::widgets::Interaction;
use std::any::Any;
use std::marker::PhantomData;

pub type Panel<Response> = WidgetData<PanelBase<Response>>;

pub struct PanelBase<Response> {
    phantom_data: PhantomData<Response>,
}
impl<Response> Default for PanelBase<Response> {
    fn default() -> Self {
        Self {
            phantom_data: Default::default(),
        }
    }
}

impl<Response: 'static> Panel<Response> {
    pub fn interact_tt<OneOffResponse: 'static>(
        &mut self,
        expected_elems_of_this_type: usize,
    ) -> Vec<OneOffResponse> {
        let anys = self.interact();
        let mut typeds = Vec::new();
        for any in anys {
            let typed = any.downcast::<OneOffResponse>();
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
    pub fn interact_t(&mut self, expected_elems_of_this_type: usize) -> Vec<Response> {
        self.interact_tt::<Response>(expected_elems_of_this_type)
    }
}
impl<Response> Renderable for Panel<Response> {
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

impl<Response> Interactable for Panel<Response> {
    fn interact(&mut self) -> Vec<Box<dyn Any>> {
        let mut interactions = Vec::new();
        for child in self.children_mut() {
            interactions.extend(child.interact());
        }
        interactions
    }
}
