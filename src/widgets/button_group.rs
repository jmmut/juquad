use crate::input::input_macroquad::InputMacroquad;
use crate::input::input_trait::InputTrait;
use crate::widgets::anchor::Anchor;
use crate::widgets::button::{render_button, Button, RenderButton};
use crate::widgets::text::{draw_text, TextRect};
use macroquad::math::Vec2;
use macroquad::prelude::{measure_text, Rect};
use macroquad::text::Font;
use std::mem::ManuallyDrop;

pub struct ButtonGroup {
    label_group: LabelGroup,
    pub render: RenderButton,
    pub input: Box<dyn InputTrait>,
}

pub struct LabelGroup {
    font_size: f32,
    font: Option<Font>,
    anchor: Anchor,
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
            render: render_button,
            input: Box::new(InputMacroquad),
        }
    }
    // pub fn new_(widgets: Vec<Widget>) -> Self {
    //     ButtonGroup
    // }

    pub fn create_generic<T, S: AsRef<str>, const N: usize>(&mut self, texts: [S; N]) -> T {
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
                self.render,
                self.input.clone(),
            ));
        }
        buttons.try_into().unwrap_or_else(|v: Vec<_>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }
}

impl LabelGroup {
    pub fn new(font_size: f32, anchor: Anchor) -> Self {
        Self::new_with_font(font_size, None, anchor)
    }
    pub fn new_with_font(font_size: f32, font: Option<Font>, anchor: Anchor) -> Self {
        Self {
            font_size,
            font,
            anchor,
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
        let pad = Vec2::new(reference_height, reference_height * 0.75);

        for text in texts {
            let text = text.as_ref().to_string();
            let text_dimensions = measure_text(&text, self.font, self.font_size as u16, 1.0);
            if text_dimensions.width > max_width {
                max_width = text_dimensions.width;
            }
            dimensions.push((text, text_dimensions));
        }
        let elem_size = Vec2::new(
            (max_width + pad.x * 2.0).round(),
            (reference_height + pad.y * 2.0).round(),
        );
        let mut top_left = self
            .anchor
            .get_top_left_pixel(elem_size * Vec2::new(1.0, N as f32));

        for (text, dimension) in dimensions {
            let rect = Rect::new(
                top_left.x.round(),
                top_left.y.round(),
                elem_size.x,
                elem_size.y,
            );

            let text_rect = TextRect {
                text,
                rect,
                font_size: self.font_size,
                font: self.font,
                pad: Vec2::new((elem_size.x - dimension.width) * 0.5, pad.y),
                offset_y: dimension.offset_y,
                text_width: dimension.width,
                text_height: dimension.height,
                reference_height,
                draw_text,
            };
            text_rects.push(text_rect);
            top_left.y += elem_size.y
                // + 1.0
            ;
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
