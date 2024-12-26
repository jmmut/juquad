use crate::widgets::anchor::Anchor;
use crate::widgets::button::Button;
use crate::widgets::text::TextRect;
use crate::widgets::Widget;
use crate::SizeInPixels;
use macroquad::math::Vec2;
use macroquad::prelude::{draw_text, measure_text, Rect, TextDimensions};
use std::mem::ManuallyDrop;

pub struct ButtonGroup {
    font_size: f32,
    anchor: Anchor,
    widest: f32,
}

union ButtonUnion<T, const N: usize> {
    b: ManuallyDrop<T>,
    v: ManuallyDrop<[Button; N]>,
}

impl ButtonGroup {
    pub fn new(font_size: f32, anchor: Anchor) -> Self {
        Self {
            font_size,
            anchor,
            widest: 0.0,
        }
    }
    // pub fn new_(widgets: Vec<Widget>) -> Self {
    //     ButtonGroup
    // }

    pub fn create_T<T>(&mut self) -> T {
        let array = self.create([
            "some button",
            "some long long long button",
            "another button",
            "UPPER CASE BUTTON",
        ]);
        let mut buttons_u = ButtonUnion {
            v: ManuallyDrop::new(array),
        };
        unsafe { ManuallyDrop::take(&mut buttons_u.b) }
    }

    pub fn create<S: AsRef<str>, const N: usize>(&self, texts: [S; N]) -> [Button; N] {
        let mut buttons = Vec::new();
        let mut max_width = 0.0;
        let mut dimensions = Vec::new();
        for text in texts {
            let text = text.as_ref().to_string();
            let text_dimensions = measure_text(&text, None, self.font_size as u16, 1.0);
            if text_dimensions.width > max_width {
                max_width = text_dimensions.width;
            }
            dimensions.push((text, text_dimensions));
        }
        let pad = Vec2::new(self.font_size, self.font_size * 0.25);
        let size = Vec2::new(
            (max_width + pad.x * 2.0).round(),
            (self.font_size + pad.y * 2.0).round(),
        );
        let mut top_left = self.anchor.get_top_left_pixel(size);

        for (text, dimension) in dimensions {
            let rect = Rect::new(top_left.x.round(), top_left.y.round(), size.x, size.y);

            let text_rect = TextRect {
                text,
                rect,
                font_size: self.font_size,
                pad: Vec2::new((size.x - dimension.width) * 0.5, pad.y),
                draw_text: draw_text,
            };
            buttons.push(Button::new_from_text_rect(text_rect));
            top_left.y += size.y + 1.0;
        }
        buttons.try_into().unwrap_or_else(|v: Vec<_>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
    /*
    pub fn add(&mut self, text: &str) -> Button {
        let button = Button::new(text, self.anchor, self.font_size);
        if button.rect().w > self.widest {
            self.widest = button.rect().w;
        }
        // TODO: support different directions
        match self.anchor {
            Anchor::Center { .. } => {
                todo!()
            }
            Anchor::CenterLeft { .. } => {
                todo!()
            }
            Anchor::CenterRight { .. } => {
                todo!()
            }
            Anchor::TopLeft { .. } => {
                self.anchor = Anchor::from_below(button.rect(), 0.0, 0.0);
            }
            Anchor::TopRight { .. } => {
                todo!()
            }
            Anchor::TopCenter { .. } => {
                todo!()
            }
            Anchor::BottomLeft { .. } => {
                todo!()
            }
            Anchor::BottomRight { .. } => {
                todo!()
            }
            Anchor::BottomCenter { .. } => {
                todo!()
            }
        }
        button
    }
    */

    // pub fn expand(&self, buttons: &[&mut Button]) {
    //     for button in buttons {
    //         button.text_rect.
    //     }
    // }
}
