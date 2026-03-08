use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, Ordering};

use glow::Context;

use super::event::Event;
#[cfg(any(feature = "sdl2_dynamic", feature = "sdl2_static"))]
use super::sdl2::platform_sdl2::PlatformSdl2 as Platform;

#[derive(PartialEq)]
pub enum GLProfile {
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

//
// Core
//
pub fn init(headless: bool) {
    HEADLESS.store(headless, Ordering::Relaxed);

    let mut platform = Platform::new();
    platform.init(headless);

    unsafe {
        PLATFORM = Box::into_raw(Box::new(platform));
    }
}

pub fn quit() {
    if !is_headless() {
        platform().quit();
    }
    std::process::exit(0);
}

pub fn ticks() -> u32 {
    if is_headless() {
        return 0;
    }
    platform().ticks()
}

pub fn export_browser_file(filename: &str) {
    if !is_headless() {
        platform().export_browser_file(filename);
    }
}

//
// Window
//
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

//
// Audio
//
pub fn start_audio<F: FnMut(&mut [i16]) + 'static>(
    sample_rate: u32,
    buffer_size: u32,
    callback: F,
) {
    if !is_headless() {
        platform().start_audio(sample_rate, buffer_size, callback);
    }
}

pub fn pause_audio(paused: bool) {
    if !is_headless() {
        platform().pause_audio(paused);
    }
}

pub fn lock_audio() {
    if !is_headless() {
        platform().lock_audio();
    }
}

pub fn unlock_audio() {
    if !is_headless() {
        platform().unlock_audio();
    }
}

//
// Frame
//
pub fn run_frame_loop<F: FnMut(f32)>(fps: u32, callback: F) {
    if !is_headless() {
        platform().run_frame_loop(fps, callback);
    }
}

pub fn step_frame(fps: u32) {
    if !is_headless() {
        platform().step_frame(fps);
    }
}

pub fn poll_events() -> Vec<Event> {
    if is_headless() {
        return Vec::new();
    }
    platform().poll_events()
}

pub fn gl_profile() -> GLProfile {
    if is_headless() {
        return GLProfile::None;
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
