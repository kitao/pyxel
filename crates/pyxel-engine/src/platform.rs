use std::sync::Arc;

use parking_lot::Mutex;

use crate::event::Event;
use crate::image::Image;
use crate::types::Rgb8;

pub trait AudioCallback {
    fn update(&mut self, out: &mut [i16]);
}

pub trait Platform {
    fn new(title: &str, width: u32, height: u32, scale: u32) -> Self;
    fn set_title(&mut self, title: &str);
    fn set_icon(&mut self, icon: &Image, colors: &[Rgb8], scale: u32);
    fn toggle_fullscreen(&mut self);
    fn tick_count(&self) -> u32;
    fn sleep(&mut self, ms: u32);
    fn poll_event(&mut self) -> Option<Event>;
    fn render_screen(&mut self, screen: &Image, colors: &[Rgb8], bg_color: Rgb8);
    fn start_audio(
        &mut self,
        sample_rate: u32,
        sample_count: u32,
        audio_callback: Arc<Mutex<dyn AudioCallback + Send>>,
    );
}
