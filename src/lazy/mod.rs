use macroquad::prelude::Rect;
use crate::widgets::{Style, Widget};

pub mod button;
pub mod text;

pub enum Size {
    
}

pub struct Ui {
    style: Style,
    children: Vec<Box<dyn Widget>>,
}

pub struct Panel {
    rect: Rect,
}


impl Widget for Panel {
    fn rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}


