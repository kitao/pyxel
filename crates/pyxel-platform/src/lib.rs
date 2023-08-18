mod event;
mod platform;
mod sdl2_sys;

pub mod keys;

pub use crate::event::Event;
pub use crate::platform::{
    elapsed_time, gl, init, is_fullscreen, poll_events, quit, run, set_audio_enabled,
    set_fullscreen, set_mouse_pos, set_mouse_visible, set_window_icon, set_window_title, sleep,
};
