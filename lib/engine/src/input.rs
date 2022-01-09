use std::collections::HashMap;

use crate::event::Event;
use crate::key::*;
use crate::platform::Platform;
use crate::types::{Key, KeyValue};
use crate::utils::as_i32;
use crate::Pyxel;

#[derive(PartialEq)]
enum KeyState {
    Pressed,
    Released,
    PressedAndReleased,
    ReleasedAndPressed,
}

pub struct Input {
    is_mouse_visible: bool,
    key_states: HashMap<Key, (u32, KeyState)>,
    key_values: HashMap<Key, KeyValue>,
    input_keys: Vec<Key>,
    input_text: String,
    drop_files: Vec<String>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            is_mouse_visible: false,
            key_states: HashMap::new(),
            key_values: HashMap::new(),
            input_keys: Vec::new(),
            input_text: "".to_string(),
            drop_files: Vec::new(),
        }
    }

    pub fn is_mouse_visible(&self) -> bool {
        self.is_mouse_visible
    }

    pub fn reset_input_states(&mut self) {
        self.key_values.insert(MOUSE_WHEEL_X, 0);
        self.key_values.insert(MOUSE_WHEEL_Y, 0);
        self.input_keys.clear();
        self.input_text = "".to_string();
        self.drop_files.clear();
    }

    pub fn process_input_event(&mut self, event: Event, frame_count: u32) {
        match event {
            // System events
            Event::Quit => {}
            Event::DropFile { filename } => {
                self.drop_files.push(filename);
            }

            // Window events
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Maximized => {}
            Event::Minimized => {}

            // Key events
            Event::KeyDown { keycode } => {
                self.press_key(keycode, frame_count);
            }
            Event::KeyUp { keycode } => {
                self.release_key(keycode, frame_count);
            }
            Event::TextInput { text } => {
                self.input_text += &text;
            }

            // Mouse events
            Event::MouseMotion { x, y } => {
                self.key_values.insert(MOUSE_POS_X, x);
                self.key_values.insert(MOUSE_POS_Y, y);
            }
            Event::MouseButtonDown { button } => {
                self.press_key(MOUSE_BUTTON_LEFT + button as Key, frame_count);
            }
            Event::MouseButtonUp { button } => {
                self.release_key(MOUSE_BUTTON_LEFT + button as Key, frame_count);
            }
            Event::MouseWheel { x, y } => {
                *self.key_values.entry(MOUSE_WHEEL_X).or_insert(0) += x;
                *self.key_values.entry(MOUSE_WHEEL_Y).or_insert(0) += y;
            }

            // Controller events
            Event::ControllerAxisMotion { which, axis, value } => {
                let offset = if which == 0 {
                    0
                } else if which == 1 {
                    GAMEPAD2_AXIS_LEFTX - GAMEPAD1_AXIS_LEFTX
                } else {
                    return;
                };
                self.key_values
                    .insert(GAMEPAD1_AXIS_LEFTX + axis as Key + offset, value);
            }
            Event::ControllerButtonDown { which, button } => {
                let offset = if which == 0 {
                    0
                } else if which == 1 {
                    GAMEPAD2_BUTTON_A - GAMEPAD1_BUTTON_A
                } else {
                    return;
                };
                self.press_key(GAMEPAD1_BUTTON_A + button as Key + offset, frame_count);
            }
            Event::ControllerButtonUp { which, button } => {
                let offset = if which == 0 {
                    0
                } else if which == 1 {
                    GAMEPAD2_BUTTON_A - GAMEPAD1_BUTTON_A
                } else {
                    return;
                };
                self.release_key(GAMEPAD1_BUTTON_A + button as Key + offset, frame_count);
            }
        }
    }

    fn press_key(&mut self, key: Key, frame_count: u32) {
        let mut key_state = KeyState::Pressed;
        if let Some((last_frame_count, last_key_state)) = self.key_states.get(&key) {
            if *last_frame_count == frame_count && *last_key_state != KeyState::Pressed {
                key_state = KeyState::ReleasedAndPressed;
            }
        }
        self.key_states.insert(key, (frame_count, key_state));
        if is_keyboard_key(key) {
            self.input_keys.push(key);
        }
        if let Some(key) = to_integrated_key(key) {
            self.press_key(key, frame_count)
        }
    }

    fn release_key(&mut self, key: Key, frame_count: u32) {
        let mut key_state = KeyState::Released;
        if let Some((last_frame_count, last_key_state)) = self.key_states.get(&key) {
            if *last_frame_count == frame_count && *last_key_state != KeyState::Released {
                key_state = KeyState::PressedAndReleased;
            }
        }
        self.key_states.insert(key, (frame_count, key_state));
        if let Some(key) = to_integrated_key(key) {
            self.release_key(key, frame_count)
        }
    }
}

impl Pyxel {
    pub fn mouse_x(&self) -> i32 {
        *self.input.key_values.get(&MOUSE_POS_X).unwrap_or(&0)
    }

    pub fn mouse_y(&self) -> i32 {
        *self.input.key_values.get(&MOUSE_POS_Y).unwrap_or(&0)
    }

    pub fn mouse_wheel(&self) -> i32 {
        *self.input.key_values.get(&MOUSE_WHEEL_Y).unwrap_or(&0)
    }

    pub fn input_keys(&self) -> &Vec<Key> {
        &self.input.input_keys
    }

    pub fn input_text(&self) -> &str {
        &self.input.input_text
    }

    pub fn drop_files(&self) -> &Vec<String> {
        &self.input.drop_files
    }

    pub fn btn(&self, key: Key) -> bool {
        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            if *key_state == KeyState::Pressed
                || *key_state == KeyState::ReleasedAndPressed
                || *frame_count == self.frame_count() && *key_state == KeyState::PressedAndReleased
            {
                return true;
            }
        }
        false
    }

    pub fn btnp(
        &self,
        key: Key,
        hold_frame_count: Option<u32>,
        period_frame_count: Option<u32>,
    ) -> bool {
        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            if *key_state == KeyState::Released {
                return false;
            }
            if *frame_count == self.frame_count() {
                return true;
            }
            if *key_state == KeyState::PressedAndReleased {
                return false;
            }
            let hold_frame_count = hold_frame_count.unwrap_or(0);
            let period_frame_count = period_frame_count.unwrap_or(0);
            if hold_frame_count == 0 || period_frame_count == 0 {
                return false;
            }
            let elapsed_frames =
                self.frame_count() as i32 - (*frame_count + hold_frame_count) as i32;
            if elapsed_frames > 0 && elapsed_frames % period_frame_count as i32 == 0 {
                return true;
            }
        }
        false
    }

    pub fn btnr(&self, key: Key) -> bool {
        if let Some((frame_count, key_state)) = self.input.key_states.get(&key) {
            if *key_state == KeyState::Pressed {
                return false;
            }
            if *frame_count == self.frame_count() {
                return true;
            }
        }
        false
    }

    pub fn btnv(&self, key: Key) -> KeyValue {
        self.input.key_values.get(&key).copied().unwrap_or(0)
    }

    pub fn mouse(&mut self, is_visible: bool) {
        self.input.is_mouse_visible = is_visible;
    }

    pub fn set_btn(&mut self, key: Key, key_state: bool) {
        if key_state {
            self.input.press_key(key, self.frame_count());
        } else {
            self.input.release_key(key, self.frame_count());
        }
    }

    pub fn set_btnv(&mut self, key: Key, key_value: f64) {
        let key_value = as_i32(key_value);
        self.input.key_values.insert(key, key_value);
    }

    pub fn set_mouse_pos(&mut self, x: f64, y: f64) {
        let x = as_i32(x);
        let y = as_i32(y);
        self.input.key_values.insert(MOUSE_POS_X, x);
        self.input.key_values.insert(MOUSE_POS_Y, y);
        self.platform.move_cursor(x, y);
    }
}
