use std::mem::transmute;
use std::ptr::null_mut;

use glow::Context;

use crate::event::Event;
#[cfg(feature = "sdl2")]
use crate::sdl2::platform_sdl2::PlatformSdl2 as Platform;
#[cfg(feature = "web")]
use crate::web::platform_web::PlatformWeb as Platform;

#[derive(PartialEq)]
pub enum GLProfile {
    None,
    Gl,
    Gles,
}

static mut PLATFORM: *mut Platform = null_mut();

pub fn platform() -> &'static mut Platform {
    unsafe { &mut *PLATFORM }
}

//
// Core
//
pub fn init() {
    let mut platform = Platform::new();

    platform.init();

    unsafe {
        PLATFORM = transmute::<Box<Platform>, *mut Platform>(Box::new(platform));
    }
}

pub fn quit() {
    platform().quit();
}

pub fn ticks() -> u32 {
    platform().ticks()
}

//
// Window
//
pub fn init_window(title: &str, width: u32, height: u32) {
    platform().init_window(title, width, height);
}

pub fn window_pos() -> (i32, i32) {
    platform().window_pos()
}

pub fn set_window_pos(x: i32, y: i32) {
    platform().set_window_pos(x, y);
}

pub fn window_size() -> (u32, u32) {
    platform().window_size()
}

pub fn set_window_size(width: u32, height: u32) {
    platform().set_window_size(width, height);
}

pub fn set_window_title(title: &str) {
    platform().set_window_title(title);
}

pub fn set_window_icon(width: u32, height: u32, rgba: &[u8]) {
    platform().set_window_icon(width, height, rgba);
}

pub fn is_fullscreen() -> bool {
    platform().is_fullscreen()
}

pub fn set_fullscreen(enabled: bool) {
    platform().set_fullscreen(enabled);
}

pub fn set_mouse_pos(x: i32, y: i32) {
    platform().set_mouse_pos(x, y);
}

pub fn set_mouse_visible(visible: bool) {
    platform().set_mouse_visible(visible);
}

pub fn display_size() -> (u32, u32) {
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
    platform().start_audio(sample_rate, buffer_size, callback);
}

pub fn pause_audio(paused: bool) {
    platform().pause_audio(paused);
}

//
// Frame
//
pub fn run_frame_loop<F: FnMut(f32)>(fps: u32, callback: F) {
    platform().run_frame_loop(fps, callback);
}

pub fn step_frame(fps: u32) {
    platform().step_frame(fps);
}

pub fn poll_events() -> Vec<Event> {
    platform().poll_events()
}

pub fn gl_profile() -> GLProfile {
    platform().gl_profile()
}

pub fn gl_context() -> &'static mut Context {
    platform().gl_context()
}
