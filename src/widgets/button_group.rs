use crate::widgets::anchor::Anchor;
use crate::widgets::button::Button;
use crate::widgets::Widget;

pub struct ButtonGroup {
    font_size: f32,
    anchor: Anchor,
}

impl ButtonGroup {
    pub fn new(font_size: f32, anchor: Anchor) -> Self {
        Self { font_size, anchor }
    }
    // pub fn new_(widgets: Vec<Widget>) -> Self {
    //     ButtonGroup
    // }
    pub fn add(&mut self, text: &str) -> Button {
        let button = Button::new(text, self.anchor, self.font_size);
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
}
