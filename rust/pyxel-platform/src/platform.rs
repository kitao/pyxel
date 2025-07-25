use std::sync::Arc;

use parking_lot::Mutex;

use crate::event::Event;

pub trait LoopCallback {
    fn update(&mut self);
}

pub trait AudioCallback {
    fn update(&mut self, out: &mut [i16]);
}

pub(crate) trait Platform {
    // Core
    fn init(&mut self);
    fn quit(&mut self);

    // Window
    fn init_window(&mut self, title: &str, width: u32, height: u32);
    fn window_pos(&self) -> (i32, i32);
    fn set_window_pos(&mut self, x: i32, y: i32);
    fn window_size(&self) -> (u32, u32);
    fn set_window_size(&mut self, width: u32, height: u32);
    fn set_window_title(&mut self, title: &str);
    fn set_window_icon(&mut self, pixels: &[u32]);
    fn is_fullscreen(&self) -> bool;
    fn set_fullscreen(&mut self, enabled: bool);
    fn set_mouse_pos(&mut self, x: i32, y: i32);
    fn set_mouse_visible(&mut self, visible: bool);

    // Audio
    fn init_audio(&mut self, sample_rate: u32, buffer_size: u32);
    fn start_audio(&mut self, callback: Arc<Mutex<dyn AudioCallback>>);
    fn pause_audio(&mut self, paused: bool);

    // Frame
    fn start_loop(&mut self, callback: Arc<Mutex<dyn LoopCallback>>);
    fn step_loop(&mut self);
    fn poll_events(&mut self) -> Vec<Event>;
    fn gl_context(&self) -> Option<glow::Context>;
    fn update_screen(&mut self, pixels: &[u32]);
    fn elapsed_time(&self) -> u32;
}

static mut PLATFORM: Option<*mut dyn Platform> = None;

fn platform() -> &'static mut dyn Platform {
    unsafe { &mut *(PLATFORM.unwrap()) }
}

pub fn init() {
    // TODO
}

pub fn quit() {
    platform().quit()
}

pub fn set_window_size(width: u32, height: u32) {
    platform().set_window_size(width, height);
}
