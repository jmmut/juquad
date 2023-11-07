//! Dumb wrappers for 2D drawing. See [`macroquad::shapes`] for more.
//!
//! In Macroquad, position (0, 0) is the top left corner, so x grows to the right, and y grows down.

use macroquad::prelude::{draw_rectangle, draw_rectangle_lines, Color, Rect, Vec2};

/// Draws the border of a rectangle. Higher x and w goes to the right, higher y and h go down.
pub fn draw_rect_lines(rectangle: Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(
        rectangle.x,
        rectangle.y,
        rectangle.w,
        rectangle.h,
        thickness,
        color,
    );
}

/// Draws a solid rectangle. Higher x and w goes to the right, higher y and h go down.
pub fn draw_rect(rectangle: Rect, color: Color) {
    draw_rectangle(rectangle.x, rectangle.y, rectangle.w, rectangle.h, color);
}

/// Draw a line from start to end. In `Vec2{x,y}`, Higher x goes to the right, higher y does down.
pub fn draw_segment(start: Vec2, end: Vec2, thickness: f32, color: Color) {
    macroquad::prelude::draw_line(start.x, start.y, end.x, end.y, thickness, color);
}
