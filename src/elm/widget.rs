use crate::elm::style::Style;
use crate::widgets::anchor::Anchor;
use crate::{PositionInPixels2d, SizeInPixels2d};
use macroquad::math::Rect;

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
}

pub struct Widget<Custom> {
    pub custom: Custom,
    pub pos: PositionInPixels2d,
    pub size: Option<SizeInPixels2d>,
    pub style: Style,
}
