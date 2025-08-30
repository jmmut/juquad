use std::future::Future;
use std::pin::Pin;
use std::task::{Poll, RawWaker, RawWakerVTable, Waker};
use macroquad::file::FileError;
use macroquad::texture::{load_texture, Texture2D};

pub type TextureLoaderAlias<'a> = ResourceLoader<
    'a,
    &'a str,
    Texture2D,
    FileError,
    fn(&'a str) -> std::pin::Pin<Box<dyn Future<Output = Result<Texture2D, FileError>> + 'a>>,
    std::pin::Pin<Box<dyn Future<Output = Result<Texture2D, FileError>> + 'a>>,
>;
impl<'a> TextureLoaderAlias<'a> {
    #[deprecated = "use ResourceLoader.get_resources() resources"]
    pub fn get_textures(&mut self) -> Result<Option<Vec<Texture2D>>, FileError> {
        self.get_resources()
    }
}
pub struct TextureLoader;
impl TextureLoader {
    pub fn new<'a>(inputs: &'a [&'a str]) -> TextureLoaderAlias<'a> {
        ResourceLoader::new(|path| Box::pin(load_texture(path)), inputs)
    }
}
// pub trait TextureLoader {
//     fn new<'a, Func: Fn(&'a str) -> Fut, Fut: Future<Output = Result<Texture2D, FileError>> + 'a>(
//         paths: &'a[&'a str]
//     ) -> ResourceLoader<&'a str, Texture2D, FileError, Func, Fut> {
//         ResourceLoader::new(load_texture, paths)
//     }
// }
// fn new_texture_loader<'a, Func: Fn(&'a str) -> Fut, Fut: Future<Output = Result<Texture2D, FileError>> + 'a>(paths: &'a [&'a str; 1]) -> ResourceLoader<'a, &'a str, Texture2D, FileError, Func, Fut> {
//     ResourceLoader::new(|path|load_texture(path), paths)
// }


/// Loads resources semi-asynchronously, so that you can render a loading screen.
///
/// This is not fully asynchronous because once the resource is loaded, there may be a format
/// conversion that will be blocking. Still, using this struct is an improvement compared to
/// blocking during both the load and the format conversion.
///
/// I have tested that this struct works as expected in linux and wasm. (Browsers were particularly
/// prone to suffer from the blocking during load).
///
/// See [`examples/hello_juquad.rs:36`] for an example of how to do a loading screen while waiting
/// for this to load.
pub struct ResourceLoader<'a, In, Out, Er, Func, Fut>
where
    Func: Fn(In) -> Fut,
    Fut: Future<Output = Result<Out, Er>> + 'a,
{
    resources_input: &'a [In],
    resources: Vec<Out>,
    in_progress: Option<Pin<Box<Fut>>>,
    load_func: Func,
}

pub struct Progress {
    pub loaded: usize,
    pub total_to_load: usize,
}

impl<'a, I, O, E, F, Fut> ResourceLoader<'a, I, O, E, F, Fut>
where
    F: Fn(I) -> Fut,
    Fut: Future<Output = Result<O, E>> + 'a,
    I: Copy,
{
    pub fn new(load_func: F, resources_bytes: &'a [I]) -> Self {
        Self {
            resources_input: resources_bytes,
            resources: Vec::new(),
            in_progress: None,
            load_func,
        }
    }

    pub fn get_progress(&self) -> Progress {
        Progress {
            loaded: self.resources.len(),
            total_to_load: self.resources_input.len(),
        }
    }

    /// returns Ok(None) until all textures are loaded, and then returns Ok(Some(textures))
    /// returns Err() if a file couldn't be read for any reason
    pub fn get_resources(&mut self) -> Result<Option<Vec<O>>, E> {
        if self.resources.len() < self.resources_input.len() {
            let next_unloaded_index = self.resources.len();

            if let Some(in_progress) = &mut self.in_progress {
                // the loading of some texture was started

                match resume(in_progress.as_mut()) {
                    Some(res) => {
                        // the texture finished loading
                        let resource = res?;
                        self.resources.push(resource);
                        self.in_progress = None;
                    }
                    None => {
                        // the texture is still loading
                    }
                }
            } else {
                // no texture is loading
                let resource_fut = (self.load_func)(self.resources_input[next_unloaded_index]);
                self.in_progress = Some(Box::pin(resource_fut));
            }
            Ok(None)
        } else {
            // finished loading textures
            let mut resources = Vec::new();
            std::mem::swap(&mut resources, &mut self.resources);
            Ok(Some(resources))
        }
    }
}

// resume() and waker() taken from macroquad::exec. I don't understand why they are private
// I only made them generic over Fut

/// returns Some(T) if future is done, None if it would block
fn resume<Fut>(mut future: Pin<&mut Fut>) -> Option<Fut::Output>
where
    Fut: Future,
{
    let waker = waker();
    let mut futures_context = std::task::Context::from_waker(&waker);
    match Future::poll(future.as_mut(), &mut futures_context) {
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
