use juquad::lazy::button::Button;
use juquad::lazy::panel::Panel;
use juquad::lazy::text::Text;
use juquad::lazy::{
    container, leaf, set_positions, set_sizes, Pad, Size, Style, Ui, UiNode, WidgetData,
};
use juquad::SizeInPixels2d;
use macroquad::miniquad::date::now;
use macroquad::prelude::{
    clear_background, is_key_pressed, is_mouse_button_pressed, mouse_position, next_frame,
    screen_height, screen_width, vec2, KeyCode, MouseButton, BLACK,
};
// const COLORING: Coloring = Coloring::new();

struct Buttons {
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
    pub fn render(&self) {
        self.some_text.render();
        self.some_text_2.render();
        self.some_text_3.render();
        self.toggle_alignment.render();
        self.exit.render();
    }
}

#[macroquad::main("juquad button group")]
async fn main() {
    let mut screen = vec2(screen_width(), screen_height());
    let (mut panel, mut buttons) = rebuild_ui(screen);
    loop {
        let start = now();
        let new_screen = vec2(screen_width(), screen_height());
        if new_screen != screen {
            screen = new_screen;
            (panel, buttons) = rebuild_ui(screen);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        buttons.toggle_alignment.interact();
        if buttons.exit.interact().is_clicked() {
            break;
        }

        clear_background(BLACK);
        panel.render();
        buttons.render();

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("{:?}", mouse_position())
        }
        print_time_since(start, "frame took");
        next_frame().await
    }
}

fn rebuild_ui(_screen: SizeInPixels2d) -> (Panel, Buttons) {
    let start = now();

    let font_size: f32 = 22.0;
    let pad = Pad::Symmetric(10.0);
    let style = Style {
        font_size,
        pad,
        margin: pad,
        ..Default::default()
    };

    let text_style: WidgetData = Style {
        font: None,
        ..style
    }
    .into();
    let start_text = now();
    let mut text = Text::new("asdf", text_style);
    let mut text_2 = Text::new("qwer", text_style);
    let mut text_3 = Text::new("QWER", text_style);
    let mut toggle_text = Text::new("Toggle alignment", text_style);
    let mut exit_text = Text::new("Exit", text_style);
    print_time_since(start_text, "time measuring text");

    let button_style: WidgetData = style.into();
    let mut toggle = Button::new(button_style, vec![]);
    let mut exit = Button::new(button_style, vec![]);

    let mut panel = Panel::new(
        Style {
            size: Size::Grow,
            ..style
        }
        .into(),
    );

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
    set_positions(&mut panel_node, vec2(2.0, 2.0));

    toggle.children = vec![Box::new(toggle_text)];
    exit.children = vec![Box::new(exit_text)];
    let buttons = Buttons {
        some_text: text,
        some_text_2: text_2,
        some_text_3: text_3,
        toggle_alignment: toggle,
        exit,
    };
    print_time_since(start, "rebuilt ui in");
    (panel, buttons)
}

fn print_time_since(_start_seconds: f64, _name: &str) {
    let _end = now();
    // println!("{} {:.3} ms", name, (end - start_seconds) * 1000.0);
}
