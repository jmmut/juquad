use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, RawWaker, RawWakerVTable, Waker};

use macroquad::prelude::{load_image, load_texture, FileError, Image, Texture2D};

/// Loads textures semi-asynchronously, so that you can render a loading screen.
///
/// This is not fully asynchronous because once the texture is loaded, there may be a format
/// conversion that will be blocking. Still, using this struct is an improvement compared to
/// blocking during both the load and the format conversion.
///
/// I have tested that this struct works as expected in linux and wasm. (Browsers were particularly
/// prone to suffer from the blocking during load).
///
/// See [`examples/hello_juquad.rs:36`] for an example of how to do a loading screen while waiting
/// for this to load.
pub struct TextureLoader<T> {
    texture_paths: &'static [&'static str], // TODO: if I make these non-static, it doesn't compile because the struct must outlive the in_progress pin ???
    textures: Vec<T>,
    in_progress: Option<Pin<Box<dyn Future<Output = Result<T, FileError>>>>>,
    load_func: fn(&str) -> Pin<Box<dyn Future<Output = Result<T, FileError>> + '_>>,
}

pub struct Progress {
    pub loaded: usize,
    pub total_to_load: usize,
}

fn pinned_load_texture(
    path: &str,
) -> Pin<Box<dyn Future<Output = Result<Texture2D, FileError>> + '_>> {
    Box::pin(load_texture(path))
}
fn pinned_load_image(path: &str) -> Pin<Box<dyn Future<Output = Result<Image, FileError>> + '_>> {
    Box::pin(load_image(path))
}

impl TextureLoader<Texture2D> {
    pub fn new(texture_paths: &'static [&'static str]) -> Self {
        Self {
            texture_paths,
            textures: Vec::new(),
            in_progress: None,
            load_func: pinned_load_texture,
        }
    }
}
impl TextureLoader<Image> {
    pub fn new_from_image(texture_paths: &'static [&'static str]) -> Self {
        Self {
            texture_paths,
            textures: Vec::new(),
            in_progress: None,
            load_func: pinned_load_image,
        }
    }
}
impl<T: 'static> TextureLoader<T> {
    pub fn get_progress(&self) -> Progress {
        Progress {
            loaded: self.textures.len(),
            total_to_load: self.texture_paths.len(),
        }
    }

    /// returns Ok(None) until all textures are loaded, and then returns Ok(Some(textures))
    /// returns Err() if a file couldn't be read for any reason
    pub fn get_textures(&mut self) -> Result<Option<Vec<T>>, FileError> {
        if self.textures.len() < self.texture_paths.len() {
            // more textures to load
            let next_unloaded_index = self.textures.len();
            if let Some(in_progress) = &mut self.in_progress {
                // the loading of some texture was started
                match resume(in_progress) {
                    Some(texture_res) => {
                        // the texture finished loading
                        let texture = texture_res?;
                        self.textures.push(texture);
                        self.in_progress = None;
                    }
                    None => {
                        // the texture is still loading
                    }
                }
            } else {
                // no texture is loading
                let texture_fut = (self.load_func)(&self.texture_paths[next_unloaded_index]);
                let texture_pin = Box::pin(texture_fut);
                self.in_progress = Some(texture_pin);
            }
            Ok(None)
        } else {
            // finished loading textures
            let mut textures = Vec::new();
            std::mem::swap(&mut textures, &mut self.textures);
            Ok(Some(textures))
        }
    }
}

// resume() and waker() taken from macroquad::exec. I don't understand why they are private

/// returns Some(T) if future is done, None if it would block
fn resume<T>(future: &mut Pin<Box<dyn Future<Output = T>>>) -> Option<T> {
    let waker = waker();
    let mut futures_context = std::task::Context::from_waker(&waker);
    match future.as_mut().poll(&mut futures_context) {
        Poll::Ready(v) => Some(v),
        Poll::Pending => None,
    }
}
fn waker() -> Waker {
    unsafe fn clone(data: *const ()) -> RawWaker {
        RawWaker::new(data, &VTABLE)
    }
    unsafe fn wake(_data: *const ()) {
        panic!(
            "macroquad does not support waking futures, please use coroutines, \
            otherwise your pending future will block until the next frame"
        )
    }
    unsafe fn wake_by_ref(data: *const ()) {
        wake(data)
    }
    unsafe fn drop(_data: *const ()) {
        // Nothing to do
    }
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    let raw_waker = RawWaker::new(std::ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}
