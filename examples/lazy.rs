use juquad::lazy::panel::Panel;
use juquad::lazy::text::Text;
use juquad::lazy::{set_positions, set_sizes, Pad, Size, Style, Ui, UiNode};
use juquad::widgets::Style as Coloring;
use juquad::SizeInPixels2d;
use macroquad::prelude::{
    clear_background, is_key_pressed, next_frame, screen_height, screen_width, vec2, KeyCode, BLACK,
};

const COLORING: Coloring = Coloring::new();

struct Buttons {
    //     expand: Button,
    //     increase_font: Button,
    //     decrease_font: Button,
    //     change_font: Button,
    //     toggle_borders: Button,
    some_text: Text,
    //     toggle_alignment: Button,
    //     exit: Button,
}
impl Buttons {
    pub fn render(&self) {
        self.some_text.render();
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

        // draw_rectangle(pad, pad, screen.x - 2.0*pad, screen.y - 2.0*pad, LIGHTGRAY);
        // draw_text("asdf", 100.0, 50.0, font_size, BLACK);
        next_frame().await
    }
}

fn rebuild_ui(ui: &mut Ui, screen: SizeInPixels2d) -> (Panel, Buttons) {
    ui.set_screen_size(screen);
    let font_size: f32 = 16.0;
    let pad = Pad::Symmetric(20.0);
    let mut text = Text::new(
        "asdf",
        Style {
            font_size,
            font: None,
            ..Default::default()
        }
        .into(),
    );
    let text_node = UiNode::leaf(&mut text);
    let mut panel = Panel::new(
        Style {
            pad,
            font_size,
            size: Size::Grow,
            ..Style::default()
        }
        .into(),
    );
    let mut panel_node = UiNode::container(&mut panel, vec![text_node]);
    set_sizes(&mut panel_node);
    set_positions(&mut panel_node, vec2(0.0, 0.0));

    // let container = ui.start_container(panel);
    //
    // panel = container.close();
    (panel, Buttons { some_text: text })
}
