use glow::Context;

use crate::event::Event;
use crate::sdl2::platform_sdl2::PlatformSdl2;

pub enum GlProfile {
    None,
    GL,
    GLES,
}

pub trait Platform {
    // Core
    fn init(&mut self);
    fn quit(&mut self);
    fn ticks(&mut self) -> u32;
    fn delay(&mut self, ms: u32);

    // Window
    fn init_window(&mut self, title: &str, width: u32, height: u32);
    fn window_pos(&mut self) -> (i32, i32);
    fn set_window_pos(&mut self, x: i32, y: i32);
    fn window_size(&mut self) -> (u32, u32);
    fn set_window_size(&mut self, width: u32, height: u32);
    fn set_window_title(&mut self, title: &str);
    fn set_window_icon(&mut self, width: u32, height: u32, pixels: &[u32]);
    fn is_fullscreen(&mut self) -> bool;
    fn set_fullscreen(&mut self, enabled: bool);
    fn set_mouse_pos(&mut self, x: i32, y: i32);
    fn set_mouse_visible(&mut self, visible: bool);
    fn display_size(&mut self) -> (u32, u32);

    // Audio
    fn init_audio(
        &mut self,
        sample_rate: u32,
        buffer_size: u32,
        callback: Box<dyn FnMut(&mut [i16]) + Send>,
    );
    fn pause_audio(&mut self, paused: bool);

    // Frame
    fn start_loop(&mut self, callback: Box<dyn FnMut() + Send>);
    fn step_loop(&mut self);
    fn poll_events(&mut self) -> Vec<Event>;
    fn gl_profile(&mut self) -> GlProfile;
    fn gl_context(&mut self) -> Context;
}

static mut PLATFORM: Option<Box<dyn Platform>> = None;

fn platform() -> &'static mut dyn Platform {
    unsafe { PLATFORM.as_mut().unwrap().as_mut() }
}

//
// Core
//
pub fn init() {
    let mut platform = PlatformSdl2::new();

    platform.init();

    unsafe {
        PLATFORM = Some(Box::new(platform));
    }
}

pub fn quit() {
    platform().quit()
}

pub fn ticks() -> u32 {
    platform().ticks()
}

pub fn delay(ms: u32) {
    platform().delay(ms);
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

pub fn set_window_icon(width: u32, height: u32, pixels: &[u32]) {
    platform().set_window_icon(width, height, pixels);
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

//
// Audio
//
pub fn init_audio(sample_rate: u32, buffer_size: u32, callback: Box<dyn FnMut(&mut [i16]) + Send>) {
    platform().init_audio(sample_rate, buffer_size, callback);
}

pub fn pause_audio(paused: bool) {
    platform().pause_audio(paused);
}

//
// Frame
//
pub fn start_loop(callback: Box<dyn FnMut() + Send>) {
    platform().start_loop(callback);
}

pub fn step_loop() {
    platform().step_loop();
}

pub fn poll_events() -> Vec<Event> {
    platform().poll_events()
}

pub fn gl_profile() -> GlProfile {
    platform().gl_profile()
}

pub fn gl_context() -> Context {
    platform().gl_context()
}
