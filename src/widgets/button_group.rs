use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::widgets::anchor::{Anchor, Horizontal};
use crate::widgets::button::Button;
use crate::widgets::text::TextRect;
use macroquad::math::{vec2, Vec2};
use macroquad::prelude::{measure_text, Rect};
use macroquad::text::Font;
use std::mem::ManuallyDrop;

pub struct ButtonGroup {
    label_group: LabelGroup,
    pub input: Box<dyn InputTrait>,
}

pub struct LabelGroup {
    pub font_size: f32,
    pub font: Option<Font>,
    pub alignment: Horizontal,
    pub direction: Direction,
    pub anchor: Anchor,
    pub pad_x: Option<f32>,
    pub pad_y: Option<f32>,
    pub margin: f32,
}
pub enum Direction {
    Top,
    Bottom,
    Right,
    Left,
}
union ButtonUnion<T, const N: usize> {
    b: ManuallyDrop<T>,
    v: ManuallyDrop<[Button; N]>,
}

impl ButtonGroup {
    pub fn new(font_size: f32, anchor: Anchor) -> Self {
        Self::new_with_labels(LabelGroup::new(font_size, anchor))
    }
    pub fn new_with_font(font_size: f32, font: Option<Font>, anchor: Anchor) -> Self {
        Self::new_with_labels(LabelGroup::new_with_font(font_size, font, anchor))
    }
    pub fn new_with_labels(label_group: LabelGroup) -> Self {
        Self {
            label_group,
            input: Box::new(InputMacroquad),
        }
    }
    // pub fn new_(widgets: Vec<Widget>) -> Self {
    //     ButtonGroup
    // }

    /// This is unsafe because Rust is allowed to reorganize the fields inside T.
    /// Marking T as repr(C) is safer but maybe still not worth it.
    pub unsafe fn create_generic<T, S: AsRef<str>, const N: usize>(&mut self, texts: [S; N]) -> T {
        assert_eq!(
            size_of::<T>(),
            N * size_of::<Button>(),
            "{} strings were specified but {} contains {} buttons",
            N,
            std::any::type_name::<T>(),
            size_of::<T>() / size_of::<Button>()
        );

        let array = self.create(texts);
        let mut buttons_u = ButtonUnion {
            v: ManuallyDrop::new(array),
        };
        unsafe { ManuallyDrop::take(&mut buttons_u.b) }
    }
    pub fn create<S: AsRef<str>, const N: usize>(&self, texts: [S; N]) -> [Button; N] {
        let text_rects = self.label_group.create(texts);
        let mut buttons = Vec::new();
        for text_rect in text_rects {
            buttons.push(Button::new_from_text_rect_generic(
                text_rect,
                self.input.clone(),
            ));
        }
        buttons.try_into().unwrap_or_else(|v: Vec<_>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
}

impl Default for LabelGroup {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            font: None,
            anchor: Anchor::top_left(0.0, 0.0),
            alignment: Horizontal::Center,
            direction: Direction::Bottom,
            pad_x: None,
            pad_y: None,
            margin: 0.0,
        }
    }
}
impl LabelGroup {
    pub fn new(font_size: f32, anchor: Anchor) -> Self {
        Self {
            font_size,
            anchor,
            ..Default::default()
        }
    }
    pub fn new_with_font(font_size: f32, font: Option<Font>, anchor: Anchor) -> Self {
        Self {
            font_size,
            font,
            anchor,
            ..Default::default()
        }
    }
    pub fn new_generic(
        font_size: f32,
        font: Option<Font>,
        anchor: Anchor,
        alignment: Horizontal,
        direction: Direction,
    ) -> Self {
        Self {
            font_size,
            font,
            anchor,
            alignment,
            direction,
            ..Default::default()
        }
    }
    pub fn create<S: AsRef<str>, const N: usize>(&self, texts: [S; N]) -> [TextRect; N] {
        let mut text_rects = Vec::new();
        let mut max_width = 0.0;
        let mut dimensions = Vec::new();
        // font_size doesn't seem to be in pixels across fonts
        let reference_size = measure_text("Odp", self.font, self.font_size as u16, 1.0);
        // let reference_size1 = measure_text("O", self.font, self.font_size as u16, 1.0);
        // let reference_size2 = measure_text(
        //     "some button to expand",
        //     self.font,
        //     self.font_size as u16,
        //     1.0,
        // );
        // let reference_size3 = measure_text("pd", self.font, self.font_size as u16, 1.0);

        let reference_height = reference_size.height;
        let pad_x = if let Some(p) = self.pad_x {
            p
        } else {
            reference_height
        };
        let pad_y = if let Some(p) = self.pad_y {
            p
        } else {
            reference_height * 0.75
        };
        
        let min_pad = vec2(pad_x, pad_y);

        for text in texts {
            let text = text.as_ref().to_string();
            let text_dimensions = measure_text(&text, self.font, self.font_size as u16, 1.0);
            if text_dimensions.width > max_width {
                max_width = text_dimensions.width;
            }
            dimensions.push((text, text_dimensions));
        }
        let elem_size = Vec2::new(
            (max_width + min_pad.x * 2.0).round(),
            (reference_height + min_pad.y * 2.0).round(),
        );
        let panel_size = -self.margin + (elem_size + self.margin)
            * match self.direction {
                Direction::Top | Direction::Bottom => Vec2::new(1.0, N as f32),
                Direction::Right | Direction::Left => Vec2::new(N as f32, 1.0),
            };
        let mut top_left = self.anchor.get_top_left_pixel(panel_size);

        for (text, dimension) in dimensions {
            let rect = Rect::new(
                top_left.x.round(),
                top_left.y.round(),
                elem_size.x,
                elem_size.y,
            );

            let pad = match self.alignment {
                Horizontal::Left => min_pad,
                Horizontal::Center => Vec2::new((elem_size.x - dimension.width) * 0.5, min_pad.y),
                Horizontal::Right => {
                    Vec2::new((elem_size.x - dimension.width) - min_pad.x, min_pad.y)
                }
            };
            let text_rect = TextRect {
                text,
                rect,
                font_size: self.font_size,
                font: self.font,
                pad,
                offset_y: dimension.offset_y,
                text_width: dimension.width,
                text_height: dimension.height,
                reference_height,
            };
            text_rects.push(text_rect);
            match self.direction {
                Direction::Top => {
                    top_left.y -= elem_size.y
                    // - 1.0
                    ;
                }
                Direction::Bottom => {
                    top_left.y += elem_size.y
                    // + 1.0
                    ;
                }
                Direction::Right => {
                    top_left.x += elem_size.x
                    // + 1.0
                    ;
                }
                Direction::Left => {
                    top_left.x -= elem_size.x
                    // - 1.0
                    ;
                }
            }
        }
        text_rects.try_into().unwrap_or_else(|v: Vec<_>| {
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
