use std::env::var;
use std::fs::{read_to_string, write};

use crate::settings::{RESET_STATE_ENV, WATCH_STATE_FILE_ENV};

pub struct WindowWatcher {
    watch_state_file: Option<String>,
    window_state: Option<(i32, i32, u32, u32)>,
}

impl WindowWatcher {
    pub fn new() -> Self {
        let (watch_state_file, window_state) = match var(WATCH_STATE_FILE_ENV) {
            Ok(watch_state_file) => (
                Some(watch_state_file.clone()),
                read_to_string(watch_state_file).unwrap_or_default(),
            ),
            Err(_) => (None, var(RESET_STATE_ENV).unwrap_or_default()),
        };

        let window_state = if window_state.is_empty() {
            None
        } else {
            let mut fields = window_state.split_whitespace();
            let x = fields.next().unwrap().parse().unwrap();
            let y = fields.next().unwrap().parse().unwrap();
            let w = fields.next().unwrap().parse().unwrap();
            let h = fields.next().unwrap().parse().unwrap();

            pyxel_platform::set_window_pos(x, y);
            pyxel_platform::set_window_size(w, h);

            Some((x, y, w, h))
        };

        Self {
            watch_state_file,
            window_state,
        }
    }

    pub fn update(&mut self) {
        if self.watch_state_file.is_none() || pyxel_platform::is_fullscreen() {
            return;
        }

        let (x, y) = pyxel_platform::window_pos();
        let (w, h) = pyxel_platform::window_size();
        if self.window_state != Some((x, y, w, h)) {
            let window_state = self.window_state();
            let watch_state_file = self.watch_state_file.as_ref().unwrap();
            write(watch_state_file, window_state).unwrap();
        }
    }

    pub fn window_state(&mut self) -> String {
        if !pyxel_platform::is_fullscreen() {
            let (x, y) = pyxel_platform::window_pos();
            let (w, h) = pyxel_platform::window_size();
            self.window_state = Some((x, y, w, h));
        }

        if let Some((x, y, w, h)) = self.window_state {
            format!("{x} {y} {w} {h}")
        } else {
            String::new()
        }
    }
}
