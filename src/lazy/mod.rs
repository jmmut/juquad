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

pub trait Widget {
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
    // fn children_mut(&mut self) -> Vec<&mut dyn Widget>;
    // fn children(&mut self) -> Vec<&dyn Widget>;
}
pub trait Renderable {
    fn render_interactive(&self, interaction: Interaction);
    fn render(&self) {
        self.render_interactive(Interaction::None)
    }
    // fn render_generic?
}
pub trait RenderableWidget: Renderable + Widget {}
pub trait AsWidget {
    fn widget(&self) -> &dyn Widget;
    fn widget_mut(&mut self) -> &mut dyn Widget;
}

impl<W: AsWidget> Widget for W {
    fn size(&self) -> SizeInPixels2d {
        self.widget().size()
    }
    fn pos(&self) -> PositionInPixels2d {
        self.widget().pos()
    }
    fn set_pos(&mut self, position: PositionInPixels2d) {
        self.widget_mut().set_pos(position)
    }
    fn set_size(&mut self, size: SizeInPixels2d) {
        self.widget_mut().set_size(size)
    }

    fn style(&self) -> &Style {
        self.widget().style()
    }
    // fn render_interactive(&self, interaction: Interaction) {
    //     unimplemented!("Widgets need to implement Widget::render(&self)")
    // }
}

#[derive(Copy, Clone)]
pub struct WidgetData {
    pos: PositionInPixels2d,
    size: Option<SizeInPixels2d>,
    style: Style,
    // TODO: custom: C,  // from WidgetData<C>
}
impl Default for WidgetData {
    fn default() -> Self {
        Self {
            pos: Default::default(),
            size: Default::default(),
            style: Default::default(),
        }
    }
}
impl Widget for WidgetData {
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
    // fn render_interactive(&self, interaction: Interaction) {
    //     unimplemented!("Widgets need to implement Widget::render(&self)")
    // }
}
impl From<Style> for WidgetData {
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
    pub fn start_container<W: Widget>(&self, widget: W) -> Container<W> {
        Container {
            widget,
            max_size: self.screen_size,
            children: Vec::new(),
        }
    }
}
pub struct Container<W: Widget> {
    pub max_size: SizeInPixels2d,
    pub widget: W,
    pub children: Vec<Box<dyn Widget>>,
}
impl<W: Widget> Container<W> {
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

pub struct UiNode<'a> {
    node: &'a mut dyn Widget,
    children: Vec<UiNode<'a>>,
}
pub fn leaf(node: &mut dyn Widget) -> UiNode<'_> {
    UiNode {
        node,
        children: Vec::new(),
    }
}
pub fn container<'a>(node: &'a mut dyn Widget, children: Vec<UiNode<'a>>) -> UiNode<'a> {
    UiNode { node, children }
}

pub fn set_sizes(node: &mut UiNode) {
    let mut accumulated_size = SizeInPixels2d::new(0.0, 0.0);
    for child in &mut node.children {
        set_sizes(child);
        let style = child.node.style();
        let parallel = style.layout.parallel_index();
        let perpendicular = style.layout.perpendicular_index();
        let size = child.node.size();
        let margin = style.margin.vec2();
        accumulated_size[parallel] += size[parallel] + 2.0 * margin[parallel];
        accumulated_size[perpendicular] =
            accumulated_size[perpendicular].max(size[perpendicular] + 2.0 * margin[perpendicular]);
    }
    accumulated_size += 2.0 * node.node.style().pad.vec2();
    node.node.set_size(accumulated_size);
    // println!(
    //     "size: {}, margin: {}, pad: {}",
    //     node.node.size(),
    //     node.node.style().margin.vec2(),
    //     node.node.style().pad.vec2()
    // );
}
pub fn set_positions(node: &mut UiNode, pos: PositionInPixels2d) {
    let mut accumulated_pos = pos;
    accumulated_pos += node.node.style().margin.vec2();
    node.node.set_pos(accumulated_pos);
    // println!("pos: {}", node.node.pos());
    let pad = node.node.style().pad.vec2();
    accumulated_pos += pad;
    for child in &mut node.children {
        let style = child.node.style();
        let parallel = style.layout.parallel_index();
        set_positions(child, accumulated_pos);
        accumulated_pos[parallel] +=
            child.node.size()[parallel] + node.node.style().margin.vec2()[parallel];
        let style = child.node.style();
        accumulated_pos[parallel] += style.margin.vec2()[parallel];
    }
}

pub fn add_contour(rect: Rect, size: SizeInPixels2d) -> Rect {
    let mut new_position = rect.point() - size;
    let mut new_size = rect.size() + size * 2.0;
    let center = rect.center();
    for i in 0..1 {
        if new_size[i] < 0.0 {
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

fn draw_debug_widget(widget_data: &WidgetData) {
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
