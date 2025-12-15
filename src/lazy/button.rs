use crate::draw::draw_rect;
use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::lazy::{
    draw_debug_widget, Interactable, Renderable, Style, WidgetData, WidgetTrait, Widgets,
    DEBUG_WIDGETS,
};
use crate::widgets::button::draw_panel_border;
use crate::widgets::{interact, Interaction};
use std::any::Any;

pub type Button = WidgetData<ButtonBase>;
pub type RenderButton = fn(widget: &Button, interaction: Interaction);

pub struct ButtonBase {
    pub interaction: Interaction,
    pub input: Box<dyn InputTrait>,
    pub render_button: RenderButton,
}
impl Default for ButtonBase {
    fn default() -> Self {
        Self {
            interaction: Interaction::None,
            input: Box::new(InputMacroquad),
            render_button: render_interactive,
        }
    }
}

impl Button {
    // pub fn new_text()
    pub fn new_generic(
        style: Style,
        input: Box<dyn InputTrait>,
        render_button: RenderButton,
        children: Widgets,
    ) -> Self {
        let custom = ButtonBase {
            interaction: Interaction::None,
            input,
            render_button,
        };
        Self {
            pos: Default::default(),
            size: None,
            style,
            custom,
            children,
        }
    }
    pub fn interact(&mut self) -> Interaction {
        self.custom.interaction = interact(self.rect(), &self.custom.input);
        self.custom.interaction
    }
    pub fn interaction(&self) -> Interaction {
        self.custom.interaction
    }
}

impl Renderable for Button {
    fn render_interactive(&self, interaction: Interaction) {
        (self.custom.render_button)(self, interaction)
    }
}
impl Interactable for Button {
    fn interact(&mut self) -> Vec<Box<dyn Any>> {
        vec![Box::new(self.interact())]
    }
}

fn render_interactive(widget: &Button, _unused: Interaction) {
    let state_style = widget.style().coloring.choose(widget.custom.interaction);
    draw_rect(widget.rect(), state_style.bg_color);
    draw_panel_border(widget.rect(), state_style);
    if unsafe { DEBUG_WIDGETS } {
        draw_debug_widget(widget);
    }
    for child in widget.children() {
        child.render_interactive(widget.custom.interaction);
    }
}
