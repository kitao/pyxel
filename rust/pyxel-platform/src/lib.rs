mod event;
mod platform;

mod sdl2;

mod web;

pub mod key;

pub use event::Event;
pub use platform::{
    delay, gl_context, gl_profile, init, init_audio, init_window, is_fullscreen, pause_audio,
    poll_events, quit, set_fullscreen, set_mouse_pos, set_mouse_visible, set_window_icon,
    set_window_pos, set_window_size, set_window_title, start_loop, step_loop, ticks, window_pos,
    window_size, GlProfile,
};
