use crate::PixelPosition;
use macroquad::prelude::{KeyCode, MouseButton};

pub trait InputTrait {
    fn is_key_down(&self, key: KeyCode) -> bool;
    fn is_key_pressed(&self, key: KeyCode) -> bool;
    fn is_mouse_button_down(&self, button: MouseButton) -> bool;
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool;
    fn is_mouse_button_released(&self, button: MouseButton) -> bool;
    fn mouse_position(&self) -> PixelPosition;
    fn mouse_wheel(&self) -> PixelPosition;

    fn clone(&self) -> Box<dyn InputTrait>;
}
