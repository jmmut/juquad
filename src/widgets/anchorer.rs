use macroquad::math::Rect;
use crate::PixelPosition;

#[derive(Copy, Clone)]
pub enum Alignment {
    Left,
    Center,
    Right
}
#[derive(Copy, Clone)]
pub enum Direction {
    Down(Alignment),
    Right,
    // Up, Left // TODO
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
            Direction::Right => self.current, // TODO
        };
        rect.move_to(new_pos);
        self.move_after(*rect);
    }
}

