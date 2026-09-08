use std::collections::HashMap;

use crate::key::{
    Key, KeyValue, GAMEPAD_AXIS_COUNT, GAMEPAD_KEY_START_INDEX, GAMEPAD_KEY_STRIDE,
    MOUSE_KEY_START_INDEX, MOUSE_POS_X, MOUSE_POS_Y, MOUSE_WHEEL_X, MOUSE_WHEEL_Y,
};
use crate::platform;
use crate::pyxel::{self, Pyxel};
use crate::utils::f32_to_i32;

#[derive(Clone, Copy, PartialEq)]
enum KeyState {
    Pressed,
    Released,
    PressedAndReleased,
    ReleasedAndPressed,
}

pub struct Input {
    mouse_visible: bool,
    key_states: HashMap<Key, (u32, KeyState)>,
    key_values: HashMap<Key, KeyValue>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            mouse_visible: false,
            key_states: HashMap::new(),
            key_values: HashMap::new(),
        }
    }
}

impl Pyxel {
    // Query API

    pub fn is_button_down(&self, key: Key) -> bool {
        assert!(!Self::is_analog_key(key), "key must be a non-analog key");
        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            match key_state {
                KeyState::Pressed | KeyState::ReleasedAndPressed => true,
                KeyState::PressedAndReleased => Self::is_current_frame(*frame_count),
                KeyState::Released => false,
            }
        } else {
            false
        }
    }

    pub fn is_button_pressed(
        &self,
        key: Key,
        hold_frames: Option<u32>,
        repeat_frames: Option<u32>,
    ) -> bool {
        assert!(!Self::is_analog_key(key), "key must be a non-analog key");
        let Some((frame_count, key_state)) = self.input.key_states.get(&key) else {
            return false;
        };
        if *key_state == KeyState::Released {
            return false;
        }
        if Self::is_current_frame(*frame_count) {
            return true;
        }
        if *key_state == KeyState::PressedAndReleased {
            return false;
        }

        // Key repeat logic
        let repeat = repeat_frames.unwrap_or(0);
        if repeat == 0 {
            return false;
        }
        let hold = hold_frames.unwrap_or(0);
        let elapsed = pyxel::frame_count().wrapping_sub(*frame_count);
        elapsed >= hold && (elapsed - hold).is_multiple_of(repeat)
    }

    pub fn is_button_released(&self, key: Key) -> bool {
        assert!(!Self::is_analog_key(key), "key must be a non-analog key");
        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            match key_state {
                KeyState::Pressed => false,
                _ => Self::is_current_frame(*frame_count),
            }
        } else {
            false
        }
    }

    pub fn button_value(&self, key: Key) -> KeyValue {
        assert!(Self::is_analog_key(key), "key must be an analog key");
        self.input.key_values.get(&key).copied().unwrap_or(0)
    }

    // Setter API

    pub fn set_mouse_visible(&mut self, visible: bool) {
        self.input.mouse_visible = visible;
    }

    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        let x = f32_to_i32(x);
        let y = f32_to_i32(y);
        *pyxel::mouse_x() = x;
        *pyxel::mouse_y() = y;
        self.input.key_values.insert(MOUSE_POS_X, x);
        self.input.key_values.insert(MOUSE_POS_Y, y);

        if !*pyxel::is_headless() {
            platform::set_mouse_pos(
                screen_to_window(x, self.system.screen_scale, self.system.screen_x),
                screen_to_window(y, self.system.screen_scale, self.system.screen_y),
            );
        }
    }

    pub fn set_button_state(&mut self, key: Key, state: bool) {
        if state {
            self.press_key(key);
        } else {
            self.release_key(key);
        }
    }

    pub fn set_button_value(&mut self, key: Key, value: KeyValue) {
        self.set_key_value(key, value);
    }

    pub fn set_input_text(&mut self, text: &str) {
        pyxel::input_text().clear();
        self.add_input_text(text);
    }

    pub fn set_dropped_files<S: AsRef<str>>(&mut self, files: &[S]) {
        pyxel::dropped_files().clear();
        for file in files {
            self.add_dropped_file(file.as_ref());
        }
    }

    // Internal API

    pub(crate) fn start_input_frame(&mut self) {
        self.input.key_values.insert(MOUSE_WHEEL_X, 0);
        self.input.key_values.insert(MOUSE_WHEEL_Y, 0);
        *pyxel::mouse_wheel() = 0;
        pyxel::input_keys().clear();
        pyxel::input_text().clear();
        pyxel::dropped_files().clear();
    }

    pub(crate) fn reset_key(&mut self, key: Key) {
        self.input.key_states.remove(&key);
    }

    pub(crate) fn press_key(&mut self, key: Key) {
        // Detect release-then-press within the same frame
        let key_state = if self.is_same_frame_transition(key, KeyState::Pressed) {
            KeyState::ReleasedAndPressed
        } else {
            KeyState::Pressed
        };
        self.input
            .key_states
            .insert(key, (*pyxel::frame_count(), key_state));

        if key < MOUSE_KEY_START_INDEX {
            pyxel::input_keys().push(key);
        }
    }

    pub(crate) fn release_key(&mut self, key: Key) {
        // Detect press-then-release within the same frame
        let key_state = if self.is_same_frame_transition(key, KeyState::Released) {
            KeyState::PressedAndReleased
        } else {
            KeyState::Released
        };
        self.input
            .key_states
            .insert(key, (*pyxel::frame_count(), key_state));
    }

    pub(crate) fn set_key_value(&mut self, key: Key, mut value: KeyValue) {
        match key {
            MOUSE_POS_X => {
                value = ((value - self.system.screen_x) as f32 / self.system.screen_scale) as i32;
                *pyxel::mouse_x() = value;
            }
            MOUSE_POS_Y => {
                value = ((value - self.system.screen_y) as f32 / self.system.screen_scale) as i32;
                *pyxel::mouse_y() = value;
            }
            MOUSE_WHEEL_Y => {
                *pyxel::mouse_wheel() = value;
            }
            _ => {}
        }
        self.input.key_values.insert(key, value);
    }

    pub(crate) fn add_input_text(&mut self, text: &str) {
        *pyxel::input_text() += text;
    }

    pub(crate) fn add_dropped_file(&mut self, filename: &str) {
        pyxel::dropped_files().push(filename.to_string());
    }

    pub(crate) fn is_mouse_visible(&self) -> bool {
        self.input.mouse_visible
    }

    // Helpers

    fn is_current_frame(frame_count: u32) -> bool {
        frame_count == *pyxel::frame_count()
    }

    fn is_same_frame_transition(&self, key: Key, current_state: KeyState) -> bool {
        matches!(
            self.input.key_states.get(&key),
            Some((fc, state)) if *fc == *pyxel::frame_count() && *state != current_state
        )
    }

    fn is_analog_key(key: Key) -> bool {
        matches!(
            key,
            MOUSE_POS_X | MOUSE_POS_Y | MOUSE_WHEEL_X | MOUSE_WHEEL_Y
        ) || (key >= GAMEPAD_KEY_START_INDEX && (key % GAMEPAD_KEY_STRIDE) < GAMEPAD_AXIS_COUNT)
    }
}

fn screen_to_window(position: i32, scale: f32, offset: i32) -> i32 {
    // Keep the product precise so rounding cannot land below the requested pixel.
    let position = position as f64 * scale as f64;
    // Round away from zero to match truncation when reading window coordinates.
    position.abs().ceil().copysign(position) as i32 + offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mouse_position_round_trips_through_window_coordinates() {
        let mut pyxel = Pyxel {
            system: crate::system::System::new(30, crate::key::KEY_ESCAPE, true),
            resource: crate::resource::Resource::new(None, Some(0), 30),
            input: Input::new(),
            graphics: None,
        };
        pyxel.system.screen_x = 10;
        pyxel.system.screen_y = 20;
        let saved_mouse = (*pyxel::mouse_x(), *pyxel::mouse_y());

        let cases = [
            (1.0, (3, 5), (13, 25)),
            (2.0, (3, 5), (16, 30)),
            (1.25, (3, 5), (14, 27)),
            (1.5, (3, 5), (15, 28)),
            (1.075, (120, 40), (140, 64)),
            (1.25, (-3, -5), (6, 13)),
            (2.75, (42, 17), (126, 67)),
        ];

        let actual = cases.map(|(scale, (x, y), _)| {
            pyxel.system.screen_scale = scale;
            let window = (
                screen_to_window(x, scale, 10),
                screen_to_window(y, scale, 20),
            );
            pyxel.set_key_value(MOUSE_POS_X, window.0);
            pyxel.set_key_value(MOUSE_POS_Y, window.1);
            (window, (*pyxel::mouse_x(), *pyxel::mouse_y()))
        });
        *pyxel::mouse_x() = saved_mouse.0;
        *pyxel::mouse_y() = saved_mouse.1;
        assert_eq!(actual, cases.map(|(_, logical, window)| (window, logical)));
    }

    #[test]
    fn button_repeat_continues_across_frame_count_wrap() {
        let mut pyxel = Pyxel {
            system: crate::system::System::new(30, crate::key::KEY_ESCAPE, true),
            resource: crate::resource::Resource::new(None, Some(0), 30),
            input: Input::new(),
            graphics: None,
        };
        pyxel
            .input
            .key_states
            .insert(crate::key::KEY_A, (u32::MAX - 1, KeyState::Pressed));
        let saved_frame_count = *pyxel::frame_count();

        let actual = [u32::MAX - 1, u32::MAX, 0, 1, 2, 3].map(|frame| {
            *pyxel::frame_count() = frame;
            pyxel.is_button_pressed(crate::key::KEY_A, Some(3), Some(2))
        });
        *pyxel::frame_count() = saved_frame_count;
        assert_eq!(actual, [true, false, false, true, false, true]);
    }
}
