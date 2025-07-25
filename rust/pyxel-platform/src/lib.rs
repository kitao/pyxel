mod event;
mod platform;

mod sdl2;

mod web;

pub mod key;

pub use event::Event;
pub use platform::{init, quit, set_window_size, AudioCallback};
