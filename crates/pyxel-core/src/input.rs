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
    pub fn is_button_down(&self, key: Key) -> bool {
        assert!(
            !self.is_analog_key(key),
            "is_button_down is called with an analog key 0x{key:X}"
        );

        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            match key_state {
                KeyState::Pressed | KeyState::ReleasedAndPressed => true,
                KeyState::PressedAndReleased => self.is_current_frame(*frame_count),
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
        assert!(
            !self.is_analog_key(key),
            "is_button_pressed is called with an analog key 0x{key:X}"
        );

        let Some((frame_count, key_state)) = self.input.key_states.get(&key) else {
            return false;
        };

        if *key_state == KeyState::Released {
            return false;
        }

        if self.is_current_frame(*frame_count) {
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
        let elapsed = *pyxel::frame_count() as i32 - (*frame_count + hold) as i32;
        elapsed >= 0 && elapsed % repeat as i32 == 0
    }

    pub fn is_button_released(&self, key: Key) -> bool {
        assert!(
            !self.is_analog_key(key),
            "is_button_released is called with an analog key 0x{key:X}"
        );

        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            match key_state {
                KeyState::Pressed => false,
                _ => self.is_current_frame(*frame_count),
            }
        } else {
            false
        }
    }

    pub fn button_value(&self, key: Key) -> KeyValue {
        assert!(
            self.is_analog_key(key),
            "button_value is called with a non-analog key 0x{key:X}"
        );

        self.input.key_values.get(&key).copied().unwrap_or(0)
    }

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
                x * self.system.screen_scale as i32 + self.system.screen_x,
                y * self.system.screen_scale as i32 + self.system.screen_y,
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

    pub fn set_dropped_files(&mut self, files: &[&str]) {
        pyxel::dropped_files().clear();
        for file in files {
            self.add_dropped_file(file);
        }
    }

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

    fn is_current_frame(&self, frame_count: u32) -> bool {
        frame_count == *pyxel::frame_count()
    }

    fn is_same_frame_transition(&self, key: Key, current_state: KeyState) -> bool {
        matches!(
            self.input.key_states.get(&key),
            Some((fc, state)) if *fc == *pyxel::frame_count() && *state != current_state
        )
    }

    fn is_analog_key(&self, key: Key) -> bool {
        matches!(
            key,
            MOUSE_POS_X | MOUSE_POS_Y | MOUSE_WHEEL_X | MOUSE_WHEEL_Y
        ) || (key >= GAMEPAD_KEY_START_INDEX
            && (key % GAMEPAD_KEY_STRIDE) < GAMEPAD_AXIS_COUNT)
    }
}
