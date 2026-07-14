use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, Ordering};

use glow::Context;

use super::event::Event;
#[cfg(any(feature = "sdl2_dynamic", feature = "sdl2_static"))]
use super::sdl2::platform_sdl2::PlatformSdl2 as Platform;

#[derive(PartialEq)]
pub enum GlProfile {
    None,
    Gl,
    Gles,
}

thread_local! {
    static PLATFORM: RefCell<Option<Platform>> = const { RefCell::new(None) };
}
static HEADLESS: AtomicBool = AtomicBool::new(false);

fn with_platform<T>(f: impl FnOnce(&Platform) -> T) -> T {
    PLATFORM.with(|platform| {
        let platform = platform.borrow();
        f(platform.as_ref().expect("Platform not initialized"))
    })
}

fn with_platform_mut<T>(f: impl FnOnce(&mut Platform) -> T) -> T {
    PLATFORM.with(|platform| {
        let mut platform = platform.borrow_mut();
        f(platform.as_mut().expect("Platform not initialized"))
    })
}

pub fn is_headless() -> bool {
    HEADLESS.load(Ordering::Relaxed)
}

// SIGINT handling

static SIGINT_RECEIVED: AtomicBool = AtomicBool::new(false);

#[cfg(not(target_os = "emscripten"))]
extern "C" fn sigint_handler(_sig: std::os::raw::c_int) {
    SIGINT_RECEIVED.store(true, Ordering::Relaxed);
}

pub fn is_sigint_received() -> bool {
    SIGINT_RECEIVED.swap(false, Ordering::Relaxed)
}

// Lifecycle

pub fn init(headless: bool) {
    HEADLESS.store(headless, Ordering::Relaxed);

    let mut platform = Platform::new();
    platform.init(headless);
    PLATFORM.with(|current| *current.borrow_mut() = Some(platform));

    #[cfg(not(target_os = "emscripten"))]
    unsafe {
        libc::signal(
            libc::SIGINT,
            sigint_handler as *const () as libc::sighandler_t,
        );
    }
}

pub fn quit() {
    if let Some(mut callback) = crate::quit_callback().take() {
        callback();
    }
    with_platform_mut(Platform::quit);
}

pub fn ticks() -> u32 {
    with_platform(Platform::ticks)
}

pub fn export_browser_file(filename: &str) {
    if !is_headless() {
        with_platform(|platform| platform.export_browser_file(filename));
    }
}

// Window

pub fn init_window(title: &str, width: u32, height: u32) {
    if !is_headless() {
        with_platform_mut(|platform| platform.init_window(title, width, height));
    }
}

pub fn window_pos() -> (i32, i32) {
    if is_headless() {
        return (0, 0);
    }
    with_platform(Platform::window_pos)
}

pub fn set_window_pos(x: i32, y: i32) {
    if !is_headless() {
        with_platform_mut(|platform| platform.set_window_pos(x, y));
    }
}

pub fn window_size() -> (u32, u32) {
    if is_headless() {
        return (0, 0);
    }
    with_platform(Platform::window_size)
}

pub fn set_window_size(width: u32, height: u32) {
    if !is_headless() {
        with_platform_mut(|platform| platform.set_window_size(width, height));
    }
}

pub fn set_window_title(title: &str) {
    if !is_headless() {
        with_platform_mut(|platform| platform.set_window_title(title));
    }
}

pub fn set_window_icon(width: u32, height: u32, rgba: &[u8]) {
    if !is_headless() {
        with_platform_mut(|platform| platform.set_window_icon(width, height, rgba));
    }
}

pub fn is_fullscreen() -> bool {
    if is_headless() {
        return false;
    }
    with_platform(Platform::is_fullscreen)
}

pub fn set_fullscreen(enabled: bool) {
    if !is_headless() {
        with_platform_mut(|platform| platform.set_fullscreen(enabled));
    }
}

pub fn set_mouse_pos(x: i32, y: i32) {
    if !is_headless() {
        with_platform_mut(|platform| platform.set_mouse_pos(x, y));
    }
}

pub fn set_mouse_visible(visible: bool) {
    if !is_headless() {
        with_platform_mut(|platform| platform.set_mouse_visible(visible));
    }
}

pub fn display_size() -> (u32, u32) {
    if is_headless() {
        return (0, 0);
    }
    with_platform(Platform::display_size)
}

// Audio

pub fn start_audio<F: FnMut(&mut [i16]) + 'static>(
    sample_rate: u32,
    buffer_size: u32,
    callback: F,
) {
    with_platform_mut(|platform| platform.start_audio(sample_rate, buffer_size, callback));
}

pub fn pause_audio(paused: bool) {
    with_platform_mut(|platform| platform.pause_audio(paused));
}

#[cfg(not(target_os = "emscripten"))]
pub fn close_audio() {
    with_platform_mut(Platform::close_audio);
}

pub fn lock_audio() {
    with_platform(Platform::lock_audio);
}

pub fn unlock_audio() {
    with_platform(Platform::unlock_audio);
}

// Frame

pub fn run_frame_loop<F: FnMut(f32)>(fps: u32, callback: F) {
    Platform::run_frame_loop(fps, callback);
}

pub fn step_frame(fps: u32) {
    with_platform_mut(|platform| platform.step_frame(fps));
}

#[cfg(not(target_os = "emscripten"))]
pub(super) fn swap_window() {
    with_platform(Platform::swap_window);
}

pub fn poll_events(events: &mut Vec<Event>) {
    events.clear();
    if !is_headless() {
        with_platform_mut(|platform| platform.poll_events(events));
    }
}

// OpenGL

pub fn gl_profile() -> GlProfile {
    if is_headless() {
        return GlProfile::None;
    }
    with_platform(Platform::gl_profile)
}

pub fn with_gl_context<T>(f: impl FnOnce(&mut Context) -> T) -> T {
    assert!(
        !is_headless(),
        "GL context is not available in headless mode"
    );
    with_platform_mut(|platform| platform.with_gl_context(f))
}
