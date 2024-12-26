use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, Style};
use juquad::widgets::button_group::ButtonGroup;
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, BLUE};
use std::mem::ManuallyDrop;

const FONT_SIZE: f32 = 16.0;
const STYLE: Style = Style::new();

struct Buttons {
    button1: Button,
    button2: Button,
    button3: Button,
    button4: Button,
}

#[macroquad::main("juquad button group")]
async fn main() {
    let mut button_group = ButtonGroup::new(
        FONT_SIZE,
        Anchor::top_center(screen_width() * 0.5, screen_height() * 0.25),
    );

    let mut buttons: Buttons = button_group.create_T();

    let mut show_button3 = false;
    loop {
        clear_background(BLUE);
        if buttons.button1.interact().is_clicked() {
            show_button3 = !show_button3;
        }
        buttons.button2.interact();
        buttons.button3.interact();
        buttons.button4.interact();

        buttons.button1.render(&STYLE);
        buttons.button2.render(&STYLE);
        if show_button3 {
            buttons.button3.render(&STYLE);
            buttons.button4.render(&STYLE);
        }
        next_frame().await
    }
}
