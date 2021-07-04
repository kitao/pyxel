use std::sync::{Arc, Mutex};

use crate::event::Event;
use crate::image::Image;
use crate::palette::Rgb24;

pub trait AudioCallback {
    fn audio_callback(&mut self, out: &mut [i16]);
}

pub trait Platform {
    fn new(title: &str, width: u32, height: u32, scale: u32) -> Self;
    fn window_pos(&self) -> (i32, i32);
    fn window_size(&self) -> (u32, u32);
    fn set_window_title(&mut self, title: &str);
    fn set_window_icon(&mut self, icon: &Image, scale: u32);
    fn toggle_fullscreen(&mut self);
    fn ticks(&self) -> u32;
    fn delay(&mut self, ms: u32);
    fn poll_event(&mut self) -> Option<Event>;
    fn render_screen(&mut self, screen: &Image, bg_color: Rgb24);
    fn start_audio(
        &mut self,
        sample_rate: u32,
        sample_count: u32,
        audio_callback: Arc<Mutex<dyn AudioCallback + Send>>,
    );
}
