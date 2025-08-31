use juquad::draw::draw_rect_lines;
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Vertical};
use juquad::widgets::button::Button;
use juquad::widgets::button_group::{Direction, LabelGroup};
use juquad::widgets::text::TextRect;
use juquad::widgets::Style;
use macroquad::color::{Color, BLACK};
use macroquad::math::Vec2;
use macroquad::prelude::{clear_background, next_frame, screen_height, screen_width, Rect, RED};
use macroquad::text::{draw_text, load_ttf_font_from_bytes, Font};

const STYLE: Style = Style::new();

struct Buttons {
    expand: Button,
    increase_font: Button,
    decrease_font: Button,
    change_font: Button,
    toggle_borders: Button,
    some_text: TextRect,
    toggle_alignment: Button,
    exit: Button,
}

#[macroquad::main("juquad button group")]
async fn main() {
    let mut font_size: f32 = 16.0;
    // let font_bytes = include_bytes!("../assets/Roboto-Regular.ttf");
    let font_bytes = include_bytes!("../assets/Saira-Regular.ttf");
    let font = load_ttf_font_from_bytes(font_bytes).unwrap();
    let mut custom_font = false;
    let mut alignment = Horizontal::Center;

    let mut buttons = create_button_group(None, font_size, alignment);

    let mut show_extra_buttons = false;
    let mut show_borders = false;
    loop {
        let mut update_buttons = false;
        clear_background(Color::new(0.85, 0.85, 0.85, 1.00));
        clear_background(Color::new(0.5, 0.7, 0.8, 1.0));
        draw_text(
            &format!("{}", font_size),
            screen_width() * 0.5,
            50.0,
            16.0,
            BLACK,
        );
        if buttons.expand.interact().is_clicked() {
            show_extra_buttons = !show_extra_buttons;
        }
        if buttons.increase_font.interact().is_clicked() {
            font_size += 1.0;
            update_buttons = true;
        }
        if show_extra_buttons && buttons.decrease_font.interact().is_clicked() {
            font_size -= 1.0;
            update_buttons = true;
        }
        if show_extra_buttons && buttons.change_font.interact().is_clicked() {
            custom_font = !custom_font;
            update_buttons = true;
        }
        if show_extra_buttons && buttons.toggle_borders.interact().is_clicked() {
            show_borders = !show_borders;
        }
        if show_extra_buttons && buttons.exit.interact().is_clicked() {
            break;
        }
        if show_extra_buttons && buttons.toggle_alignment.interact().is_clicked() {
            alignment = match alignment {
                Horizontal::Left => Horizontal::Center,
                Horizontal::Center => Horizontal::Right,
                Horizontal::Right => Horizontal::Left,
            };
            update_buttons = true;
        }

        buttons.expand.render_default(&STYLE);
        buttons.increase_font.render_default(&STYLE);
        if show_extra_buttons {
            buttons.change_font.render_default(&STYLE);
            buttons.decrease_font.render_default(&STYLE);
            buttons.toggle_borders.render_default(&STYLE);
            buttons.some_text.render_default(&STYLE.at_rest);
            buttons.toggle_alignment.render_default(&STYLE);
            buttons.exit.render_default(&STYLE);
        }
        if show_borders {
            draw_rect_lines(text_border(&buttons.expand.text_rect), 2.0, RED);
            draw_rect_lines(text_border(&buttons.increase_font.text_rect), 2.0, RED);
            draw_rect_lines(text_border(&buttons.change_font.text_rect), 2.0, RED);
            draw_rect_lines(text_border(&buttons.decrease_font.text_rect), 2.0, RED);
            draw_rect_lines(text_border(&buttons.toggle_borders.text_rect), 2.0, RED);
            draw_rect_lines(text_border(&buttons.some_text), 2.0, RED);
            draw_rect_lines(text_border(&buttons.exit.text_rect), 2.0, RED);
        }
        if update_buttons {
            let font_option = if custom_font { Some(font) } else { None };
            buttons = create_button_group(font_option, font_size, alignment);
        }

        next_frame().await
    }
}

fn create_button_group(font: Option<Font>, font_size: f32, alignment: Horizontal) -> Buttons {
    let _pad = if font.is_some() {
        Vec2::new(font_size * 2.0, font_size * 0.65)
    } else {
        Vec2::new(font_size, font_size * 0.25)
    };
    let anchor = Anchor::top_center(screen_width() * 0.5, screen_height() * 0.25);
    let direction = Direction::Bottom;
    let label_group = LabelGroup {
        font_size,
        font,
        anchor,
        alignment,
        direction,
    };

    let [expand, increase_font, decrease_font, change_font, toggle_borders, some_text, toggle_alignment, exit] =
        label_group.create([
            "some button to expand",
            "long button to increase font size",
            "decrease font size",
            "CHANGE FONT",
            "toggle borders",
            "some text",
            "toggle alignment",
            "exit",
        ]);
    // let [expand, increase_font, decrease_font, change_font, toggle_borders, some_text, exit] =
    //     texts;
    Buttons {
        expand: expand.into(),
        increase_font: increase_font.into(),
        decrease_font: decrease_font.into(),
        change_font: change_font.into(),
        toggle_borders: toggle_borders.into(),
        some_text,
        toggle_alignment: toggle_alignment.into(),
        exit: exit.into(),
    }
}

fn text_border(rect: &TextRect) -> Rect {
    Rect::new(
        (rect.rect.x + rect.pad.x).round(),
        (rect.rect.y + rect.pad.y + 1.0).round(),
        rect.text_width.round(),
        rect.text_height.round(),
    )
}
