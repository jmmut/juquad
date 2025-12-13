use juquad::lazy::button::{Button, ButtonBase};
use juquad::lazy::panel::Panel;
use juquad::lazy::text::Text;
use juquad::lazy::{
    container, leaf, set_positions, set_sizes, Pad, Renderable, RenderableWidget, Size, Style,
    WidgetData, WidgetTrait, WidgetsView, WidgetsViewMut,
};
use juquad::widgets::anchor::{Horizontal, Layout, Vertical};
use juquad::widgets::Interaction;
use juquad::{PositionInPixels2d, SizeInPixels2d};
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
    toggle_direction: Button,
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
            &self.toggle_direction,
            &self.exit,
        ]
    }
}

impl WidgetTrait for Buttons {
    fn size(&self) -> SizeInPixels2d {
        self.panel.size()
    }

    fn pos(&self) -> PositionInPixels2d {
        self.panel.pos()
    }

    fn set_pos(&mut self, position: PositionInPixels2d) {
        self.panel.set_pos(position)
    }

    fn set_size(&mut self, size: SizeInPixels2d) {
        self.panel.set_size(size)
    }

    fn style(&self) -> &Style {
        self.panel.style()
    }

    fn children_mut(&mut self) -> WidgetsViewMut<'_> {
        vec![
            &mut self.some_text,
            &mut self.some_text_2,
            &mut self.some_text_3,
            &mut self.toggle_alignment,
            &mut self.toggle_direction,
            &mut self.exit,
        ]
    }

    fn children(&self) -> WidgetsView<'_> {
        vec![
            &self.some_text,
            &self.some_text_2,
            &self.some_text_3,
            &self.toggle_alignment,
            &self.toggle_direction,
            &self.exit,
        ]
    }
}
impl Renderable for Buttons {
    fn render_interactive(&self, _interaction: Interaction) {
        self.render()
    }
    fn render(&self) {
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
            rotate_alignment(&mut style);
            recalculate_ui = true;
        }
        if buttons.toggle_direction.interact().is_clicked() {
            rotate_direction(&mut style);
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

fn rotate_alignment(style: &mut Style) {
    match &mut style.layout {
        Layout::Horizontal { alignment, .. } => *alignment = next_v(*alignment),
        Layout::Vertical { alignment, .. } => *alignment = next_h(*alignment),
    }
}

fn rotate_direction(style: &mut Style) {
    match &mut style.layout {
        Layout::Horizontal { direction, .. } => *direction = next_h(*direction),
        Layout::Vertical { direction, .. } => *direction = next_v(*direction),
    }
}

fn next_v(direction: Vertical) -> Vertical {
    match direction {
        Vertical::Top => Vertical::Center,
        Vertical::Center => Vertical::Bottom,
        Vertical::Bottom => Vertical::Top,
    }
}

fn next_h(direction: Horizontal) -> Horizontal {
    match direction {
        Horizontal::Left => Horizontal::Center,
        Horizontal::Center => Horizontal::Right,
        Horizontal::Right => Horizontal::Left,
    }
}

fn rebuild_ui(screen: SizeInPixels2d, style: Style) -> Buttons {
    let start = now();

    let text_style = Style {
        font: None,
        ..style
    };
    print_time_since(now(), "time measuring text");

    let mut buttons = Buttons {
        panel: Panel::leaf(Style {
            size: Size::Grow,
            ..style
        }),
        some_text: Text::new_text(text_style, "asdf"),
        some_text_2: Text::new_text(text_style, "qwer"),
        some_text_3: Text::new_text(text_style, "QWER"),
        toggle_alignment: Button::container(
            style,
            vec![Box::new(Text::new_text(text_style, "Toggle alignment"))],
        ),
        toggle_direction: Button::container(
            style,
            vec![Box::new(Text::new_text(text_style, "Toggle direction"))],
        ),
        exit: Button::container(style, vec![Box::new(Text::new_text(text_style, "Exit"))]),
    };

    set_sizes(&mut buttons);
    set_positions(&mut buttons, vec2(0.0, 0.0), screen, &style);

    print_time_since(start, "rebuilt ui in");
    buttons
}

fn print_time_since(_start_seconds: f64, _name: &str) {
    let _end = now();
    // println!("{} {:.3} ms", name, (end - start_seconds) * 1000.0);
}
