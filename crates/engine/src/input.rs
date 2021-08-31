use std::collections::HashMap;

use crate::event::Event;
use crate::key::*;
use crate::types::{Key, KeyValue};
use crate::Pyxel;

enum KeyState {
    Pressed { frame_count: u32 },
    Released { frame_count: u32 },
}

pub struct Input {
    is_mouse_visible: bool,
    key_states: HashMap<Key, KeyState>,
    key_values: HashMap<Key, KeyValue>,
    text_input: String,
    drop_files: Vec<String>,
}

impl Input {
    pub fn new() -> Input {
        Input {
            is_mouse_visible: false,
            key_states: HashMap::new(),
            key_values: HashMap::new(),
            text_input: "".to_string(),
            drop_files: Vec::new(),
        }
    }

    pub fn is_mouse_visible(&self) -> bool {
        self.is_mouse_visible
    }

    pub fn reset_input_states(&mut self) {
        self.key_values.insert(MOUSE_WHEEL_X, 0);
        self.key_values.insert(MOUSE_WHEEL_Y, 0);
        self.text_input = "".to_string();
        self.drop_files.clear();
    }

    pub fn process_input_event(&mut self, event: Event, frame_count: u32) {
        match event {
            //
            // System Events
            //
            Event::DropFile { filename } => {
                self.drop_files.push(filename);
            }

            //
            // Key Events
            //
            Event::KeyDown { key } => {
                if key >= KEY_MIN_VALUE && key <= KEY_MAX_VALUE {
                    self.press_key(key, frame_count);

                    if let Some(key) = Input::get_common_key(key) {
                        self.press_key(key, frame_count);
                    }
                }
            }
            Event::KeyUp { key } => {
                if key >= KEY_MIN_VALUE && key <= KEY_MAX_VALUE {
                    self.release_key(key, frame_count);

                    if let Some(key) = Input::get_common_key(key) {
                        self.release_key(key, frame_count);
                    }
                }
            }
            Event::TextInput { text } => {
                self.text_input += &text;
            }

            //
            // Mouse Events
            //
            Event::MouseMotion { x, y } => {
                self.key_values.insert(MOUSE_POS_X, x as KeyValue);
                self.key_values.insert(MOUSE_POS_Y, y as KeyValue);
            }
            Event::MouseButtonDown { button } => {
                self.press_key(MOUSE_BUTTON_LEFT + button as Key, frame_count);
            }
            Event::MouseButtonUp { button } => {
                self.release_key(MOUSE_BUTTON_LEFT + button as Key, frame_count);
            }
            Event::MouseWheel { x, y } => {
                *self.key_values.entry(MOUSE_WHEEL_X).or_insert(0) += x as KeyValue;
                *self.key_values.entry(MOUSE_WHEEL_Y).or_insert(0) += y as KeyValue;
            }

            //
            // Controller Events
            //
            Event::ControllerAxisMotion { which, axis, value } => {
                let offset = if which == 0 {
                    0
                } else if which == 1 {
                    GAMEPAD2_AXIS_LEFTX - GAMEPAD1_AXIS_LEFTX
                } else {
                    return;
                };

                self.key_values
                    .insert(GAMEPAD1_AXIS_LEFTX + axis as Key + offset as Key, value);
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

            //
            // Default
            //
            _ => {}
        }
    }

    fn get_common_key(key: Key) -> Option<Key> {
        match key {
            KEY_LSHIFT | KEY_RSHIFT => Some(KEY_SHIFT),
            KEY_LCTRL | KEY_RCTRL => Some(KEY_CTRL),
            KEY_LALT | KEY_RALT => Some(KEY_ALT),
            KEY_LGUI | KEY_RGUI => Some(KEY_GUI),
            _ => None,
        }
    }

    fn press_key(&mut self, key: Key, frame_count: u32) {
        self.key_states.insert(
            key,
            KeyState::Pressed {
                frame_count: frame_count,
            },
        );
    }

    fn release_key(&mut self, key: Key, frame_count: u32) {
        self.key_states.insert(
            key,
            KeyState::Released {
                frame_count: frame_count,
            },
        );
    }
}

impl Pyxel {
    pub fn mouse_x(&self) -> i32 {
        *self.input.key_values.get(&MOUSE_POS_X).unwrap()
    }

    pub fn mouse_y(&self) -> i32 {
        *self.input.key_values.get(&MOUSE_POS_Y).unwrap()
    }

    pub fn mouse_wheel(&self) -> i32 {
        *self.input.key_values.get(&MOUSE_WHEEL_Y).unwrap()
    }

    pub fn text_input(&self) -> &str {
        &self.input.text_input
    }

    pub fn drop_files(&self) -> &Vec<String> {
        &self.input.drop_files
    }

    pub fn mouse(&mut self, is_visible: bool) {
        self.input.is_mouse_visible = is_visible;
    }

    pub fn btn(&self, key: Key) -> bool {
        if let Some(KeyState::Pressed { .. }) = self.input.key_states.get(&key) {
            true
        } else {
            false
        }
    }

    pub fn btnp(
        &self,
        key: Key,
        hold_frame_count: Option<u32>,
        period_frame_count: Option<u32>,
    ) -> bool {
        if let Some(KeyState::Pressed { frame_count }) = self.input.key_states.get(&key) {
            if *frame_count == self.frame_count() {
                return true;
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
        if let Some(KeyState::Released { frame_count }) = self.input.key_states.get(&key) {
            if *frame_count == self.frame_count() {
                return true;
            }
        }

        false
    }

    pub fn btnv(&self, key: Key) -> KeyValue {
        self.input.key_values.get(&key).cloned().unwrap_or(0)
    }
}

/*
void Input::Update(Window* window, int32_t frame_count) {
  if (is_mouse_visible_) {
    SDL_ShowCursor(true);
    SDL_SetCursor(mouse_x_ >= 0 && mouse_x_ < window->ScreenWidth() &&
                          mouse_y_ >= 0 && mouse_y_ < window->ScreenHeight()
                      ? blank_cursor_
                      : normal_cursor_);
  } else {
    SDL_ShowCursor(false);
  }
}
*/
