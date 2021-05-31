use std::collections::HashMap;

use crate::event::Event;
use crate::key::*;
use crate::rectarea::RectArea;

enum KeyState {
  Pressed { frame_count: u32 },
  Released { frame_count: u32 },
}

pub struct Input {
  frame_count: u32,
  is_mouse_visible: bool,
  window_rect: RectArea,
  key_states: HashMap<KeyCode, KeyState>,
  key_values: HashMap<KeyCode, KeyValue>,
}

impl Input {
  pub fn new() -> Input {
    Input {
      frame_count: 0,
      window_rect: RectArea::with_size(0, 0, 0, 0),
      is_mouse_visible: true,
      key_states: HashMap::new(),
      key_values: HashMap::new(),
    }
  }

  #[inline]
  pub fn is_key_pressed(
    &self,
    key: KeyCode,
    hold_frame: Option<u32>,
    period_frame: Option<u32>,
  ) -> bool {
    false

    /*
    if (key < 0 || key >= KEY_COUNT) {
      PYXEL_ERROR("invalid key");
    }

    if (frame_count_ == 0) {
      return false;
    }

    int32_t press_frame = key_state_[key];

    if (press_frame == frame_count_) {
      return true;
    }

    if (press_frame <= 0 || period_frame <= 0) {
      return false;
    }

    int32_t elapsed_frame = frame_count_ - (press_frame + hold_frame);

    if (elapsed_frame >= 0 && elapsed_frame % period_frame == 0) {
      return true;
    }

    return false;
    */
  }

  #[inline]
  pub fn is_key_released(&self, key: KeyCode) -> bool {
    false

    /*
    if (key < 0 || key >= KEY_COUNT) {
      PYXEL_ERROR("invalid key");
    }

    if (frame_count_ == 0) {
      return false;
    }

    return key_state_[key] == -frame_count_;
    */
  }

  #[inline]
  pub fn is_key_on(&self, key: KeyCode) -> bool {
    match self.key_states.get(&key) {
      Some(KeyState::Pressed { .. }) => true,
      _ => false,
    }
  }

  #[inline]
  pub fn key_value(&self, key: KeyCode) -> KeyValue {
    self.key_values.get(&key).cloned().unwrap_or(0)
  }

  #[inline]
  pub fn is_mouse_visible(&self) -> bool {
    self.is_mouse_visible
  }

  #[inline]
  pub fn set_mouse_visible(&mut self, is_mouse_visible: bool) {
    self.is_mouse_visible = is_mouse_visible;
  }

  #[inline]
  pub(crate) fn start_update(&mut self, frame_count: u32, window_rect: RectArea) {
    self.frame_count = frame_count;
    self.window_rect = window_rect;

    self.key_values.insert(MOUSE_WHEEL_X, 0);
    self.key_values.insert(MOUSE_WHEEL_Y, 0);
  }

  #[inline]
  pub(crate) fn process_event(&mut self, event: Event) {
    match event {
      Event::KeyDown { key } => {
        if key >= KEY_MIN_VALUE && key <= KEY_MAX_VALUE {
          self.press_key(key);

          if let Some(key) = Self::get_common_key(key) {
            self.press_key(key);
          }
        }
      }

      Event::KeyUp { key } => {
        if key >= KEY_MIN_VALUE && key <= KEY_MAX_VALUE {
          self.release_key(key);

          if let Some(key) = Self::get_common_key(key) {
            self.release_key(key);
          }
        }
      }

      Event::TextInput { text } => {}

      Event::MouseMotion { x, y } => {
        self.key_values.insert(MOUSE_POS_X, x as KeyValue);
        self.key_values.insert(MOUSE_POS_Y, y as KeyValue);
      }

      Event::MouseButtonDown { button } => {
        self.press_key(MOUSE_BUTTON_LEFT + button as KeyCode);
      }

      Event::MouseButtonUp { button } => {
        self.release_key(MOUSE_BUTTON_LEFT + button as KeyCode);
      }

      Event::MouseWheel { x, y } => {
        *self.key_values.entry(MOUSE_WHEEL_X).or_insert(0) += x as KeyValue;
        *self.key_values.entry(MOUSE_WHEEL_Y).or_insert(0) += y as KeyValue;
      }

      Event::ControllerAxisMotion { which, axis, value } => {
        let offset = if which == 0 {
          0
        } else if which == 1 {
          GAMEPAD2_AXIS_LEFTX - GAMEPAD1_AXIS_LEFTX
        } else {
          return;
        };

        self.key_values.insert(
          GAMEPAD1_AXIS_LEFTX + axis as KeyCode + offset as KeyCode,
          value,
        );
      }

      Event::ControllerButtonDown { which, button } => {
        let offset = if which == 0 {
          0
        } else if which == 1 {
          GAMEPAD2_BUTTON_A - GAMEPAD1_BUTTON_A
        } else {
          return;
        };

        self.press_key(GAMEPAD1_BUTTON_A + button as KeyCode + offset);
      }

      Event::ControllerButtonUp { which, button } => {
        let offset = if which == 0 {
          0
        } else if which == 1 {
          GAMEPAD2_BUTTON_A - GAMEPAD1_BUTTON_A
        } else {
          return;
        };

        self.release_key(GAMEPAD1_BUTTON_A + button as KeyCode + offset);
      }

      _ => {}
    }
    //
  }

  #[inline]
  pub(crate) fn end_update(&mut self) {
    //
  }

  #[inline]
  fn get_common_key(key: KeyCode) -> Option<KeyCode> {
    match key {
      KEY_LSHIFT | KEY_RSHIFT => Some(KEY_SHIFT),
      KEY_LCTRL | KEY_RCTRL => Some(KEY_CTRL),
      KEY_LALT | KEY_RALT => Some(KEY_ALT),
      KEY_LGUI | KEY_RGUI => Some(KEY_GUI),
      _ => None,
    }
  }

  #[inline]
  fn press_key(&mut self, key: KeyCode) {
    self.key_states.insert(
      key,
      KeyState::Pressed {
        frame_count: self.frame_count,
      },
    );
  }

  #[inline]
  fn release_key(&mut self, key: KeyCode) {
    self.key_states.insert(
      key,
      KeyState::Released {
        frame_count: self.frame_count,
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
