#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_ptr_alignment,
    clippy::cast_sign_loss,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::too_many_lines,
    clippy::unreadable_literal,
    clippy::wildcard_imports
)]

mod audio;
#[cfg(target_os = "emscripten")]
pub mod emscripten;
mod event;
mod gamepad;
mod keyboard;
pub mod keys;
mod mouse;
mod platform;
mod sdl2_sys;
mod window;

pub use crate::audio::{set_audio_enabled, start_audio, AudioCallback};
pub use crate::event::{poll_events, Event};
pub use crate::platform::{elapsed_time, init, quit, run, sleep};
pub use crate::window::{
    glow_context, is_fullscreen, is_gles_enabled, set_fullscreen, set_mouse_pos, set_mouse_visible,
    set_window_icon, set_window_pos, set_window_size, set_window_title, swap_window, window_pos,
    window_size,
};
