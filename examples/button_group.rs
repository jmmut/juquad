use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, Style};
use juquad::widgets::button_group::ButtonGroup;
use macroquad::prelude::{screen_height, screen_width};

const FONT_SIZE: f32 = 16.0;
const STYLE: Style = Style::new();

struct Buttons {
    button1: Button,
    button2: Button,
    button3: Button,
}

#[macroquad::main("juquad button group")]
async fn main() {
    let mut button_group = ButtonGroup::new(
        FONT_SIZE,
        Anchor::top_center(screen_width() * 0.5, screen_height() * 0.25),
    );
    let mut buttons = Buttons {
        button1: button_group.add("some button"),
        button2: button_group.add("some long long long button"),
        button3: button_group.add("another button"),
    };

    let mut show_button3 = false;
    if buttons.button1.interact().is_clicked() {
        show_button3 = !show_button3;
    }

    buttons.button1.render(&STYLE);
    buttons.button2.render(&STYLE);
    if show_button3 {
        buttons.button3.render(&STYLE);
    }
}
