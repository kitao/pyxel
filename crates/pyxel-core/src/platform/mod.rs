pub mod key;

mod event;
mod facade;
#[cfg(any(feature = "sdl2_system", feature = "sdl2_bundle"))]
mod sdl2;

pub use event::Event;
pub use facade::{
    display_size, export_browser_file, gl_context, gl_profile, init, init_window, is_fullscreen,
    pause_audio, poll_events, quit, run_frame_loop, set_fullscreen, set_mouse_pos,
    set_mouse_visible, set_window_icon, set_window_pos, set_window_size, set_window_title,
    start_audio, step_frame, ticks, window_pos, window_size, GLProfile,
};
