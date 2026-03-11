use crate::draw::{draw_rect, draw_rect_lines};
use crate::elm::style::Style;
use crate::elm::widget::{
    Interactable, Renderable, RenderableWidget, Widget, WidgetTrait, Widgets,
};
use crate::widgets::Interaction;

pub type Container<I> = Widget<(), I>;

pub fn container<I: Clone + 'static, Sty: Into<Style>>(
    style: Sty,
    children: Widgets<I>,
) -> Container<I> {
    Container::new_raw(style, children)
}

impl<I: Clone + 'static> Container<I> {
    pub fn new<Sty: Into<Style>>(style: Sty, children: Widgets<I>) -> Box<dyn RenderableWidget<I>> {
        Box::new(Self::new_raw(style, children))
    }
    pub fn new_raw<Sty: Into<Style>>(style: Sty, children: Widgets<I>) -> Container<I> {
        Widget {
            pos: Default::default(),
            size: Default::default(),
            style: style.into(),
            custom: (),
            children,
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

impl<I> Renderable for Container<I> {
    fn render_interactive(&self, parent_interaction: Interaction) {
        let widget = self;
        let state_style = widget.style().coloring.choose(parent_interaction);
        draw_rect(widget.rect(), state_style.bg_color);
        draw_rect_lines(
            widget.rect(),
            widget.style.border * 2.0,
            state_style.border_color,
        );
        for child in &self.children {
            child.render_interactive(parent_interaction);
        }
    }
}
