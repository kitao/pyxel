use std::collections::HashMap;

use crate::keys::{Key, KeyValue, MOUSE_POS_X, MOUSE_POS_Y, MOUSE_WHEEL_X, MOUSE_WHEEL_Y};
use crate::pyxel::Pyxel;
use crate::utils::f64_to_i32;

#[derive(PartialEq)]
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
    pub fn btn(&mut self, key: Key) -> bool {
        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            if *key_state == KeyState::Pressed
                || *key_state == KeyState::ReleasedAndPressed
                || *frame_count == self.frame_count && *key_state == KeyState::PressedAndReleased
            {
                return true;
            }
        }
        false
    }

    pub fn btnp(
        &mut self,
        key: Key,
        hold_frame_count: Option<u32>,
        repeat_frame_count: Option<u32>,
    ) -> bool {
        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            if *key_state == KeyState::Released {
                return false;
            }
            if *frame_count == self.frame_count {
                return true;
            }
            if *key_state == KeyState::PressedAndReleased {
                return false;
            }
            let hold_frame_count = hold_frame_count.unwrap_or(0);
            let repeat_frame_count = repeat_frame_count.unwrap_or(0);
            if repeat_frame_count == 0 {
                return false;
            }
            let elapsed_frames = self.frame_count as i32 - (*frame_count + hold_frame_count) as i32;
            if elapsed_frames >= 0 && elapsed_frames % repeat_frame_count as i32 == 0 {
                return true;
            }
        }
        false
    }

    pub fn btnr(&mut self, key: Key) -> bool {
        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            if *key_state == KeyState::Pressed {
                return false;
            }
            if *frame_count == self.frame_count {
                return true;
            }
        }
        false
    }

    pub fn btnv(&mut self, key: Key) -> KeyValue {
        self.input.key_values.get(&key).copied().unwrap_or(0)
    }

    pub fn mouse(&mut self, visible: bool) {
        self.input.mouse_visible = visible;
    }

    pub fn warp_mouse(&mut self, x: f64, y: f64) {
        let x = f64_to_i32(x);
        let y = f64_to_i32(y);
        self.input.key_values.insert(MOUSE_POS_X, x);
        self.input.key_values.insert(MOUSE_POS_Y, y);
        pyxel_platform::set_mouse_pos(
            x * self.system.screen_scale as i32 + self.system.screen_x,
            y * self.system.screen_scale as i32 + self.system.screen_y,
        );
    }

    pub(crate) fn start_input_frame(&mut self) {
        self.input.key_values.insert(MOUSE_WHEEL_X, 0);
        self.input.key_values.insert(MOUSE_WHEEL_Y, 0);
        self.mouse_wheel = 0;
        self.input_text = String::new();
        self.dropped_files.clear();
    }

    pub(crate) fn reset_key(&mut self, key: Key) {
        self.input.key_states.remove(&key);
    }

    pub(crate) fn press_key(&mut self, key: Key) {
        let mut key_state = KeyState::Pressed;
        if let Some((last_frame_count, last_key_state)) = self.input.key_states.get(&key) {
            if *last_frame_count == self.frame_count && *last_key_state != KeyState::Pressed {
                key_state = KeyState::ReleasedAndPressed;
            }
        }
        self.input
            .key_states
            .insert(key, (self.frame_count, key_state));
    }

    pub(crate) fn release_key(&mut self, key: Key) {
        let mut key_state = KeyState::Released;
        if let Some((last_frame_count, last_key_state)) = self.input.key_states.get(&key) {
            if *last_frame_count == self.frame_count && *last_key_state != KeyState::Released {
                key_state = KeyState::PressedAndReleased;
            }
        }
        self.input
            .key_states
            .insert(key, (self.frame_count, key_state));
    }

    pub(crate) fn change_key_value(&mut self, key: Key, value: KeyValue) {
        let mut value = value;
        match key {
            MOUSE_POS_X => {
                value = (value - self.system.screen_x) / self.system.screen_scale as i32;
                self.mouse_x = value;
            }
            MOUSE_POS_Y => {
                value = (value - self.system.screen_y) / self.system.screen_scale as i32;
                self.mouse_y = value;
            }
            MOUSE_WHEEL_Y => {
                self.mouse_wheel = value;
            }
            _ => {}
        }
        self.input.key_values.insert(key, value);
    }

    pub(crate) fn add_input_text(&mut self, text: &str) {
        self.input_text += text;
    }

    pub(crate) fn add_dropped_file(&mut self, filename: &str) {
        self.dropped_files.push(filename.to_string());
    }

    pub(crate) fn is_mouse_visible(&self) -> bool {
        self.input.mouse_visible
    }
}
