use crate::event::Event;
use crate::image::Image;
use crate::palette::Rgb24;

pub trait Platform {
    fn window_pos(&self) -> (i32, i32);
    fn window_size(&self) -> (u32, u32);
    fn window_title(&self) -> &str;
    fn set_window_title(&mut self, title: &str);
    fn set_window_icon(&mut self, icon: &Image, scale: u32);
    fn is_fullscreen(&self) -> bool;
    fn set_fullscreen(&mut self, is_full_screen: bool);
    fn ticks(&self) -> u32;
    fn delay(&mut self, ms: u32);
    fn poll_event(&mut self) -> Option<Event>;
    fn render_screen(&mut self, screen: &Image, bg_color: Rgb24);
}
