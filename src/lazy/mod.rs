use macroquad::prelude::{measure_text, vec2, Font, Rect, Vec2};
use crate::draw::draw_rect;
use crate::{PositionInPixels2d, SizeInPixels2d};
use crate::widgets::{Style as Coloring};
use crate::widgets::anchor::{Layout, Vertical, Horizontal, Anchor};
use crate::widgets::text::{draw_text, Pixels};

pub mod button;
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
}

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
}
impl From<Style> for WidgetData {
    fn from(style: Style) -> Self {
        Self {
            style,
            ..Default::default()
        }
    }
}

pub enum Size {
    Fit,
    Grow,
    Size{w: Pixels, h: Pixels},
    Ratio{w: f32, h: f32},
}

pub enum Pad {
    Symmetric(f32),
    Asymmetric{x: f32, y: f32},
}
impl Pad {
    pub fn position(&self, position: PositionInPixels2d) -> PositionInPixels2d {
        match self {
            Pad::Symmetric(pad) => { position + vec2(*pad, *pad)}
            Pad::Asymmetric { x, y } => {position + vec2(*x, *y)}
        }
    }
    pub fn vec2(&self) -> SizeInPixels2d{
        match self {
            Pad::Symmetric(pad) => {vec2(*pad, *pad)}
            Pad::Asymmetric { x, y } => {vec2(*x, *y)}
        }
    }
}
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
        Container {widget, max_size: self.screen_size, children: Vec::new()}
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
            Size::Fit |
            Size::Size { .. } |
            Size::Ratio { .. } => {
                unimplemented!()
            }
        }
        self.widget
    }
}

pub struct Panel {
    pub widget_data: WidgetData,
}
impl Default for Panel {
    fn default() -> Self {
        Self {
            widget_data: Default::default(),
        }
    }
}
impl Panel {
    pub fn new(widget_data: WidgetData) -> Self {
        Self {
            widget_data
        }
    }
    pub fn render(&self) {
        draw_rect(self.widget_data.rect(), self.style().coloring.at_rest.bg_color)
    }
}
impl AsWidget for Panel {
    fn widget(&self) -> &dyn Widget {
        &self.widget_data
    }
    fn widget_mut(&mut self) -> &mut dyn Widget {
        &mut self.widget_data
    }
}

pub struct Text {
    pub widget_data: WidgetData,
    pub text: String,
}
impl AsWidget for Text {
    fn widget(&self) -> &dyn Widget {
        &self.widget_data
    }
    fn widget_mut(&mut self) -> &mut dyn Widget {
        &mut self.widget_data
    }
}
impl Text {
    pub fn new(text: &str, mut widget_data: WidgetData) -> Self {
        let size = size_text(text, widget_data.style());
        widget_data.set_size(size);
        Self { widget_data, text: text.to_string()}
    }
    pub fn render(&self) {
        let pos = self.widget_data.pos();
        draw_text(&self.text, pos.x, pos.y, self.widget_data.style.font_size, &self.widget_data.style.coloring.at_rest, self.widget_data.style.font);
    }
}
// 
// impl Widget for Text {
//     fn rect(&self) -> Rect {
//         self.rect
//     }
// 
//     fn set_rect(&mut self, rect: Rect) {
//         todo!()
//     }
// }

fn size_text(text: &str, style: &Style) -> SizeInPixels2d {
    // font_size doesn't seem to be in pixels across fonts
    let reference_size = measure_text("Odp", style.font, style.font_size as u16, 1.0);
    let reference_height = reference_size.height;
    let pad = Vec2::new(reference_height, reference_height * 0.75);
    let text_dimensions = measure_text(text, style.font, style.font_size as u16, 1.0);

    let size = Vec2::new(
        (text_dimensions.width + pad.x * 2.0).round(),
        (reference_height + pad.y * 2.0).round(),
    );
    size
}

