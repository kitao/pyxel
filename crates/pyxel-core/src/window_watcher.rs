use std::env::{set_var, var};
use std::fs::{read_to_string, write};

use crate::settings::{WATCH_STATE_FILE_ENV, WINDOW_STATE_ENV};

pub struct WindowWatcher {
    watch_state_file: Option<String>,
    window_state: Option<(i32, i32, u32, u32)>,
}

impl WindowWatcher {
    pub fn new() -> Self {
        let (watch_state_file, state_str) = if let Ok(watch_state_file) = var(WATCH_STATE_FILE_ENV)
        {
            (
                Some(watch_state_file.clone()),
                read_to_string(watch_state_file).unwrap_or_default(),
            )
        } else {
            (None, var(WINDOW_STATE_ENV).unwrap_or_default())
        };

        let window_state = Self::parse_window_state(&state_str);

        if let Some((x, y, w, h)) = window_state {
            crate::platform::set_window_pos(x, y);
            crate::platform::set_window_size(w, h);
            set_var(WINDOW_STATE_ENV, &state_str);
        }

        Self {
            watch_state_file,
            window_state,
        }
    }

    pub fn update(&mut self) {
        if crate::platform::is_fullscreen() {
            return;
        }

        let (x, y) = crate::platform::window_pos();
        let (w, h) = crate::platform::window_size();
        let window_state = Some((x, y, w, h));

        if self.window_state != window_state {
            self.window_state = window_state;

            let state_str = Self::format_window_state(window_state);
            set_var(WINDOW_STATE_ENV, &state_str);
            if let Some(watch_state_file) = &self.watch_state_file {
                write(watch_state_file, &state_str).unwrap();
            }
        }
    }

    fn parse_window_state(state_str: &str) -> Option<(i32, i32, u32, u32)> {
        let mut fields = state_str.split_whitespace();
        let x = fields.next()?.parse().ok()?;
        let y = fields.next()?.parse().ok()?;
        let w = fields.next()?.parse().ok()?;
        let h = fields.next()?.parse().ok()?;
        Some((x, y, w, h))
    }

    fn format_window_state(window_state: Option<(i32, i32, u32, u32)>) -> String {
        if let Some((x, y, w, h)) = window_state {
            format!("{x} {y} {w} {h}")
        } else {
            String::new()
        }
    }
}
