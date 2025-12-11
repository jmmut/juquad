use std::cmp::max;
use macroquad::prelude::{clear_background, draw_rectangle, draw_text, is_key_pressed, next_frame, screen_height, screen_width, vec2, KeyCode, Rect, BLACK, LIGHTGRAY};
use juquad::draw::draw_rect;
use juquad::lazy::button::Button;
use juquad::lazy::{AsWidget, Pad, Panel, Size, Style, Text, Ui, Widget};
use juquad::{PositionInPixels2d, SizeInPixels2d};
use juquad::widgets::anchor::Layout;
use juquad::widgets::Style as Coloring;
use juquad::widgets::text::TextRect;

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

pub struct UiNode<'a> {
    node: &'a mut dyn Widget,
    children: Vec<UiNode<'a>>,
}

impl<'a> UiNode<'a> {
    pub fn leaf(node: &'a mut dyn Widget) -> UiNode<'a> {
        Self {
            node,
            children: Vec::new(),
        }
    }
    pub fn container(node: &'a mut dyn Widget, children: Vec<UiNode<'a>>) -> Self {
        Self {
            node,
            children,
        }
    }
}

fn rebuild_ui(ui: &mut Ui, screen: SizeInPixels2d) -> (Panel, Buttons) {
    ui.set_screen_size(screen);
    let font_size: f32 = 16.0;
    let pad = Pad::Symmetric(20.0);
    let mut text = Text::new("asdf", Style {
        font_size,
        font: None,
        ..Default::default()
    }.into());
    let text_node = UiNode::leaf(&mut text);
    let mut panel = Panel::new(Style {
            pad,
            font_size,
            size: Size::Grow,
            ..Style::default()
        }.into());
    let mut panel_node = UiNode::container(&mut panel, vec![text_node]);
    set_sizes(&mut panel_node);
    set_positions(&mut panel_node, vec2(0.0, 0.0));
    
    // let container = ui.start_container(panel);
    // 
    // panel = container.close();
    (panel, Buttons {some_text: text})
}

fn set_sizes(node: &mut UiNode) {
    let mut accumulated_size = SizeInPixels2d::new(0.0, 0.0);
    for child in &mut node.children {
        set_sizes(child);
        let style = child.node.style();
        let parallel = style.layout.parallel_index();
        let perpendicular = style.layout.perpendicular_index();
        let size = child.node.size();
        let margin = style.margin.vec2();
        accumulated_size[parallel] += size[parallel] + margin[parallel];
        accumulated_size[perpendicular] = accumulated_size[perpendicular].max(size[perpendicular] + margin[perpendicular]);
    }
    accumulated_size += 2.0*node.node.style().pad.vec2();
    node.node.set_size(accumulated_size);
}
fn set_positions(node: &mut UiNode, pos: PositionInPixels2d) {
    let mut accumulated_pos = pos;
    accumulated_pos += node.node.style().margin.vec2();
    node.node.set_pos(accumulated_pos);
    let pad = node.node.style().pad.vec2();
    accumulated_pos += pad;
    for child in &mut node.children {
        let style = child.node.style();
        let parallel = style.layout.parallel_index();
        set_positions(child, accumulated_pos);
        accumulated_pos[parallel] += child.node.size()[parallel];
    }
}

