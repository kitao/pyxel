use std::ptr::null_mut;
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

static mut PLATFORM: *mut Platform = null_mut();
static HEADLESS: AtomicBool = AtomicBool::new(false);

fn platform() -> &'static mut Platform {
    unsafe { &mut *PLATFORM }
}

fn is_headless() -> bool {
    HEADLESS.load(Ordering::Relaxed)
}

// SIGINT

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

    // Drop the previous platform if init() is called again
    unsafe {
        if !PLATFORM.is_null() {
            drop(Box::from_raw(PLATFORM));
            PLATFORM = null_mut();
        }
    }

    let mut platform = Platform::new();
    platform.init(headless);
    unsafe {
        PLATFORM = Box::into_raw(Box::new(platform));
    }

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
    platform().quit();
}

pub fn ticks() -> u32 {
    platform().ticks()
}

pub fn export_browser_file(filename: &str) {
    if !is_headless() {
        platform().export_browser_file(filename);
    }
}

// Window

pub fn init_window(title: &str, width: u32, height: u32) {
    if !is_headless() {
        platform().init_window(title, width, height);
    }
}

pub fn window_pos() -> (i32, i32) {
    if is_headless() {
        return (0, 0);
    }
    platform().window_pos()
}

pub fn set_window_pos(x: i32, y: i32) {
    if !is_headless() {
        platform().set_window_pos(x, y);
    }
}

pub fn window_size() -> (u32, u32) {
    if is_headless() {
        return (0, 0);
    }
    platform().window_size()
}

pub fn set_window_size(width: u32, height: u32) {
    if !is_headless() {
        platform().set_window_size(width, height);
    }
}

pub fn set_window_title(title: &str) {
    if !is_headless() {
        platform().set_window_title(title);
    }
}

pub fn set_window_icon(width: u32, height: u32, rgba: &[u8]) {
    if !is_headless() {
        platform().set_window_icon(width, height, rgba);
    }
}

pub fn is_fullscreen() -> bool {
    if is_headless() {
        return false;
    }
    platform().is_fullscreen()
}

pub fn set_fullscreen(enabled: bool) {
    if !is_headless() {
        platform().set_fullscreen(enabled);
    }
}

pub fn set_mouse_pos(x: i32, y: i32) {
    if !is_headless() {
        platform().set_mouse_pos(x, y);
    }
}

pub fn set_mouse_visible(visible: bool) {
    if !is_headless() {
        platform().set_mouse_visible(visible);
    }
}

pub fn display_size() -> (u32, u32) {
    if is_headless() {
        return (0, 0);
    }
    platform().display_size()
}

// Audio

pub fn start_audio<F: FnMut(&mut [i16]) + 'static>(
    sample_rate: u32,
    buffer_size: u32,
    callback: F,
) {
    platform().start_audio(sample_rate, buffer_size, callback);
}

pub fn pause_audio(paused: bool) {
    platform().pause_audio(paused);
}

pub fn lock_audio() {
    platform().lock_audio();
}

pub fn unlock_audio() {
    platform().unlock_audio();
}

// Frame

pub fn run_frame_loop<F: FnMut(f32)>(fps: u32, callback: F) {
    platform().run_frame_loop(fps, callback);
}

pub fn step_frame(fps: u32) {
    platform().step_frame(fps);
}

pub fn poll_events(events: &mut Vec<Event>) {
    events.clear();
    if !is_headless() {
        platform().poll_events(events);
    }
}

// OpenGL

pub fn gl_profile() -> GlProfile {
    if is_headless() {
        return GlProfile::None;
    }
    platform().gl_profile()
}

pub fn gl_context() -> &'static mut Context {
    assert!(
        !is_headless(),
        "GL context is not available in headless mode"
    );
    platform().gl_context()
}
