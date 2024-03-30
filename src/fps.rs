use macroquad::miniquad::date::now;
use macroquad::prelude::next_frame;

pub type Seconds = f64;

pub async fn sleep_until_next_frame(previous_time: &mut Seconds) {
    #[cfg(not(target_family = "wasm"))]
    {
        const MAX_FPS: f64 = 80.0;
        const FRAME_PERIOD: f64 = 1.0 / MAX_FPS;
        let new_time = now();
        // dbg!(new_time);
        // dbg!(*previous_time);
        let frame_duration = new_time - *previous_time;
        if frame_duration < FRAME_PERIOD {
            let sleep_secs = FRAME_PERIOD - frame_duration;
            // info!("sleeping for {}", sleep_secs);

            // this is a blocking sleep on purpose. My current understanding is that macroquad
            // relies on OS or GPU drivers to limit the FPS to ~60 on non-wasm, which doesn't always
            // work. I was experiencing ~8000 FPS and this is the only way I know to limit them.
            // This may not work in web.
            std::thread::sleep(std::time::Duration::from_secs_f64(sleep_secs));
        }
    }
    next_frame().await;
    *previous_time = now();
}
