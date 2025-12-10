use macroquad::prelude::{clear_background, draw_rectangle, draw_text, next_frame, screen_height, screen_width, vec2, Rect, BLACK, LIGHTGRAY};
use juquad::draw::draw_rect;
use juquad::lazy::button::Button;
use juquad::widgets::Style;
use juquad::widgets::text::TextRect;

const STYLE: Style = Style::new();

// struct Buttons {
//     expand: Button,
//     increase_font: Button,
//     decrease_font: Button,
//     change_font: Button,
//     toggle_borders: Button,
//     some_text: TextRect,
//     toggle_alignment: Button,
//     exit: Button,
// }


#[macroquad::main("juquad button group")]
async fn main() {
    let mut font_size: f32 = 16.0;
    let v = vec2(10.0, 20.0);
    let pad = 20.0;
    println!("{}", v[0]);
    loop {
        clear_background(BLACK);
        let screen = vec2(screen_width(), screen_height());
        draw_rectangle(pad, pad, screen.x - 2.0*pad, screen.y - 2.0*pad, LIGHTGRAY);
        draw_text("asdf", 100.0, 50.0, font_size, BLACK);
        next_frame().await
    }
}

