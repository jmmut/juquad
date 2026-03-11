use juquad::draw::to_rect;
use juquad::elm::button::Button;
use juquad::elm::container::Container;
use juquad::elm::slider::Slider;
use juquad::elm::style::Style;
use juquad::elm::text::Text;
use juquad::elm::widget::{compute_layout, RenderableWidget};
use juquad::widgets::anchor::{Horizontal, Layout, Spot, Vertical};
use juquad::SizeInPixels2d;
use macroquad::miniquad::date::now;
use macroquad::prelude::{
    clear_background, is_key_pressed, is_mouse_button_pressed, load_ttf_font_from_bytes,
    mouse_position, next_frame, screen_height, screen_width, vec2, KeyCode, MouseButton,
};

#[derive(Copy, Clone)]
pub enum Message {
    None,
    Exit,
    PadX(f32),
    PadY(f32),
    MarginX(f32),
    MarginY(f32),
}

#[macroquad::main("juquad elm ui")]
async fn main() {
    let font_size: f32 = 16.0;
    // let font_bytes = include_bytes!("../assets/Saira-Regular.ttf");
    let font_bytes = include_bytes!("../assets/Roboto-Regular.ttf");
    let font = Some(load_ttf_font_from_bytes(font_bytes).unwrap());
    // let pad = Pad::new_symmetric(10.0);
    // let pad = Pad::new(0.0, 0.0);
    // let margin = Pad::new(0.0, 10.0);
    let mut style = Style {
        font_size,
        // pad,
        // margin,
        font,
        ..Default::default()
    };

    // unsafe {
    //     DEBUG_WIDGETS = true;
    // }
    let mut screen = vec2(screen_width(), screen_height());
    let mut recalculate_ui = false;
    let mut ui = rebuild_ui(screen, &style);
    'main_loop: loop {
        let _start = now();
        let new_screen = vec2(screen_width(), screen_height());
        if new_screen != screen {
            screen = new_screen;
            recalculate_ui = true;
        }
        if recalculate_ui {
            recalculate_ui = false;
            ui = rebuild_ui(screen, &style);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        for message in ui.interact() {
            match message {
                Message::None => {}
                Message::Exit => break 'main_loop,
                Message::PadX(new_value) => {
                    maybe_modify(&mut style.pad.x, new_value, &mut recalculate_ui);
                }
                Message::PadY(new_value) => {
                    maybe_modify(&mut style.pad.y, new_value, &mut recalculate_ui);
                }
                Message::MarginX(new_value) => {
                    maybe_modify(&mut style.margin.x, new_value, &mut recalculate_ui);
                }
                Message::MarginY(new_value) => {
                    maybe_modify(&mut style.margin.y, new_value, &mut recalculate_ui);
                }
            }
        }

        clear_background(style.coloring.at_rest.bg_color);
        ui.render();

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("{:?}", mouse_position())
        }
        // print_time_since(_start, "frame took");
        next_frame().await
    }
}

fn maybe_modify(current: &mut f32, new_value: f32, recalculate_ui: &mut bool) {
    if !float_eq(*current, new_value, 0.0001) {
        *current = new_value;
        *recalculate_ui = true;
    }
}

fn float_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

fn rotate_alignment(style: &mut Style) {
    style.layout = style.layout.map_alignment(Spot::next);
    // match &mut style.layout {
    //     Layout::Horizontal { alignment, .. } => *alignment = alignment.next(),
    //     Layout::Vertical { alignment, .. } => *alignment = alignment.next(),
    // }
}

#[allow(unused)]
fn rotate_direction(style: &mut Style) {
    // match &mut style.layout {
    //     Layout::Horizontal { direction, .. } => *direction = direction.next(),
    //     Layout::Vertical { direction, .. } => *direction = direction.next(),
    // }
    style.layout = style.layout.map_direction(Spot::next);
}

fn flip_direction(style: &mut Style) {
    // match &mut style.layout {
    //     Layout::Horizontal { direction, .. } => *direction = direction.opposite(),
    //     Layout::Vertical { direction, .. } => *direction = direction.opposite(),
    // }
    // style.layout = style
    //     .layout
    //     .with_direction(style.layout.get_direction().opposite())
    style.layout = style.layout.map_direction(Spot::opposite)
}

fn rotate_layout(style: &mut Style) {
    // style.layout = match &mut style.layout {
    //     Layout::Horizontal {
    //         direction,
    //         alignment,
    //     } => Layout::Vertical {
    //         direction: direction.rotate(),
    //         alignment: alignment.rotate(),
    //     },
    //     Layout::Vertical {
    //         direction,
    //         alignment,
    //     } => Layout::Horizontal {
    //         direction: direction.rotate(),
    //         alignment: alignment.rotate(),
    //     },
    // };
    style.layout = style.layout.transpose(Horizontal::rotate, Vertical::rotate)
}

fn rebuild_ui(screen: SizeInPixels2d, style: &Style) -> Box<dyn RenderableWidget<Message>> {
    let start = now();
    let style_horizontal = &Style {
        layout: Layout::horizontal(Horizontal::Right, Vertical::Center),
        ..style.clone()
    };

    let mut ui = Container::new(
        style,
        vec![
            Text::new(style, "Some text"),
            Container::new(
                style_horizontal,
                vec![
                    Text::new(style, format!("Pad x: {:0>6.2}", style.pad.x)),
                    Slider::new(style, 0.0, 100.0, style.pad.x, Message::PadX),
                    Text::new(style, format!("Pad y: {:0>6.2}", style.pad.y)),
                    Slider::new(style, 0.0, 100.0, style.pad.y, Message::PadY),
                ],
            ),
            Container::new(
                style_horizontal,
                vec![
                    Text::new(style, format!("Margin x: {:0>6.2}", style.margin.x)),
                    Slider::new(style, 0.0, 100.0, style.margin.x, Message::MarginX),
                    Text::new(style, format!("Margin y: {:0>6.2}", style.margin.y)),
                    Slider::new(style, 0.0, 100.0, style.margin.y, Message::MarginY),
                ],
            ),
            Button::new_text(style, Message::Exit, "Exit"),
        ],
    );

    let screen_rect = to_rect(vec2(0.0, 0.0), screen);
    compute_layout(&mut *ui, screen_rect, style.layout);

    print_time_since(start, "rebuilt ui in");
    ui
}

fn print_time_since(_start_seconds: f64, _name: &str) {
    let _end = now();
    println!("{} {:.3} ms", _name, (_end - _start_seconds) * 1000.0);
}
