use crate::elm::style::Style;
use crate::elm::widget::Widget;
use crate::lazy::text::size_text_generic;
use crate::widgets::StateColor;
use macroquad::math::Vec2;
use macroquad::prelude::{Font, TextParams};

pub type Text = Widget<TextBase>;

pub struct TextBase {
    text: String,
    reference_height: f32,
}

impl Widget<TextBase> {
    pub fn new<Str: Into<String>, Sty: Into<Style>>(style: Sty, text: Str) -> Widget<TextBase> {
        let style = style.into();
        let text = text.into();
        let size = size_text_generic(
            &text,
            macroquad::prelude::measure_text,
            style.font.as_ref(),
            style.font_size,
        );

        let custom = TextBase {
            text: text.into(),
            reference_height: size.y,
        };

        let size = Some(size + 2.0 * style.pad.vec2());
        Text {
            style,
            size,
            pos: Default::default(),
            custom,
        }
    }

    pub fn render(&self) {
        draw_text(
            &self.custom.text,
            self.pos,
            self.style.pad.vec2(),
            self.custom.reference_height,
            self.style.font_size,
            self.style.font.as_ref(),
            self.style.coloring.at_rest,
        );
    }
}

/// Here the position is of the border. The top left corner of the text is at pos + pad.
pub fn draw_text(
    text: &str,
    pos: Vec2,
    pad: Vec2,
    reference_height: f32,
    font_size: f32,
    font: Option<&Font>,
    state_color: StateColor,
) {
    // macroquad's draw_text() draws from the baseline of the text
    // https://en.wikipedia.org/wiki/Baseline_(typography)
    // I don't use self.text_dimensions.offset_y because that changes depending on the letters,
    // so I prefer an approximate distance that makes all buttons at the same baseline
    let approx_height_from_baseline_to_top = 0.85 * reference_height;

    let text_pos = pos + pad;
    let x = text_pos.x.round();
    let y = (text_pos.y + approx_height_from_baseline_to_top).round();

    let params = TextParams {
        font,
        font_size: font_size as u16,
        color: state_color.text_color,
        ..TextParams::default()
    };
    macroquad::text::draw_text_ex(text, x, y, params);
}
