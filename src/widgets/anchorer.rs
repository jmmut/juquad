use crate::widgets::anchor::{Anchor, Layout};
use crate::widgets::button::Button;
use crate::widgets::text::TextRect;
use crate::widgets::Widget;
use crate::PixelPosition;
use macroquad::math::Rect;

pub struct Anchorer {
    current: Rect,
    layout: Layout,
    pad: f32,
}
impl Anchorer {
    pub fn new_pos(layout: Layout, start: PixelPosition, pad: f32) -> Self {
        Self::new(layout, Rect::new(start.x, start.y, 0.0, 0.0), pad)
    }
    pub fn new(layout: Layout, start: Rect, pad: f32) -> Self {
        Self {
            layout,
            current: start,
            pad,
        }
    }
    pub fn move_and_modify(&mut self, rect: &mut Rect) {
        let anchor = Anchor::next_to(self.current, self.layout, self.pad);
        *rect = anchor.get_rect(rect.size());
        self.current = *rect;
    }

    pub fn new_button(&mut self, text: &str, font_size: f32) -> Button {
        self.new_widget(|anchor| Button::new(text, anchor, font_size))
    }
    pub fn new_text(&mut self, text: &str, font_size: f32) -> TextRect {
        self.new_widget(|anchor| TextRect::new(text, anchor, font_size))
    }
    pub fn new_widget<W: Widget>(&mut self, f: impl Fn(Anchor) -> W) -> W {
        let anchor = Anchor::next_to(self.current, self.layout, 0.0);
        let widget = f(anchor);
        self.current = widget.rect();
        widget
    }
}
