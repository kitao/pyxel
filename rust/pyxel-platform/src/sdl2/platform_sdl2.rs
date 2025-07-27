use std::ptr::null_mut;

use glow::Context;

use crate::event::Event;
use crate::platform::{GlProfile, Platform};
use crate::sdl2::gamepad::Gamepad;
use crate::sdl2::sdl2_sys::*;

pub(crate) struct PlatformSdl2 {
    pub window: *mut SDL_Window,
    pub gl_context: *mut Context,
    pub audio_device_id: SDL_AudioDeviceID,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub gamepads: Vec<Gamepad>,
}

impl PlatformSdl2 {
    pub fn new() -> Self {
        Self {
            window: null_mut(),
            gl_context: null_mut(),
            audio_device_id: 0,
            mouse_x: i32::MIN,
            mouse_y: i32::MIN,
            gamepads: Vec::new(),
        }
    }
}

impl Platform for PlatformSdl2 {
    // Core
    fn init(&mut self) {
        self.init();
    }
    fn quit(&mut self) {
        self.quit();
    }
    fn ticks(&mut self) -> u32 {
        self.ticks()
    }
    fn delay(&mut self, ms: u32) {
        self.delay(ms);
    }

    // Window
    fn init_window(&mut self, title: &str, width: u32, height: u32) {
        self.init_window(title, width, height);
    }
    fn window_pos(&mut self) -> (i32, i32) {
        self.window_pos()
    }
    fn set_window_pos(&mut self, x: i32, y: i32) {
        self.set_window_pos(x, y);
    }
    fn window_size(&mut self) -> (u32, u32) {
        self.window_size()
    }
    fn set_window_size(&mut self, width: u32, height: u32) {
        self.set_window_size(width, height);
    }
    fn set_window_title(&mut self, title: &str) {
        self.set_window_title(title);
    }
    fn set_window_icon(&mut self, width: u32, height: u32, pixels: &[u32]) {
        self.set_window_icon(width, height, pixels);
    }
    fn is_fullscreen(&mut self) -> bool {
        self.is_fullscreen()
    }
    fn set_fullscreen(&mut self, enabled: bool) {
        self.set_fullscreen(enabled);
    }
    fn set_mouse_pos(&mut self, x: i32, y: i32) {
        self.set_mouse_pos(x, y);
    }
    fn set_mouse_visible(&mut self, visible: bool) {
        self.set_mouse_visible(visible);
    }
    fn display_size(&mut self) -> (u32, u32) {
        self.display_size()
    }

    // Audio
    fn init_audio(
        &mut self,
        sample_rate: u32,
        buffer_size: u32,
        callback: Box<dyn FnMut(&mut [i16]) + Send>,
    ) {
        self.init_audio(sample_rate, buffer_size, callback);
    }
    fn pause_audio(&mut self, paused: bool) {
        self.pause_audio(paused);
    }

    // Frame
    fn start_loop(&mut self, callback: Box<dyn FnMut() + Send>) {
        self.start_loop(callback);
    }
    fn step_loop(&mut self) {
        self.step_loop();
    }
    fn poll_events(&mut self) -> Vec<Event> {
        self.poll_events()
    }
    fn gl_profile(&mut self) -> GlProfile {
        self.gl_profile()
    }
    fn gl_context(&mut self) -> Context {
        self.gl_context()
    }
}
