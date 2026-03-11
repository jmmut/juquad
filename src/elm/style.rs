use crate::lazy::{Margin, Pad, Size, DEFAULT_FONT_SIZE};
use crate::widgets::anchor::{Horizontal, Layout, Vertical};
use crate::widgets::Coloring;
use macroquad::prelude::Font;

#[derive(Clone)]
pub struct Style {
    pub pad: Pad,
    pub margin: Margin,
    pub border: f32,
    pub layout: Layout,
    pub font_size: f32,
    pub font: Option<Font>,
    pub size: Size,
    pub coloring: Coloring,
}

impl Into<Style> for &Style {
    fn into(self) -> Style {
        self.clone()
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            pad: Pad::new(DEFAULT_FONT_SIZE * 1.5, DEFAULT_FONT_SIZE),
            margin: Margin::new_symmetric(0.0),
            border: 1.0,
            layout: Layout::vertical(Vertical::Bottom, Horizontal::Center),
            font_size: DEFAULT_FONT_SIZE,
            font: None,
            size: Size::Fit,
            coloring: Coloring::default(),
        }
    }
}
