use juquad::lazy::panel::Panel;
use juquad::lazy::text::Text;
use juquad::lazy::{set_positions, set_sizes, Pad, Size, Style, Ui, UiNode, WidgetData};
use juquad::SizeInPixels2d;
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
    //     toggle_alignment: Button,
    //     exit: Button,
}
impl Buttons {
    pub fn render(&self) {
        self.some_text.render();
        self.some_text_2.render();
        self.some_text_3.render();
    }
}

#[macroquad::main("juquad button group")]
async fn main() {
    let mut ui = Ui::default();
    let mut screen = vec2(screen_width(), screen_height());
    let (mut panel, mut buttons) = rebuild_ui(&mut ui, screen);
    loop {
        let new_screen = vec2(screen_width(), screen_height());
        if new_screen != screen {
            screen = new_screen;
            (panel, buttons) = rebuild_ui(&mut ui, screen);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        clear_background(BLACK);
        panel.render();
        buttons.render();
        // panel with pad
        // centered text

        if is_mouse_button_pressed(MouseButton::Left) {
            println!("{:?}", mouse_position())
        }
        // draw_rectangle(pad, pad, screen.x - 2.0*pad, screen.y - 2.0*pad, LIGHTGRAY);
        // draw_text("asdf", 100.0, 50.0, font_size, BLACK);
        next_frame().await
    }
}

fn rebuild_ui(ui: &mut Ui, screen: SizeInPixels2d) -> (Panel, Buttons) {
    ui.set_screen_size(screen);
    let font_size: f32 = 64.0;
    let pad = Pad::Symmetric(40.0);
    let text_style: WidgetData = Style {
        font_size,
        font: None,
        ..Default::default()
    }
        .into();
    let mut text = Text::new("asdf", text_style.clone());
    let mut text_2 = Text::new("qwer", text_style.clone());
    let mut text_3 = Text::new("QWER", text_style.clone());
    let text_node = UiNode::leaf(&mut text);
    let text_node_2 = UiNode::leaf(&mut text_2);
    let text_node_3 = UiNode::leaf(&mut text_3);
    let mut panel = Panel::new(
        Style {
            pad,
            font_size,
            size: Size::Grow,
            ..Style::default()
        }
        .into(),
    );
    let mut panel_node = UiNode::container(&mut panel, vec![text_node, text_node_2, text_node_3]);
    set_sizes(&mut panel_node);
    set_positions(&mut panel_node, vec2(2.0, 2.0));

    // let container = ui.start_container(panel);
    //
    // panel = container.close();
    (
        panel,
        Buttons {
            some_text: text,
            some_text_2: text_2,
            some_text_3: text_3,
        },
    )
}
