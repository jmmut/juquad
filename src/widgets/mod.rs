use macroquad::math::Rect;

pub mod anchor;
pub mod anchorer;
pub mod button;
pub mod button_group;
pub mod text;
pub mod texture_button;

pub trait Widget {
    fn rect(&self) -> Rect;
    fn rect_mut(&mut self) -> &mut Rect;
    // TODO: define interface
}
