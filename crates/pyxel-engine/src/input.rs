use std::collections::HashMap;

use pyxel_platform::keys::*;

use crate::utils::as_i32;

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
    input_text: String,
    dropped_files: Vec<String>,
}

unsafe_singleton!(Input);

impl Input {
    pub fn init() {
        Self::set_instance(Self {
            mouse_visible: false,
            key_states: HashMap::new(),
            key_values: HashMap::new(),
            input_text: String::new(),
            dropped_files: Vec::new(),
        });
    }

    /*pub const fn is_mouse_visible(&self) -> bool {
        self.is_mouse_visible
    }*/

    pub fn reset_input_states(&mut self) {
        self.key_values.insert(MOUSE_WHEEL_X, 0);
        self.key_values.insert(MOUSE_WHEEL_Y, 0);
        self.input_text = String::new();
        self.dropped_files.clear();
    }

    pub fn press_key(&mut self, key: Key) {
        let frame_count = crate::frame_count();
        let mut key_state = KeyState::Pressed;
        if let Some((last_frame_count, last_key_state)) = self.key_states.get(&key) {
            if *last_frame_count == frame_count && *last_key_state != KeyState::Pressed {
                key_state = KeyState::ReleasedAndPressed;
            }
        }
        self.key_states.insert(key, (frame_count, key_state));
    }

    pub fn release_key(&mut self, key: Key) {
        let frame_count = crate::frame_count();
        let mut key_state = KeyState::Released;
        if let Some((last_frame_count, last_key_state)) = self.key_states.get(&key) {
            if *last_frame_count == frame_count && *last_key_state != KeyState::Released {
                key_state = KeyState::PressedAndReleased;
            }
        }
        self.key_states.insert(key, (frame_count, key_state));
    }

    pub fn change_key_value(&mut self, key: Key, value: KeyValue) {
        self.key_values.insert(key, value);
    }

    pub fn add_input_text(&mut self, text: &str) {
        self.input_text += text;
    }

    pub fn add_dropped_file(&mut self, filename: &str) {
        self.dropped_files.push(filename.to_string());
    }
}

pub fn mouse_x() -> i32 {
    *Input::instance().key_values.get(&MOUSE_POS_X).unwrap_or(&0)
}

pub fn mouse_y() -> i32 {
    *Input::instance().key_values.get(&MOUSE_POS_Y).unwrap_or(&0)
}

pub fn mouse_wheel() -> i32 {
    *Input::instance()
        .key_values
        .get(&MOUSE_WHEEL_Y)
        .unwrap_or(&0)
}

pub fn input_text() -> &'static str {
    &Input::instance().input_text
}

pub fn drop_files() -> &'static Vec<String> {
    &Input::instance().dropped_files
}

pub fn btn(key: Key) -> bool {
    if let Some((frame_count, key_state)) = Input::instance().key_states.get(&key) {
        if *key_state == KeyState::Pressed
            || *key_state == KeyState::ReleasedAndPressed
            || *frame_count == crate::frame_count() && *key_state == KeyState::PressedAndReleased
        {
            return true;
        }
    }
    false
}

pub fn btnp(key: Key, hold_frame_count: Option<u32>, repeat_frame_count: Option<u32>) -> bool {
    if let Some((frame_count, key_state)) = Input::instance().key_states.get(&key) {
        if *key_state == KeyState::Released {
            return false;
        }
        if *frame_count == crate::frame_count() {
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
        let elapsed_frames = crate::frame_count() as i32 - (*frame_count + hold_frame_count) as i32;
        if elapsed_frames >= 0 && elapsed_frames % repeat_frame_count as i32 == 0 {
            return true;
        }
    }
    false
}

pub fn btnr(key: Key) -> bool {
    if let Some((frame_count, key_state)) = Input::instance().key_states.get(&key) {
        if *key_state == KeyState::Pressed {
            return false;
        }
        if *frame_count == crate::frame_count() {
            return true;
        }
    }
    false
}

pub fn btnv(key: Key) -> KeyValue {
    Input::instance().key_values.get(&key).copied().unwrap_or(0)
}

pub fn mouse(visible: bool) {
    Input::instance().mouse_visible = visible;
}

pub fn set_mouse_pos(x: f64, y: f64) {
    let x = as_i32(x);
    let y = as_i32(y);
    Input::instance().key_values.insert(MOUSE_POS_X, x);
    Input::instance().key_values.insert(MOUSE_POS_Y, y);
    pyxel_platform::set_mouse_pos(x, y);
}
