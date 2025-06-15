use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::anchorer::Anchorer;
use juquad::widgets::button::Button;
use juquad::widgets::Style;
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
        let mut anchorer = Anchorer::new(layout, button_enable.rect(), 0.0);
        let mut button_1 = anchorer.new_button("toggle center", FONT_SIZE);
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
            for text in texts {
                let mut button = anchorer.new_button(text, FONT_SIZE);
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
