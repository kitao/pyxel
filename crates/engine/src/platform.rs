use parking_lot::Mutex;
use std::sync::Arc;

use crate::event::Event;
use crate::image::SharedImage;
use crate::types::Rgb8;

pub trait AudioCallback {
    fn update(&mut self, out: &mut [i16]);
}

pub trait Platform {
    fn new(title: &str, width: u32, height: u32, display_ratio: f64) -> Self;
    fn set_title(&mut self, title: &str);
    fn set_icon(&mut self, icon: SharedImage, colors: &[Rgb8], scale: u32);
    fn show_cursor(&self, show: bool);
    fn toggle_fullscreen(&mut self);
    fn tick_count(&self) -> u32;
    fn sleep(&mut self, ms: u32);
    fn poll_event(&mut self) -> Option<Event>;
    fn render_screen(&mut self, screen: SharedImage, colors: &[Rgb8], bg_color: Rgb8);
    fn start_audio(
        &mut self,
        sample_rate: u32,
        sample_count: u32,
        audio_callback: Arc<Mutex<dyn AudioCallback + Send>>,
    );
}
