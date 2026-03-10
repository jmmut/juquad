use crate::draw::to_rect;
use crate::elm::style::Style;
use crate::widgets::anchor::{Anchor, Layout};
use crate::widgets::Interaction;
use crate::{PositionInPixels2d, SizeInPixels2d};
use macroquad::math::{vec2, Rect};
use std::any::Any;

pub type Widgets<I> = Vec<Box<dyn RenderableWidget<I>>>;
// pub type WidgetsViewMut<'a, 'b> = Vec<&'a mut (dyn RenderableWidget + 'b)>;
pub type WidgetsViewMut<'a, I> = Vec<&'a mut dyn RenderableWidget<I>>;
pub type WidgetsView<'a, I> = Vec<&'a dyn RenderableWidget<I>>;

pub trait WidgetTrait<I> {
    fn rect(&self) -> Rect {
        let pos = self.pos();
        let size = self.size();
        Rect::new(pos.x, pos.y, size.x, size.y)
    }
    fn size(&self) -> SizeInPixels2d;
    fn pos(&self) -> PositionInPixels2d;
    fn set_rect(&mut self, rect: Rect) {
        self.set_pos(rect.point());
        self.set_size(rect.size())
    }
    fn set_pos(&mut self, position: PositionInPixels2d);
    fn set_size(&mut self, size: SizeInPixels2d);
    fn reanchor(&mut self, anchor: Anchor) {
        let new_rect = anchor.get_top_left_pixel(self.size());
        self.set_pos(new_rect);
    }
    fn style(&self) -> &Style;

    fn children(&self) -> WidgetsView<'_, I>;
    fn children_mut(&mut self) -> WidgetsViewMut<'_, I>;
}

pub trait Renderable {
    /// parent_interaction is the state of the parent, e.g. if the nested text inside a button
    /// needs to be rendered differently if the button is clicked. If a widget provides its own
    /// interaction, this parameter can be ignored.
    fn render_interactive(&self, parent_interaction: Interaction);
    fn render(&self) {
        self.render_interactive(Interaction::None)
    }
    // fn render_generic?
}

pub trait Interactable<I> {
    fn interact(&mut self) -> Vec<I> {
        Vec::new()
    }
}

pub trait RenderableWidget<I>: Renderable + WidgetTrait<I> + Interactable<I> {}

impl<I, T: Renderable + WidgetTrait<I> + Interactable<I>> RenderableWidget<I> for T {}

pub struct Widget<Custom, I> {
    pub custom: Custom,
    pub pos: PositionInPixels2d,
    pub size: Option<SizeInPixels2d>,
    pub style: Style,
    pub children: Widgets<I>,
}

impl<C, I> WidgetTrait<I> for Widget<C, I> {
    fn size(&self) -> SizeInPixels2d {
        if let Some(size) = self.size {
            size
        } else {
            SizeInPixels2d::default()
        }
    }
    fn pos(&self) -> PositionInPixels2d {
        self.pos
    }
    fn set_pos(&mut self, position: PositionInPixels2d) {
        self.pos = position;
    }
    fn set_size(&mut self, size: SizeInPixels2d) {
        if self.size.is_none() {
            self.size = Some(size);
        }
    }
    fn style(&self) -> &Style {
        &self.style
    }

    fn children(&self) -> WidgetsView<'_, I> {
        self.children
            .iter()
            .map(|child| child.as_ref() as &dyn RenderableWidget<I>)
            .collect()
    }
    fn children_mut(&mut self) -> WidgetsViewMut<'_, I> {
        self.children
            .iter_mut()
            .map(|child| child.as_mut() as &mut dyn RenderableWidget<I>)
            .collect()
    }
}

pub fn compute_layout<I>(ui: &mut dyn WidgetTrait<I>, rect: Rect, layout: Layout) {
    set_sizes(ui);
    let anchor = Anchor::inside(rect, layout, vec2(0.0, 0.0));
    set_positions(ui, anchor);
}

pub fn set_sizes<I>(node: &mut dyn WidgetTrait<I>) {
    let mut accumulated_size = SizeInPixels2d::new(0.0, 0.0);
    let style = node.style();
    let parallel = style.layout.parallel_index();
    let perpendicular = style.layout.perpendicular_index();
    for child in node.children_mut() {
        set_sizes(child);
        let size = child.size();
        let margin = child.style().margin.vec2();
        accumulated_size[parallel] += size[parallel] + 2.0 * margin[parallel];
        accumulated_size[perpendicular] =
            accumulated_size[perpendicular].max(size[perpendicular] + 2.0 * margin[perpendicular]);
    }
    accumulated_size += 2.0 * node.style().pad.vec2();
    node.set_size(accumulated_size);
    // println!(
    //     "size: {}, margin: {}, pad: {}",
    //     node.node().size(),
    //     node.node().style().margin.vec2(),
    //     node.node().style().pad.vec2()
    // );
}
pub fn set_positions<I>(node: &mut dyn WidgetTrait<I>, outer_anchor: Anchor) -> Rect {
    let margined_size = node.size() + node.style().margin.vec2() * 2.0;
    let margined_pos = outer_anchor.get_top_left_pixel(margined_size);
    let pos = margined_pos + node.style().margin.vec2();
    node.set_pos(pos);

    // the first child has to Anchor::inside, and the next ones need to Anchor::next_to, so create an empty rect as first child
    let initial_anchor = Anchor::inside(node.rect(), node.style().layout, node.style().pad.vec2());
    let zero2d = SizeInPixels2d::default();
    let mut previous_rect = initial_anchor.get_rect(zero2d);
    let style = node.style().clone();
    for child in node.children_mut() {
        let anchor = Anchor::next_to(previous_rect, style.layout, 0.0);
        previous_rect = set_positions(child, anchor);
    }
    to_rect(margined_pos, margined_size)
}
