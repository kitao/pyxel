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
            is_mouse_visible: true,
            key_states: HashMap::new(),
            key_values: HashMap::new(),
            text_input: String::from(""),
            drop_files: Vec::new(),
        }
    }

    pub fn is_mouse_visible(&self) -> bool {
        self.is_mouse_visible
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
        repeat_frame_count: Option<u32>,
    ) -> bool {
        if let Some(KeyState::Pressed { frame_count }) = self.input.key_states.get(&key) {
            if *frame_count == self.frame_count() {
                return true;
            }

            let hold_frame_count = hold_frame_count.unwrap_or(0);
            let repeat_frame_count = repeat_frame_count.unwrap_or(0);

            if hold_frame_count == 0 || repeat_frame_count == 0 {
                return false;
            }

            let elapsed_frames =
                self.frame_count() as i32 - (*frame_count + hold_frame_count) as i32;

            if elapsed_frames > 0 && elapsed_frames % repeat_frame_count as i32 == 0 {
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

    pub(crate) fn reset_input_states(&mut self) {
        self.input.key_values.insert(MOUSE_WHEEL_X, 0);
        self.input.key_values.insert(MOUSE_WHEEL_Y, 0);
        self.input.text_input = String::from("");
        self.input.drop_files.clear();
    }

    pub(crate) fn process_input_event(&mut self, event: Event) {
        match event {
            //
            // System Events
            //
            Event::DropFile { filename } => {
                self.input.drop_files.push(filename);
            }

            //
            // Key Events
            //
            Event::KeyDown { key } => {
                if key >= KEY_MIN_VALUE && key <= KEY_MAX_VALUE {
                    self.press_key(key);

                    if let Some(key) = Input::get_common_key(key) {
                        self.press_key(key);
                    }
                }
            }
            Event::KeyUp { key } => {
                if key >= KEY_MIN_VALUE && key <= KEY_MAX_VALUE {
                    self.release_key(key);

                    if let Some(key) = Input::get_common_key(key) {
                        self.release_key(key);
                    }
                }
            }
            Event::TextInput { text } => {
                self.input.text_input += &text;
            }

            //
            // Mouse Events
            //
            Event::MouseMotion { x, y } => {
                self.input.key_values.insert(MOUSE_POS_X, x as KeyValue);
                self.input.key_values.insert(MOUSE_POS_Y, y as KeyValue);
            }
            Event::MouseButtonDown { button } => {
                self.press_key(MOUSE_BUTTON_LEFT + button as Key);
            }
            Event::MouseButtonUp { button } => {
                self.release_key(MOUSE_BUTTON_LEFT + button as Key);
            }
            Event::MouseWheel { x, y } => {
                *self.input.key_values.entry(MOUSE_WHEEL_X).or_insert(0) += x as KeyValue;
                *self.input.key_values.entry(MOUSE_WHEEL_Y).or_insert(0) += y as KeyValue;
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

                self.input
                    .key_values
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

                self.press_key(GAMEPAD1_BUTTON_A + button as Key + offset);
            }
            Event::ControllerButtonUp { which, button } => {
                let offset = if which == 0 {
                    0
                } else if which == 1 {
                    GAMEPAD2_BUTTON_A - GAMEPAD1_BUTTON_A
                } else {
                    return;
                };

                self.release_key(GAMEPAD1_BUTTON_A + button as Key + offset);
            }

            //
            // Default
            //
            _ => {}
        }
    }

    fn press_key(&mut self, key: Key) {
        self.input.key_states.insert(
            key,
            KeyState::Pressed {
                frame_count: self.frame_count(),
            },
        );
    }

    fn release_key(&mut self, key: Key) {
        self.input.key_states.insert(
            key,
            KeyState::Released {
                frame_count: self.frame_count(),
            },
        );
    }
}

/*
Input::Input() {
  gamepad1_ = SDL_GameControllerOpen(0);
  gamepad2_ = SDL_GameControllerOpen(1);

  const uint8_t data[] = {8};
  blank_cursor_ = SDL_CreateCursor(data, data, 1, 1, 0, 0);
  normal_cursor_ = SDL_GetCursor();
  SDL_SetCursor(blank_cursor_);

  is_mouse_visible_ = false;

  for (int32_t i = 0; i < KEY_COUNT; i++) {
    key_state_[i] = 0;
  }
}

Input::~Input() {
  SDL_FreeCursor(blank_cursor_);

  if (gamepad1_) {
    SDL_GameControllerClose(gamepad1_);
  }

  if (gamepad2_) {
    SDL_GameControllerClose(gamepad2_);
  }
}

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
