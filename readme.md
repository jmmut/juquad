# juquad
## jmmut's utilities for Macroquad

### Quickstart

After you set up a project (`cargo new hello_juquad && cd hello_juquad`), add these dependencies in your Cargo.toml:
```toml
[dependencies]
juquad = { git = "https://github.com/jmmut/juquad.git" }
macroquad = "0.3.24"
```
(I recommend adding a tag `juquad = { git = "https://github.com/jmmut/juquad.git", tag = "0.1.0" }` for a particular version. Note that I won't respect semver until this lib has more than 1 user, so expect breaking changes regardless of the type of the version bump I do.)

and then put this in your main.rs:
```rust
use macroquad::prelude::{
    clear_background, next_frame, screen_height, screen_width, FileError, BLACK, WHITE,
};

use juquad::widgets::anchor::Anchor;
use juquad::widgets::text::TextRect;

#[macroquad::main("Hello juquad")]
async fn main() -> Result<(), FileError> {
    loop {
        clear_background(BLACK);
        let center = Anchor::center(screen_width() * 0.5, screen_height() * 0.5);
        TextRect::new("Hello juquad!", center, 32.0).render_text(WHITE);
        next_frame().await
    }
}
```

and you should be able to just do `cargo run`, and get a resizable window rendering some text at high FPS.

### More info

See [examples/hello_juquad.rs](examples/hello_juquad.rs) for a showcase of the structs and functions in this crate. Note that that example uses a texture from `assets/ferris.png`, so make sure the relative path exists from wherever you run it.
