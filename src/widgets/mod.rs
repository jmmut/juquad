use crate::widgets::button::Button;
use crate::widgets::button_group::ButtonGroup;
use crate::widgets::text::TextRect;
use macroquad::math::Rect;

pub mod anchor;
pub mod anchor2;
pub mod anchorer;
pub mod button;
pub mod button_group;
pub mod text;
pub mod texture_button;

pub enum Widget {
    Text(TextRect),
    Button(Button),
    ButtonGroup(ButtonGroup),
    Custom(Box<dyn CustomWidget>),
}

pub trait CustomWidget {
    fn rect(&self) -> Rect;
    fn rect_mut(&mut self) -> &mut Rect;
    // TODO: define interface
}
