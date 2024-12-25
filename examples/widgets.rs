use macroquad::prelude::{Color, next_frame, Rect, screen_height, screen_width};
use juquad::{PixelPosition, SizeInPixels};
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, InteractionStyle, Style};
use juquad::widgets::button_group::ButtonGroup;
use juquad::widgets::Widget;

const FONT_SIZE: f32 = 16.0;
const STYLE: Style = Style::new();

#[macroquad::main("juquad widgets")]
async fn main() {
    // let anchor = Anchor::default();
    let anchor = Anchor::center(screen_width() * 0.5, screen_height() * 0.125);
    let mut button_enable = Button::new("toggle many buttons", anchor, FONT_SIZE);
    let mut alignment = Alignment::Left;
    let mut buttons_enabled = false;
    loop {
        let mut anchorer = Anchorer::new(Direction::Down(alignment));
        anchorer.move_after(button_enable.rect());
        let mut button_1 = Button::new("toggle center", Anchor::default(), FONT_SIZE);
        anchorer.move_and_modify(&mut button_1.text_rect.rect);
        if button_1.interact().is_clicked() {
            if let Alignment::Left = alignment {
                alignment = Alignment::Center;
            } else {
                alignment = Alignment::Left;
            }
        }
        let mut extra_buttons = Vec::new();
        if button_enable.interact().is_clicked() {
            buttons_enabled = !buttons_enabled;
        }
        if buttons_enabled {
            let texts = vec!["some long button text", "button 3"];
            for text in texts {
                let mut button = Button::new(text, Anchor::default(), FONT_SIZE);
                anchorer.move_and_modify(&mut button.text_rect.rect);
                button.interact();
                extra_buttons.push(button);
            }
        }

        // render
        button_enable.render(&STYLE);
        button_1.render(&STYLE);
        if buttons_enabled {
            for extra_button in extra_buttons {
                extra_button.render(&STYLE);
            }
        }
        next_frame().await
    }
}

#[derive(Copy, Clone)]
enum Alignment {
    Left,
    Center,
    Right
}
#[derive(Copy, Clone)]
enum Direction {
    Down(Alignment),
    Right,
    // Up, Left
}

pub struct Anchorer {
    current: PixelPosition,
    direction: Direction
}
impl Anchorer {
    pub fn new(direction: Direction) -> Self {
        Anchorer {
            current: PixelPosition::default(),
            direction,
        }
    }
    pub fn move_after(&mut self, rect: Rect) {
        match self.direction {
            Direction::Down(Alignment::Left) => {
                self.current.x = rect.x;
                self.current.y = rect.y + rect.h;
            }
            Direction::Down(Alignment::Center) => {
                self.current.x = rect.x + rect.w * 0.5;
                self.current.y = rect.y + rect.h;
            },
            Direction::Down(Alignment::Right) => todo!(),
            Direction::Right => {
                self.current.x = rect.x + rect.w;
                self.current.y = rect.y;
            }
        }
    }
    pub fn move_and_modify(&mut self, rect :&mut Rect) {
        let new_pos = match self.direction {
            Direction::Down(Alignment::Left) => self.current,
            Direction::Down(Alignment::Center) => {
                let mut pos = self.current;
                pos.x -= rect.w * 0.5;
                pos
            }.round(),
            Direction::Down(Alignment::Right) => {
                let mut pos = self.current;
                pos.x -= rect.w;
                pos
            }.round(),
            Direction::Right => self.current,
        };
        rect.move_to(new_pos);
        self.move_after(*rect);
    }
}

