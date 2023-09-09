mod audio;
mod controller;
mod event;
pub mod keys;
mod platform;
mod sdl2_sys;
mod window;

pub use crate::audio::set_audio_enabled;
pub use crate::event::{poll_events, Event};
pub use crate::platform::{elapsed_time, init, quit, run, sleep};
pub use crate::window::{
    display_size, glow_context, is_fullscreen, set_fullscreen, set_mouse_pos, set_mouse_visible,
    set_window_icon, set_window_pos, set_window_size, set_window_title, swap_window, window_pos,
    window_size,
};
