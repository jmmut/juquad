use juquad::lazy::button::{Button, ButtonBase};
use juquad::lazy::panel::Panel;
use juquad::lazy::text::Text;
use juquad::lazy::{
    container, leaf, set_positions, set_sizes, Pad, Renderable, Size, Style, WidgetData,
};
use juquad::widgets::anchor::{Horizontal, Layout, Vertical};
use juquad::SizeInPixels2d;
use macroquad::miniquad::date::now;
use macroquad::prelude::{
    clear_background, is_key_pressed, is_mouse_button_pressed, mouse_position, next_frame,
    screen_height, screen_width, vec2, KeyCode, MouseButton, BLACK,
};
// const COLORING: Coloring = Coloring::new();

struct Buttons {
    panel: Panel,
    //     expand: Button,
    //     increase_font: Button,
    //     decrease_font: Button,
    //     change_font: Button,
    //     toggle_borders: Button,
    some_text: Text,
    some_text_2: Text,
    some_text_3: Text,
    toggle_alignment: Button,
    exit: Button,
}
impl Buttons {
    pub fn widgets(&self) -> Vec<&dyn Renderable> {
        vec![
            &self.panel,
            &self.some_text,
            &self.some_text_2,
            &self.some_text_3,
            &self.toggle_alignment,
            &self.exit,
        ]
    }
    pub fn render(&self) {
        for widget in self.widgets() {
            widget.render();
        }
    }
}

#[macroquad::main("juquad button group")]
async fn main() {
    let font_size: f32 = 22.0;
    let pad = Pad::Symmetric(10.0);
    let mut style = Style {
        font_size,
        pad,
        margin: pad,
        ..Default::default()
    };

    let mut screen = vec2(screen_width(), screen_height());
    let mut recalculate_ui = false;
    let mut buttons = rebuild_ui(screen, style);
    loop {
        let start = now();
        let new_screen = vec2(screen_width(), screen_height());
        if new_screen != screen {
            screen = new_screen;
            recalculate_ui = true;
        }
        if recalculate_ui {
            recalculate_ui = false;
            buttons = rebuild_ui(screen, style);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if buttons.toggle_alignment.interact().is_clicked() {
            match &mut style.layout {
                Layout::Horizontal { alignment, .. } => {
                    *alignment = match *alignment {
                        Vertical::Top => Vertical::Center,
                        Vertical::Center => Vertical::Bottom,
                        Vertical::Bottom => Vertical::Top,
                    };
                }
                Layout::Vertical { alignment, .. } => {
                    *alignment = match *alignment {
                        Horizontal::Left => Horizontal::Center,
                        Horizontal::Center => Horizontal::Right,
                        Horizontal::Right => Horizontal::Left,
                    };
                }
            }
            recalculate_ui = true;
        }
        if buttons.exit.interact().is_clicked() {
            break;
        }

        clear_background(BLACK);
        buttons.render();

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("{:?}", mouse_position())
        }
        print_time_since(start, "frame took");
        next_frame().await
    }
}

fn rebuild_ui(screen: SizeInPixels2d, style: Style) -> Buttons {
    let start = now();

    let text_style = Style {
        font: None,
        ..style
    };
    let start_text = now();
    let mut text = Text::new_text(text_style, "asdf");
    let mut text_2 = Text::new_text(text_style, "qwer");
    let mut text_3 = Text::new_text(text_style, "QWER");
    let mut toggle_text = Text::new_text(text_style, "Toggle alignment");
    let mut exit_text = Text::new_text(text_style, "Exit");
    print_time_since(start_text, "time measuring text");

    let mut toggle: Button = style.into();
    let mut exit: Button = style.into();
    // let mut exit: Button = Button::new(style, ButtonBase::new(vec![Box::new(exit_text)]));

    let mut panel: Panel = Style {
        size: Size::Grow,
        ..style
    }
    .into();

    let mut panel_node = container(
        &mut panel,
        vec![
            leaf(&mut text),
            leaf(&mut text_2),
            leaf(&mut text_3),
            container(&mut toggle, vec![leaf(&mut toggle_text)]),
            container(&mut exit, vec![leaf(&mut exit_text)]),
        ],
    );
    set_sizes(&mut panel_node);
    set_positions(&mut panel_node, vec2(0.0, 0.0), screen, &style);

    toggle.custom.children = vec![Box::new(toggle_text)];
    exit.custom.children = vec![Box::new(exit_text)];
    let buttons = Buttons {
        panel,
        some_text: text,
        some_text_2: text_2,
        some_text_3: text_3,
        toggle_alignment: toggle,
        exit,
    };
    print_time_since(start, "rebuilt ui in");
    buttons
}

fn print_time_since(_start_seconds: f64, _name: &str) {
    let _end = now();
    // println!("{} {:.3} ms", name, (end - start_seconds) * 1000.0);
}
