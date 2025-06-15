//! This example showcases a few use cases of this crate:
//! - How [`TextureLoader`] can be used to load a texture and render a loading screen while waiting for it to load.
//! - How [`TextRect`] and [`Anchor`] can be used to draw text and a bar of similar size.
//! - How to reuse a [`Button`] created once at the beginning.
//! - How to reposition stuff when the window is resized.

use macroquad::miniquad::date::now;
use macroquad::prelude::{
    clear_background, draw_texture_ex, next_frame, screen_height, screen_width, DrawTextureParams,
    FileError, Vec2, DARKGRAY, WHITE,
};

use juquad::draw::draw_rect;
use juquad::fps::Seconds;
use juquad::texture_loader::TextureLoader;
use juquad::widgets::anchor::{Anchor, Horizontal};
use juquad::widgets::button::{Button};
use juquad::widgets::text::TextRect;
use juquad::widgets::{Style, Widget};

const FONT_SIZE: f32 = 32.0;

#[macroquad::main("Hello juquad")]
async fn main() -> Result<(), FileError> {
    let style: Style = Style::new();
    let mut loader = TextureLoader::new(&["assets/ferris.png"]);
    let mut textures_opt = None;
    let mut frame = 0;
    let mut button = Button::new("Reload", Anchor::top_left(0.0, 0.0), 16.0);
    let mut previous_time: Seconds = now();
    loop {
        frame += 1;
        let fps = calculate_fps(&mut previous_time);

        clear_background(DARKGRAY);
        let center = Vec2::new(screen_width() * 0.5, screen_height() * 0.5);
        match &textures_opt {
            None => {
                let rect = TextRect::new("Loading...", Anchor::center_v(center), 32.0);
                rect.render_text(WHITE);
                let mut bar_rect = rect.rect;
                bar_rect.y += bar_rect.h;
                bar_rect.w = frame as f32 * 5.0 % rect.rect.w;
                draw_rect(bar_rect, WHITE);

                if let Some(loaded) = loader.get_textures()? {
                    textures_opt = Some(loaded);
                }
            }
            Some(textures) => {
                let dest_size = Vec2::new(300.0, 200.0);
                let position = center - 0.5 * dest_size;
                draw_texture_ex(
                    textures[0],
                    position.x,
                    position.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(dest_size),
                        ..Default::default()
                    },
                );
                let button_position = Anchor::center_v(center + Vec2::new(0.0, 200.0));
                button.reanchor(button_position);
                if button.interact().is_clicked() {
                    textures_opt = None
                }
                button.render_default(&style);

                let fps_pos = Anchor::below(button.rect(), Horizontal::Center, 0.0);
                let text_rect = TextRect::new(&format!("FPS: {:.1}", fps), fps_pos, FONT_SIZE);
                // juquad::draw::draw_rect_lines(text_rect.rect, 2.0, macroquad::prelude::BLACK);
                text_rect.render_text(WHITE);
            }
        }
        next_frame().await
    }
}

fn calculate_fps(previous_time: &mut Seconds) -> Seconds {
    let current_time: Seconds = now();
    let frame_time: Seconds = current_time - *previous_time;
    *previous_time = current_time;
    let fps = 1.0 / frame_time;
    fps
}
