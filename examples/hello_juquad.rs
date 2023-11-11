//! This example showcases a few use cases of this crate:
//! - How [`TextureLoader`] can be used to load a texture and render a loading screen while waiting for it to load.
//! - How [`TextRect`] and [`Anchor`] can be used to draw text and a bar of similar size.
//! - How to reuse a [`Button`] created once at the beginning.
//! - How to reposition stuff when the window is resized.

use macroquad::prelude::{clear_background, draw_texture_ex, next_frame, screen_height, screen_width, DrawTextureParams, FileError, Vec2, DARKGRAY, WHITE, Texture2D};

use juquad::draw::draw_rect;
use juquad::texture_loader::TextureLoader;
use juquad::widgets::anchor::Anchor;
use juquad::widgets::button::{Button, Style};
use juquad::widgets::text::TextRect;

#[macroquad::main("Hello juquad")]
async fn main() -> Result<(), FileError> {
    let style: Style = Style::new();
    let mut loader = TextureLoader::new(&["assets/ferris.png"]);
    let mut textures_opt = None;
    let mut frame = 0;
    let mut button = Button::new("Reload", Anchor::top_left(0.0, 0.0), 16.0);
    loop {
        frame += 1;
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
                button.text_rect.reanchor(button_position);
                if button.interact().is_clicked() {
                    textures_opt = None
                }
                button.render(&style);
            }
        }
        next_frame().await
    }
}
