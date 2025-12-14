use crate::draw::{draw_rect_lines, to_rect};
use crate::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use crate::widgets::text::Pixels;
use crate::widgets::{Interaction, Style as Coloring};
use crate::{PositionInPixels2d, SizeInPixels2d};
use macroquad::color::{Color, BLACK, BLUE, ORANGE};
use macroquad::prelude::{vec2, Font, Rect, Vec2};

pub mod button;
pub mod panel;
pub mod text;

pub const DEFAULT_FONT_SIZE: f32 = 16.0;
pub static mut DEBUG_WIDGETS: bool = false;

pub type Widgets = Vec<Box<dyn RenderableWidget>>;
// pub type WidgetsViewMut<'a, 'b> = Vec<&'a mut (dyn RenderableWidget + 'b)>;
pub type WidgetsViewMut<'a> = Vec<&'a mut dyn RenderableWidget>;
pub type WidgetsView<'a> = Vec<&'a dyn RenderableWidget>;

pub trait WidgetTrait {
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
    fn reanchor(&mut self, anchor: Anchor) {
        let new_rect = anchor.get_top_left_pixel(self.size());
        self.set_pos(new_rect);
    }
    fn set_pos(&mut self, position: PositionInPixels2d);
    fn set_size(&mut self, size: SizeInPixels2d);
    fn style(&self) -> &Style;
    // fn children_mut(&mut self) -> &mut Widgets;
    // fn children(&self) -> &Widgets;
    fn children_mut(&mut self) -> WidgetsViewMut<'_>;
    fn children(&self) -> WidgetsView<'_>;
}
pub trait Renderable {
    fn render_interactive(&self, interaction: Interaction);
    fn render(&self) {
        self.render_interactive(Interaction::None)
    }
    // fn render_generic?
}
pub trait RenderableWidget: Renderable + WidgetTrait {}

impl<T: Renderable + WidgetTrait> RenderableWidget for T {}

pub struct WidgetData<Custom> {
    pos: PositionInPixels2d,
    size: Option<SizeInPixels2d>,
    style: Style,
    pub custom: Custom,
    pub children: Widgets,
    // TODO: children?
}
impl<Custom> WidgetData<Custom> {
    pub fn container_custom(style: Style, custom: Custom, children: Widgets) -> Self {
        Self {
            pos: Default::default(),
            size: Default::default(),
            style,
            custom,
            children,
        }
    }
    pub fn leaf_custom(style: Style, custom: Custom) -> Self {
        Self {
            pos: Default::default(),
            size: Default::default(),
            style,
            custom,
            children: Vec::new(),
        }
    }
}
impl<Custom: Default> WidgetData<Custom> {
    pub fn container(style: Style, children: Widgets) -> Self {
        Self {
            pos: Default::default(),
            size: Default::default(),
            style,
            custom: Default::default(),
            children,
        }
    }
    pub fn leaf(style: Style) -> Self {
        Self {
            pos: Default::default(),
            size: Default::default(),
            style,
            custom: Default::default(),
            children: Vec::new(),
        }
    }
}
impl<Custom: Default> Default for WidgetData<Custom> {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            size: Default::default(),
            style: Default::default(),
            custom: Default::default(),
            children: Default::default(),
        }
    }
}
impl<Custom> WidgetTrait for WidgetData<Custom> {
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

    // fn children_mut(&mut self) -> &mut Widgets {
    //     &mut self.children
    // }
    //
    // fn children(&self) -> &Widgets {
    //     &self.children
    // }
    fn children_mut(&mut self) -> WidgetsViewMut<'_> {
        self.children
            .iter_mut()
            .map(|child| child.as_mut() as &mut dyn RenderableWidget)
            .collect()
    }
    fn children(&self) -> WidgetsView<'_> {
        self.children
            .iter()
            .map(|child| child.as_ref() as &dyn RenderableWidget)
            .collect()
    }
}

impl<Custom: Default> From<Style> for WidgetData<Custom> {
    fn from(style: Style) -> Self {
        Self {
            style,
            ..Default::default()
        }
    }
}

#[derive(Copy, Clone)]
pub enum Size {
    Fit,
    Grow,
    Size { w: Pixels, h: Pixels },
    Ratio { w: f32, h: f32 },
}

#[derive(Copy, Clone)]
pub enum Pad {
    Symmetric(f32),
    Asymmetric { x: f32, y: f32 },
}
impl Pad {
    pub fn position(&self, position: PositionInPixels2d) -> PositionInPixels2d {
        match self {
            Pad::Symmetric(pad) => position + vec2(*pad, *pad),
            Pad::Asymmetric { x, y } => position + vec2(*x, *y),
        }
    }
    pub fn vec2(&self) -> SizeInPixels2d {
        match self {
            Pad::Symmetric(pad) => vec2(*pad, *pad),
            Pad::Asymmetric { x, y } => vec2(*x, *y),
        }
    }
}
#[derive(Copy, Clone)]
pub struct Style {
    pub pad: Pad,
    pub margin: Pad,
    pub layout: Layout,
    pub font_size: f32,
    pub font: Option<Font>,
    pub size: Size,
    pub coloring: Coloring,
}
impl Default for Style {
    fn default() -> Self {
        Self {
            pad: Pad::Symmetric(DEFAULT_FONT_SIZE),
            margin: Pad::Symmetric(DEFAULT_FONT_SIZE),
            layout: Layout::Vertical {
                direction: Vertical::Bottom,
                alignment: Horizontal::Center,
            },
            font_size: DEFAULT_FONT_SIZE,
            font: None,
            size: Size::Fit,
            coloring: Coloring::default(),
        }
    }
}

pub struct Ui {
    pub style: Style,
    pub screen_size: SizeInPixels2d,
    // children: Vec<Box<dyn Widget>>,
}

impl Default for Ui {
    fn default() -> Self {
        Self {
            style: Default::default(),
            screen_size: Default::default(),
            // children: vec![],
        }
    }
}

impl Ui {
    pub fn set_screen_size(&mut self, size: SizeInPixels2d) {
        self.screen_size = size;
    }
    pub fn start_container<W: WidgetTrait>(&self, widget: W) -> Container<W> {
        Container {
            widget,
            max_size: self.screen_size,
            children: Vec::new(),
        }
    }
}
pub struct Container<W: WidgetTrait> {
    pub max_size: SizeInPixels2d,
    pub widget: W,
    pub children: Vec<Box<dyn WidgetTrait>>,
}
impl<W: WidgetTrait> Container<W> {
    pub fn close(mut self) -> W {
        let style = self.widget.style();
        match style.size {
            Size::Grow => {
                self.widget.set_size(self.max_size);
            }
            Size::Fit | Size::Size { .. } | Size::Ratio { .. } => {
                unimplemented!()
            }
        }
        self.widget
    }
}
pub trait UiNodeTrait {}
pub struct UiNode<'a, 'b> {
    node_: &'a mut dyn WidgetTrait,
    children_: Vec<UiNode<'b, 'b>>,
}
impl<'a, 'b> UiNode<'a, 'b> {
    pub fn node(&'_ mut self) -> &'_ mut dyn WidgetTrait {
        self.node_
    }
    pub fn children(&'_ mut self) -> &'_ mut Vec<UiNode<'b, 'b>> {
        &mut self.children_
    }
}
pub fn leaf(node: &mut dyn WidgetTrait) -> UiNode<'_, '_> {
    UiNode {
        node_: node,
        children_: Vec::new(),
    }
}
pub fn container<'a, 'b>(
    node: &'a mut dyn WidgetTrait,
    children: Vec<UiNode<'b, 'b>>,
) -> UiNode<'a, 'b> {
    UiNode {
        node_: node,
        children_: children,
    }
}

pub fn set_sizes(node: &mut dyn RenderableWidget) {
    let mut accumulated_size = SizeInPixels2d::new(0.0, 0.0);
    let style = { *node.style() };
    let parallel = style.layout.parallel_index();
    let perpendicular = style.layout.perpendicular_index();
    for child in node.children_mut() {
        set_sizes(child);
        let size = child.size();
        let margin = style.margin.vec2();
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
pub fn set_positions(node: &mut dyn RenderableWidget, outer_anchor: Anchor) -> Rect {
    let margined_size = node.size() + node.style().margin.vec2() * 2.0;
    let margined_pos = outer_anchor.get_top_left_pixel(margined_size);
    let pos = margined_pos + node.style().margin.vec2();
    node.set_pos(pos);

    // the first child has to Anchor::inside, and the next ones need to Anchor::next_to, so create an empty rect as first child
    let initial_anchor = Anchor::inside(node.rect(), node.style().layout, node.style().pad.vec2());
    let zero2d = SizeInPixels2d::default();
    let mut previous_rect = to_rect(initial_anchor.get_top_left_pixel(zero2d), zero2d);
    let style = *node.style();
    for child in node.children_mut() {
        let anchor = Anchor::next_to(previous_rect, style.layout, 0.0);
        previous_rect = set_positions(child, anchor);
    }
    to_rect(margined_pos, margined_size)
}

pub fn add_contour(rect: Rect, size: SizeInPixels2d) -> Rect {
    let mut new_position = rect.point() - size;
    let mut new_size = rect.size() + size * 2.0;
    let center = rect.center();
    for i in 0..1 {
        if new_size[i] < 0.0 {
            // size reduced so much that the rect flips. collapse rather than invert
            new_position[i] = center[i];
            new_size[i] = 0.0;
        }
    }
    to_rect(new_position, new_size)
}

pub const DEBUGGING_ALPHA: f32 = 0.5;
pub const DEBUGGING_THICKNESS: f32 = 8.0;

pub fn with_alpha(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha)
}

fn draw_debug_widget<C>(widget_data: &WidgetData<C>) {
    let half_thickness = DEBUGGING_THICKNESS * 0.5;
    let contours = [
        (Vec2::new(0.0, 0.0), BLACK),
        (widget_data.style.margin.vec2(), BLUE),
        (-widget_data.style.pad.vec2() + half_thickness, ORANGE),
    ];
    let rect = widget_data.rect();
    for (contour, color) in contours {
        let drawn_rect = add_contour(rect, contour);
        let rect_color = with_alpha(color, DEBUGGING_ALPHA);
        draw_rect_lines(drawn_rect, DEBUGGING_THICKNESS, rect_color);
    }
}
