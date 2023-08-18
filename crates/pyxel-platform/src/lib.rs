mod event;
mod platform;
mod sdl2_sys;
mod window;

pub mod keys;

pub use crate::event::{poll_events, Event};
pub use crate::platform::{
    display_size, elapsed_time, init, quit, run, set_audio_enabled, set_mouse_pos,
    set_mouse_visible, sleep,
};
pub use crate::window::{
    create_window, gl, is_fullscreen, set_fullscreen, set_window_icon, set_window_title,
};
