#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_ptr_alignment,
    clippy::cast_sign_loss,
    clippy::must_use_candidate,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::wildcard_imports
)]

mod event;
mod platform;

mod sdl2;

mod web;

pub mod key;

pub use event::Event;
pub use platform::{
    delay, display_size, gl_context, gl_profile, gl_swap_buffers, init, init_audio, init_window,
    is_fullscreen, pause_audio, poll_events, quit, set_fullscreen, set_mouse_pos,
    set_mouse_visible, set_window_icon, set_window_pos, set_window_size, set_window_title,
    start_loop, step_loop, ticks, window_pos, window_size, GlProfile,
};
