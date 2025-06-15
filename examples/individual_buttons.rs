use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::button::{Button, Style};
use macroquad::prelude::{next_frame, screen_height, screen_width};

const FONT_SIZE: f32 = 16.0;
const STYLE: Style = Style::new();

#[macroquad::main("juquad individual buttons")]
async fn main() {
    let anchor = Anchor::center(screen_width() * 0.5, screen_height() * 0.125);
    let mut button_enable = Button::new("toggle many buttons", anchor, FONT_SIZE);
    let mut alignment = Horizontal::Left;
    let mut buttons_enabled = false;
    loop {
        let layout = Layout::vertical(Vertical::Bottom, alignment);
        let anchor = Anchor::next_to(button_enable.rect(), layout, 0.0);
        let mut button_1 = Button::new("toggle center", anchor, FONT_SIZE);
        if button_1.interact().is_clicked() {
            if let Horizontal::Left = alignment {
                alignment = Horizontal::Center;
            } else {
                alignment = Horizontal::Left;
            }
        }
        let mut extra_buttons = Vec::new();
        if button_enable.interact().is_clicked() {
            buttons_enabled = !buttons_enabled;
        }
        if buttons_enabled {
            let texts = vec!["some long button text", "button 3"];
            let mut previous = button_1.rect();
            for text in texts {
                let anchor = Anchor::next_to(previous, layout, 0.0);
                let mut button = Button::new(text, anchor, FONT_SIZE);
                previous = button.rect();
                button.interact();
                extra_buttons.push(button);
            }
        }

        // render
        button_enable.render_default(&STYLE);
        button_1.render_default(&STYLE);
        if buttons_enabled {
            for extra_button in extra_buttons {
                extra_button.render_default(&STYLE);
            }
        }
        next_frame().await
    }
}
