use juquad::widgets::anchor::Anchor;
use juquad::widgets::anchorer::{Alignment, Anchorer, Direction};
use juquad::widgets::button::{Button, Style};
use macroquad::prelude::{next_frame, screen_height, screen_width};

const FONT_SIZE: f32 = 16.0;
const STYLE: Style = Style::new();

#[macroquad::main("juquad individual buttons")]
async fn main() {
    // let anchor = Anchor::default();
    let anchor = Anchor::center(screen_width() * 0.5, screen_height() * 0.125);
    let mut button_enable = Button::new("toggle many buttons", anchor, FONT_SIZE);
    let mut alignment = Alignment::Left;
    let mut buttons_enabled = false;
    loop {
        let mut anchorer = Anchorer::new(Direction::Down(alignment));
        anchorer.move_after(button_enable.rect());
        let mut button_1 = Button::new("toggle center", Anchor::default(), FONT_SIZE);
        anchorer.move_and_modify(button_1.rect_mut());
        if button_1.interact().is_clicked() {
            if let Alignment::Left = alignment {
                alignment = Alignment::Center;
            } else {
                alignment = Alignment::Left;
            }
        }
        let mut extra_buttons = Vec::new();
        if button_enable.interact().is_clicked() {
            buttons_enabled = !buttons_enabled;
        }
        if buttons_enabled {
            let texts = vec!["some long button text", "button 3"];
            for text in texts {
                let mut button = Button::new(text, Anchor::default(), FONT_SIZE);
                anchorer.move_and_modify(button.rect_mut());
                button.interact();
                extra_buttons.push(button);
            }
        }

        // render
        button_enable.render(&STYLE);
        button_1.render(&STYLE);
        if buttons_enabled {
            for extra_button in extra_buttons {
                extra_button.render(&STYLE);
            }
        }
        next_frame().await
    }
}
