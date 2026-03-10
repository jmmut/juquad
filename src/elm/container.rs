use crate::draw::draw_rect;
use crate::elm::style::Style;
use crate::elm::widget::{Interactable, Renderable, Widget, WidgetTrait, Widgets};
use crate::widgets::Interaction;

pub type Container<I> = Widget<(), I>;

pub fn container<I, Sty: Into<Style>>(style: Sty, children: Widgets<I>) -> Container<I> {
    Container::new(style, children)
}

impl<I> Container<I> {
    pub fn new<Sty: Into<Style>>(style: Sty, children: Widgets<I>) -> Container<I> {
        Widget {
            pos: Default::default(),
            size: Default::default(),
            style: style.into(),
            custom: (),
            children,
        }
    }
}

impl<I> Renderable for Container<I> {
    fn render_interactive(&self, parent_interaction: Interaction) {
        let widget = self;
        let state_style = widget.style().coloring.choose(parent_interaction);
        draw_rect(widget.rect(), state_style.bg_color);
        for child in &self.children {
            child.render_interactive(parent_interaction);
        }
    }
}
impl<I> Interactable<I> for Container<I> {
    fn interact(&mut self) -> Vec<I> {
        let mut messages = Vec::new();
        for child in &mut self.children {
            messages.extend(child.interact());
        }
        messages
    }
}
