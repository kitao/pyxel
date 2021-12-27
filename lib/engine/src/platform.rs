use crate::event::Event;
use crate::types::{Color, Rgb8};

pub trait AudioCallback {
    fn update(&mut self, out: &mut [i16]);
}

pub trait Platform {
    fn new(title: &str, width: u32, height: u32, display_ratio: f64) -> Self;
    fn set_title(&mut self, title: &str);
    fn set_icon(&mut self, image: &[Vec<Color>], colors: &[Rgb8], scale: u32);
    fn show_cursor(&self, show: bool);
    fn move_cursor(&self, x: i32, y: i32);
    fn toggle_fullscreen(&mut self);
    fn tick_count(&self) -> u32;
    fn sleep(&mut self, ms: u32);
    fn poll_event(&mut self) -> Option<Event>;
    fn render_screen(&mut self, image: &[Vec<Color>], colors: &[Rgb8], bg_color: Rgb8);
    fn start_audio(
        &mut self,
        sample_rate: u32,
        num_samples: u32,
        audio_callback: shared_type!(dyn AudioCallback + Send),
    );
    fn pause_audio(&mut self);
    fn resume_audio(&mut self);
}
