use crate::event::Event;
use crate::image::Image;
use crate::palette::Rgb24;

pub trait Platform {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn caption(&mut self) -> &str;
    fn set_caption(&mut self, caption: &str);
    fn set_icon(&mut self, icon: &Image, scale: u32);
    fn is_full_screen(&self) -> bool;
    fn set_full_screen(&mut self, is_full_screen: bool);
    fn ticks(&self) -> u32;
    fn delay(&self, ms: u32);
    fn poll_event(&mut self) -> Option<Event>;
    fn render_screen(&mut self, screen: &Image, bg_color: Rgb24);
}
