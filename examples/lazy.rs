use juquad::draw::to_rect;
use juquad::lazy::button::Button;
use juquad::lazy::panel::Panel;
use juquad::lazy::slider::Slider;
use juquad::lazy::text::Text;
use juquad::lazy::{
    set_positions, set_sizes, Interactable, Pad, Renderable, RenderableWidget, Size, Style,
    WidgetTrait, WidgetsView, WidgetsViewMut, DEBUG_WIDGETS,
};
use juquad::widgets::anchor::{Anchor, Horizontal, Layout, Spot, Vertical};
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
    toggle_alignment: Button,
    toggle_direction: Button,
    rotate_layout: Button,
    toggle_debug: Button,
    pad_y: Panel,
    exit: Button,
}
impl Buttons {
    pub fn widgets(&self) -> Vec<&dyn RenderableWidget> {
        vec![
            &self.panel,
            &self.some_text,
            &self.pad_y,
            &self.toggle_alignment,
            &self.toggle_direction,
            &self.rotate_layout,
            &self.toggle_debug,
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
            &mut self.pad_y,
            &mut self.toggle_alignment,
            &mut self.toggle_direction,
            &mut self.rotate_layout,
            &mut self.toggle_debug,
            &mut self.exit,
        ]
    }

    fn children(&self) -> WidgetsView<'_> {
        let mut iter = self.widgets().into_iter();
        iter.next();
        iter.collect()
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
impl Interactable for Buttons {}

#[macroquad::main("juquad button group")]
async fn main() {
    let font_size: f32 = 22.0;
    let pad = Pad::new_symmetric(10.0);
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
        let _start = now();
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
            flip_direction(&mut style);
            recalculate_ui = true;
        }
        if buttons.rotate_layout.interact().is_clicked() {
            rotate_layout(&mut style);
            recalculate_ui = true;
        }
        if buttons.toggle_debug.interact().is_clicked() {
            unsafe {
                DEBUG_WIDGETS = !DEBUG_WIDGETS;
            }
            recalculate_ui = true;
        }
        let pad = style.pad.vec2();
        let slider_value: f32 = *buttons.pad_y.interact()[0].downcast_ref().unwrap();
        if !float_eq(slider_value, style.pad.vec2().y, 0.01) {
            style.pad = Pad::new(pad.x, slider_value);
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
        // print_time_since(_start, "frame took");
        next_frame().await
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

fn rebuild_ui(screen: SizeInPixels2d, style: Style) -> Buttons {
    let start = now();

    let text_style = Style {
        font: None,
        pad: Pad::new(style.pad.vec2().x, style.pad.vec2().y * 0.5),
        margin: Pad::new(style.margin.vec2().x, 0.0),
        ..style
    };
    let horizontal_layout = Layout::Horizontal {
        direction: Horizontal::Right,
        alignment: Vertical::Center,
    };
    let button_style = Style {
        layout: horizontal_layout,
        pad: Pad::new_symmetric(0.0),
        ..style
    };
    let slider_container_style = Style {
        pad: Pad::new_symmetric(0.0),
        margin: Pad::new_symmetric(0.0),
        layout: horizontal_layout,
        ..style
    };
    let mut buttons = Buttons {
        panel: Panel::leaf(Style {
            size: Size::Grow,
            ..style
        }),
        some_text: Text::new(text_style, "Title"),
        toggle_alignment: Button::container(
            button_style,
            vec![Box::new(Text::new(text_style, "Toggle alignment"))],
        ),
        toggle_direction: Button::container(
            button_style,
            vec![Box::new(Text::new(text_style, "Toggle direction"))],
        ),
        rotate_layout: Button::container(
            button_style,
            vec![Box::new(Text::new(text_style, "Rotate layout"))],
        ),
        toggle_debug: Button::container(
            button_style,
            vec![Box::new(Text::new(text_style, "Debug widgets"))],
        ),
        pad_y: Panel::container(
            slider_container_style,
            vec![
                Box::new(Text::new(style, "Pad y: ")),
                Box::new(Slider::new(style, 0.0, 50.0, style.pad.vec2().y)),
            ],
        ),
        exit: Button::container(button_style, vec![Box::new(Text::new(text_style, "Exit"))]),
    };

    set_sizes(&mut buttons);
    let screen_rect = to_rect(vec2(0.0, 0.0), screen);
    let anchor = Anchor::inside(screen_rect, style.layout, vec2(0.0, 0.0));
    set_positions(&mut buttons, anchor);

    print_time_since(start, "rebuilt ui in");
    buttons
}

fn print_time_since(_start_seconds: f64, _name: &str) {
    let _end = now();
    println!("{} {:.3} ms", _name, (_end - _start_seconds) * 1000.0);
}
