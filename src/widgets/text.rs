use crate::draw::draw_rect_lines;
use crate::widgets::anchor::Anchor;
use macroquad::color::BLUE;
use macroquad::prelude::{Color, Font, Rect, TextDimensions, Vec2};
use macroquad::text::TextParams;
use std::ops::AddAssign;

pub type Pixels = f32;

pub const ALERT_COLOR: Color = Color::new(0.98, 0.95, 0.3, 1.00);
pub const TEXT_PANEL_COLOR: Color = Color::new(1.0, 0.97, 0.8, 1.00);

pub type DrawText =
    fn(text: &str, x: f32, y: f32, font_size: f32, color: Color, font: Option<Font>);
pub type MeasureText =
    fn(text: &str, font: Option<Font>, font_size: u16, font_scale: f32) -> TextDimensions;

/// Renders some text in some anchor position.
///
/// The anchor position allows specifying what the position means. For example,
/// ```no_run
/// use macroquad::prelude::{Vec2, BLACK};
/// use juquad::widgets::{anchor::Anchor, text::TextRect};
///
/// // the text will be rendered so that its center is at (100.0, 200.0)
/// TextRect::new("some text", Anchor::center(100.0, 200.0), 16.0).render_text(BLACK);
/// ```
///
/// The struct also provides the text rectangle so that you can do buttons or frames manually,
/// or just know how big it is, possibly creating it only once:
/// ```no_run
/// use macroquad::prelude::{Vec2, LIGHTGRAY, DARKGRAY, BLACK};
/// use juquad::widgets::{anchor::Anchor, text::TextRect};
/// use juquad::draw::{draw_rect, draw_rect_lines};
///
/// // in initialization, stored somewhere
/// let mut text_rect = TextRect::new("some tooltip", Anchor::top_left(0.0, 0.0), 16.0);
///
/// // pass it to this function to render it next to the mouse with a small panel
/// pub fn draw_tooltip(text_rect: &mut TextRect, mouse_position: Vec2) {
///     text_rect.reanchor(Anchor::bottom_left_v(mouse_position));
///     draw_rect(text_rect.rect, LIGHTGRAY);
///     draw_rect_lines(text_rect.rect, 2.0, DARKGRAY);
///     text_rect.render_text(BLACK);
/// }
/// ```
#[derive(Clone)]
pub struct TextRect {
    pub text: String,
    pub rect: Rect,
    pub font_size: f32,
    pub font: Option<Font>,
    pub pad: Vec2,
    pub offset_y: f32,
    pub text_width: f32,
    pub text_height: f32,
    pub draw_text: DrawText,
}
impl TextRect {
    pub fn new(text: &str, position_pixels: Anchor, font_size: f32) -> Self {
        let measure_text = macroquad::prelude::measure_text;
        Self::new_generic(
            text,
            position_pixels,
            font_size,
            None,
            measure_text,
            draw_text,
        )
    }

    pub fn new_generic(
        text: &str,
        position_pixels: Anchor,
        font_size: f32,
        font: Option<Font>,
        measure_text: MeasureText,
        draw_text: DrawText,
    ) -> Self {
        let text_dimensions = measure_text(text, font, font_size as u16, 1.0);

        let pad = Vec2::new(font_size, font_size * 0.25);
        let size = Vec2::new(
            (text_dimensions.width + pad.x * 2.0).round(),
            (font_size + pad.y * 2.0).round(),
        );
        let top_left = position_pixels.get_top_left_pixel(size);

        let rect = Rect::new(top_left.x, top_left.y, size.x, size.y);
        Self {
            text: text.to_string(),
            rect,
            font_size,
            font,
            pad,
            offset_y: text_dimensions.offset_y,
            text_width: text_dimensions.width,
            text_height: text_dimensions.height,
            draw_text,
        }
    }

    pub fn reanchor(&mut self, position_pixels: Anchor) -> &mut Self {
        let top_left = position_pixels.get_top_left_pixel(self.rect.size());
        self.rect.x = top_left.x;
        self.rect.y = top_left.y;
        self
    }

    pub fn render_text(&self, color: Color) {
        // draw_text() draws from the baseline of the text
        // https://en.wikipedia.org/wiki/Baseline_(typography)
        // I don't use self.text_dimensions.offset_y because that changes depending on the letters,
        // so I prefer an approximate distance that makes all buttons at the same baseline
        let approx_height_from_baseline_to_top = 0.75 * self.font_size;

        (self.draw_text)(
            &self.text,
            (self.rect.x + self.pad.x).round(),
            (self.rect.y + self.pad.y + approx_height_from_baseline_to_top).round(),
            self.font_size,
            color,
            self.font,
        );
    }
}

/// A nice combo is:
/// ```no_run
/// use macroquad::prelude::BLACK;
/// use juquad::widgets::anchor::Anchor;
/// use juquad::widgets::text::{draw_text_lines, wrap_or_hide_text};
///
/// let text = "long, potentially multiline\ntext.";
/// let line_height = 20.0;
/// let font_size = 16.0;
/// let lines = wrap_or_hide_text(text, font_size, line_height, 150.0, 100.0);
/// draw_text_lines(lines, Anchor::center(300.0, 50.0), font_size, line_height, BLACK)
/// ```
pub fn draw_text_lines(
    text_lines: Vec<String>,
    mut position: Anchor,
    font_size: f32,
    line_height: f32,
    color: Color,
) {
    for line in text_lines {
        TextRect::new(&line, position, font_size).render_text(color);
        position.offset(0.0, line_height);
    }
}

pub fn draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color, font: Option<Font>) {
    if let Some(font) = font {
        let params = TextParams {
            font,
            font_size: font_size as u16,
            color,
            ..TextParams::default()
        };
        macroquad::text::draw_text_ex(text, x, y, params)
    } else {
        macroquad::text::draw_text(text, x, y, font_size, color)
    }
}

pub fn wrap_or_hide_text(
    text: &str,
    font_size: f32,
    line_height: Pixels,
    panel_width: Pixels,
    panel_height: Pixels,
) -> Vec<String> {
    wrap_or_hide_text_generic(
        text,
        font_size,
        line_height,
        panel_width,
        panel_height,
        &macroquad::prelude::measure_text,
    )
}

#[allow(unused)]
pub fn wrap_or_hide_text_mocked(
    text: &str,
    font_size: f32,
    line_height: Pixels,
    panel_width: Pixels,
    panel_height: Pixels,
) -> Vec<String> {
    wrap_or_hide_text_generic(
        text,
        font_size,
        line_height,
        panel_width,
        panel_height,
        &|text, _font, font_size, _scale| {
            return TextDimensions {
                width: text.len() as f32 * font_size as f32,
                height: font_size as f32,
                offset_y: font_size as f32,
            };
        },
    )
}

pub fn wrap_or_hide_text_generic<F>(
    text: &str,
    font_size: f32,
    line_height: Pixels,
    panel_width: Pixels,
    panel_height: Pixels,
    measure_text: &F,
) -> Vec<String>
where
    F: Fn(&str, Option<Font>, u16, f32) -> TextDimensions,
{
    if panel_width < 0.0 || panel_height < 0.0 {
        return Vec::new();
    }
    assert!(panel_width >= 0.0);
    assert!(panel_height >= 0.0);
    if text.is_empty() {
        return vec!["".to_string()];
    }

    let lines = text.split("\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let dimensions = lines
        .iter()
        .map(|line| measure_text(line, None, font_size as u16, 1.0))
        .collect::<Vec<_>>();
    let max_width = dimensions
        .iter()
        .map(|d| d.width)
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap();
    let max_height = font_size;
    if line_height.max(max_height) > panel_height {
        return Vec::new(); // not enough space for a single line, hide all text
    } else if max_width <= panel_width && max_height * lines.len() as f32 <= panel_height {
        return lines;
    } else {
        let mut result: Vec<String> = Vec::new();
        for (line, line_dimensions) in lines.iter().zip(dimensions) {
            let mut remaining_text = line.as_str();
            let letter_width_estimate: Pixels = line_dimensions.width / remaining_text.len() as f32;
            let letters_per_line_estimate = (panel_width / letter_width_estimate).trunc() as usize;
            loop {
                if (result.len() + 1) as f32 * line_height > panel_height {
                    let mut last_line = result.pop().unwrap();
                    // lines will usually end in a space, so the index points to the letter before the last one
                    let mut last_letter_in_last_word_utf8 = last_line.len() - 2;
                    while !last_line.is_char_boundary(last_letter_in_last_word_utf8) {
                        last_letter_in_last_word_utf8 -= 1;
                    }
                    let line_break_index = last_line[..last_letter_in_last_word_utf8].rfind(" ");
                    let mut last_line = if let Some(previous_word_index) = line_break_index {
                        last_line[..previous_word_index].to_string()
                    } else {
                        last_line.pop();
                        last_line.pop();
                        last_line.pop();
                        last_line
                    };
                    last_line.add_assign("...");
                    result.push(last_line);
                    break;
                }
                if remaining_text.len() <= letters_per_line_estimate {
                    result.push(remaining_text.to_string());
                    break;
                } else {
                    let mut letters_per_line_estimate_utf8 = letters_per_line_estimate;
                    while !remaining_text.is_char_boundary(letters_per_line_estimate_utf8 + 1) {
                        letters_per_line_estimate_utf8 -= 1;
                    }
                    let line_break_index = remaining_text[0..=letters_per_line_estimate_utf8]
                        .rfind(" ")
                        .unwrap_or(letters_per_line_estimate_utf8 - 1); // TODO: put a dash for cut words?
                    result.push(remaining_text[0..=line_break_index].to_string());
                    remaining_text = &remaining_text[(line_break_index + 1)..];
                    if remaining_text.is_empty() {
                        break;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_basic() {
        let text = "word_1 word_2 word_3";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size - 1.0,
            font_size * 3.0,
        );
        assert_eq!(lines, vec!["word_1 word_2 ", "word_3"]);
    }

    #[test]
    fn test_wrap_text_ellipsis() {
        let text = "word_1 word_2 word_3";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size - 1.0,
            font_size * 1.5,
        );
        assert_eq!(lines, vec!["word_1..."]);
    }
    #[test]
    fn test_wrap_text_no_space() {
        let text = "word_1 word_2 word_3";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size - 1.0,
            font_size * 0.5,
        );
        assert_eq!(lines, Vec::<String>::new());
    }
    #[test]
    fn test_wrap_text_long_word() {
        let text = "looooooooooooooooooooooong_word";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size - 1.0,
            font_size * 1.5,
        );
        assert_eq!(lines, vec!["looooooooooooooooooooooong_..."]);
    }

    #[test]
    fn test_wrap_text_respect_newlines() {
        let text = "first line\nsecond line";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            text.len() as f32 * font_size,
            font_size * 3.0,
        );
        assert_eq!(lines, vec!["first line", "second line"]);
    }

    #[test]
    fn test_wrap_text_respect_newlines_short_lines() {
        let text = "first line with many words\nsecond line with many words as well";
        let font_size = 10.0;
        let lines = wrap_or_hide_text_mocked(
            text,
            font_size,
            font_size,
            "first line with many wo".len() as f32 * font_size,
            font_size * 4.0,
        );
        assert_eq!(
            lines,
            vec![
                "first line with many ",
                "words",
                "second line with many ",
                "words as well"
            ]
        );
    }
}
