use std::env::var;
use std::fs::{read_to_string, write};

use crate::platform;
use crate::settings::{WATCH_STATE_FILE_ENV, WINDOW_STATE_ENV};

pub struct WindowWatcher {
    watch_state_file: Option<String>,
    window_state: Option<(i32, i32, u32, u32)>,
}

impl WindowWatcher {
    // Constructor

    pub fn new() -> Self {
        let (watch_state_file, raw_state) = if let Ok(path) = var(WATCH_STATE_FILE_ENV) {
            let content = read_to_string(&path).unwrap_or_default();
            (Some(path), content)
        } else {
            (None, var(WINDOW_STATE_ENV).unwrap_or_default())
        };

        let restored_state = Self::parse_window_state(&raw_state);

        if let Some((x, y, w, h)) = restored_state {
            platform::set_window_pos(x, y);
            platform::set_window_size(w, h);
        }
        let (x, y) = platform::window_pos();
        let (w, h) = platform::window_size();

        Self {
            watch_state_file,
            window_state: Some((x, y, w, h)),
        }
    }

    pub fn new_headless() -> Self {
        Self {
            watch_state_file: None,
            window_state: None,
        }
    }

    // State capture

    pub fn update(&mut self) {
        if platform::is_fullscreen() {
            return;
        }

        let (x, y) = platform::window_pos();
        let (w, h) = platform::window_size();
        let window_state = Some((x, y, w, h));

        if self.window_state != window_state {
            self.window_state = window_state;

            if let Some(path) = &self.watch_state_file {
                // Best-effort write; watcher continues even if the state file is unavailable.
                write(path, format!("{x} {y} {w} {h}")).ok();
            }
        }
    }

    #[cfg(not(target_os = "emscripten"))]
    pub fn state_string(&self) -> Option<String> {
        self.window_state
            .map(|(x, y, w, h)| format!("{x} {y} {w} {h}"))
    }

    // Parsing

    fn parse_window_state(raw_state: &str) -> Option<(i32, i32, u32, u32)> {
        let mut fields = raw_state.split_whitespace();
        let x = fields.next()?.parse().ok()?;
        let y = fields.next()?.parse().ok()?;
        let w = fields.next()?.parse().ok()?;
        let h = fields.next()?.parse().ok()?;
        Some((x, y, w, h))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_window_state() {
        assert_eq!(
            WindowWatcher::parse_window_state("10 20 320 240"),
            Some((10, 20, 320, 240))
        );
        // Negative positions are valid (window partly off-screen)
        assert_eq!(
            WindowWatcher::parse_window_state("-5 -10 320 240"),
            Some((-5, -10, 320, 240))
        );
        // Any whitespace separates fields
        assert_eq!(
            WindowWatcher::parse_window_state(" 10\t20  320 240 "),
            Some((10, 20, 320, 240))
        );
        // Trailing extra fields are ignored
        assert_eq!(
            WindowWatcher::parse_window_state("1 2 3 4 5"),
            Some((1, 2, 3, 4))
        );
    }

    #[test]
    fn test_parse_window_state_invalid_input_falls_back() {
        // Malformed state strings yield None so the default placement is used
        assert_eq!(WindowWatcher::parse_window_state(""), None);
        assert_eq!(WindowWatcher::parse_window_state("10 20 320"), None);
        assert_eq!(WindowWatcher::parse_window_state("10 20 abc 240"), None);
        // Sizes are unsigned; a negative size is rejected
        assert_eq!(WindowWatcher::parse_window_state("10 20 -320 240"), None);
    }

    #[cfg(not(target_os = "emscripten"))]
    #[test]
    fn test_state_string() {
        let watcher = WindowWatcher {
            watch_state_file: None,
            window_state: Some((-5, 10, 320, 240)),
        };
        assert_eq!(watcher.state_string().as_deref(), Some("-5 10 320 240"));
        assert_eq!(WindowWatcher::new_headless().state_string(), None);
    }
}
