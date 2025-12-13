use crate::draw::draw_rect;
use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::lazy::{draw_debug_widget, Renderable, Style, WidgetTrait, WidgetData};
use crate::widgets::button::draw_panel_border;
use crate::widgets::text::TextRect;
use crate::widgets::{interact, Interaction};

pub type RenderButton = fn(interaction: Interaction, text_rect: &TextRect, style: &Style);
pub type Widgets = Vec<Box<dyn Renderable>>;
pub type Button = WidgetData<ButtonBase>;

pub struct ButtonBase {
    pub interaction: Interaction,
    input: Box<dyn InputTrait>,
}
impl Default for ButtonBase {
    fn default() -> Self {
        Self {
            interaction: Interaction::None,
            input: Box::new(InputMacroquad),
        }
    }
}

impl Button {
    pub fn interact(&mut self) -> Interaction {
        self.custom.interaction = interact(self.rect(), &self.custom.input);
        self.custom.interaction
    }
    pub fn interaction(&self) -> Interaction {
        self.custom.interaction
    }
}

impl Renderable for Button {
    fn render_interactive(&self, _unused: Interaction) {
        let widget = self;

        let state_style = widget.style().coloring.choose(self.custom.interaction);
        draw_rect(widget.rect(), state_style.bg_color);
        draw_panel_border(widget.rect(), state_style);
        draw_debug_widget(widget);
        for child in self.children() {
            child.render_interactive(self.custom.interaction);
        }
    }
}
