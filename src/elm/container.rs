use crate::draw::draw_rect;
use crate::elm::style::Style;
use crate::elm::text::Text;
use crate::elm::widget::{
    Interactable, Renderable, RenderableWidget, ToWidgets, Widget, WidgetTrait, Widgets, W,
};
use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::widgets::button::draw_panel_border;
use crate::widgets::{interact, Interaction};

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
pub trait ContainerTuple<W, I> {
    fn new_tuples<Sty: Into<Style>>(style: Sty, children: W) -> Container<I>;
}

impl<I, A: RenderableWidget<I> + 'static, B: RenderableWidget<I> + 'static>
    ContainerTuple<(A, B), I> for Container<I>
{
    fn new_tuples<Sty: Into<Style>>(style: Sty, children: (A, B)) -> Container<I> {
        Self::new(style, W::widgets(children))
    }
}
impl<
        I,
        A: RenderableWidget<I> + 'static,
        B: RenderableWidget<I> + 'static,
        C: RenderableWidget<I> + 'static,
    > ContainerTuple<(A, B, C), I> for Container<I>
{
    fn new_tuples<Sty: Into<Style>>(style: Sty, children: (A, B, C)) -> Container<I> {
        Self::new(style, W::widgets(children))
    }
}
impl<
        I,
        A: RenderableWidget<I> + 'static,
        B: RenderableWidget<I> + 'static,
        C: RenderableWidget<I> + 'static,
        D: RenderableWidget<I> + 'static,
    > ContainerTuple<(A, B, C, D), I> for Container<I>
{
    fn new_tuples<Sty: Into<Style>>(style: Sty, children: (A, B, C, D)) -> Container<I> {
        Self::new(style, W::widgets(children))
    }
}
// pub fn container2<Ws, I, Sty: Into<Style>>(style: Sty, children: Ws) -> Container<I> {
//     Widget {
//         pos: Default::default(),
//         size: Default::default(),
//         style: style.into(),
//         custom: (),
//         children: W::w(children),
//     }
// }

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
