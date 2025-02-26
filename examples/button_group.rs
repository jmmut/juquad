use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, Style};
use juquad::widgets::button_group::ButtonGroup;
use macroquad::color::BLACK;
use macroquad::math::Vec2;
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, LIGHTGRAY};
use macroquad::text::{draw_text, load_ttf_font_from_bytes, Font};

const STYLE: Style = Style::new();

struct Buttons {
    expand: Button,
    button2: Button,
    change_font: Button,
    button4: Button,
    exit: Button,
}

#[macroquad::main("juquad button group")]
async fn main() {
    let mut font_size: f32 = 16.0;
    let font_bytes = include_bytes!("../assets/Roboto-Regular.ttf");
    // let font_bytes = include_bytes!("../assets/Roboto-Light.ttf");
    let font = load_ttf_font_from_bytes(font_bytes).unwrap();
    let mut custom_font = false;

    let mut buttons = create_button_group(None, font_size);

    let mut show_button3 = false;
    loop {
        let mut update_buttons = false;
        clear_background(LIGHTGRAY);
        draw_text(
            &format!("{}", font_size),
            screen_width() * 0.5,
            50.0,
            16.0,
            BLACK,
        );
        if buttons.expand.interact().is_clicked() {
            show_button3 = !show_button3;
        }
        if buttons.button2.interact().is_clicked() {
            font_size += 1.0;
            update_buttons = true;
        }
        if buttons.button4.interact().is_clicked() {
            font_size -= 1.0;
            update_buttons = true;
        }
        if buttons.change_font.interact().is_clicked() {
            custom_font = !custom_font;
            update_buttons = true;
        }
        if buttons.exit.interact().is_clicked() {
            break;
        }

        buttons.expand.render(&STYLE);
        buttons.button2.render(&STYLE);
        if show_button3 {
            buttons.change_font.render(&STYLE);
            buttons.button4.render(&STYLE);
            buttons.exit.render(&STYLE);
        }
        if update_buttons {
            let font_option = if custom_font { Some(font) } else { None };
            buttons = create_button_group(font_option, font_size);
        }

        next_frame().await
    }
}

fn create_button_group(font: Option<Font>, font_size: f32) -> Buttons {
    let pad = if font.is_some() {
        Vec2::new(font_size * 2.0, font_size * 0.65)
    } else {
        Vec2::new(font_size, font_size * 0.25)
    };
    let mut button_group = ButtonGroup::new_with_font(
        font_size,
        font,
        pad,
        Anchor::top_center(screen_width() * 0.5, screen_height() * 0.25),
    );

    let buttons: Buttons = button_group.create_generic([
        "some button to expand",
        "some long long long button",
        "change font",
        "UPPER CASE BUTTON",
        "exit",
    ]);
    buttons
}
