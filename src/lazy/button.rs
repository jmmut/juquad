use crate::draw::draw_rect;
use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::lazy::{
    draw_debug_widget, AsWidget, Renderable, RenderableWidget, Style, Widget, WidgetData,
};
use crate::widgets::button::{draw_panel_border, render_button};
use crate::widgets::text::TextRect;
use crate::widgets::{interact, Interaction};

pub type RenderButton = fn(interaction: Interaction, text_rect: &TextRect, style: &Style);
pub type Widgets = Vec<Box<dyn RenderableWidget>>;

pub struct Button {
    pub widget_data: WidgetData,
    pub interaction: Interaction,
    input: Box<dyn InputTrait>,
    pub children: Widgets,
}
impl Default for Button {
    fn default() -> Self {
        Self {
            widget_data: Default::default(),
            interaction: Interaction::None,
            input: Box::new(InputMacroquad),
            children: Vec::new(),
        }
    }
}
impl Button {
    pub fn new(widget_data: WidgetData, children: Widgets) -> Self {
        Self {
            widget_data,
            children,
            ..Default::default()
        }
    }
    pub fn interact(&mut self) -> Interaction {
        self.interaction = interact(self.rect(), &self.input);
        self.interaction
    }
    pub fn interaction(&self) -> Interaction {
        self.interaction
    }

    pub fn render(&self) {
        let widget = &self.widget_data;

        let state_style = widget.style.coloring.choose(self.interaction);
        draw_rect(widget.rect(), state_style.bg_color);
        draw_panel_border(widget.rect(), state_style);
        draw_debug_widget(widget);
        for child in &self.children {
            child.render_interactive(self.interaction);
        }
    }
}
impl AsWidget for Button {
    fn widget(&self) -> &dyn Widget {
        &self.widget_data
    }
    fn widget_mut(&mut self) -> &mut dyn Widget {
        &mut self.widget_data
    }
}
