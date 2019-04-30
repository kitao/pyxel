#include "pyxelcore/input.h"

#include "pyxelcore/system.h"
#include "pyxelcore/utilities.h"

#include <SDL2/SDL.h>

namespace pyxelcore {

const uint8_t SCANCODE_TABLE[SDL_KEY_COUNT] = {
    SDL_SCANCODE_SPACE,
    SDL_SCANCODE_APOSTROPHE,
    SDL_SCANCODE_COMMA,
    SDL_SCANCODE_MINUS,
    SDL_SCANCODE_PERIOD,
    SDL_SCANCODE_SLASH,
    SDL_SCANCODE_0,
    SDL_SCANCODE_1,
    SDL_SCANCODE_2,
    SDL_SCANCODE_3,
    SDL_SCANCODE_4,
    SDL_SCANCODE_5,
    SDL_SCANCODE_6,
    SDL_SCANCODE_7,
    SDL_SCANCODE_8,
    SDL_SCANCODE_9,
    SDL_SCANCODE_SEMICOLON,
    SDL_SCANCODE_EQUALS,
    SDL_SCANCODE_A,
    SDL_SCANCODE_B,
    SDL_SCANCODE_C,
    SDL_SCANCODE_D,
    SDL_SCANCODE_E,
    SDL_SCANCODE_F,
    SDL_SCANCODE_G,
    SDL_SCANCODE_H,
    SDL_SCANCODE_I,
    SDL_SCANCODE_J,
    SDL_SCANCODE_K,
    SDL_SCANCODE_L,
    SDL_SCANCODE_M,
    SDL_SCANCODE_N,
    SDL_SCANCODE_O,
    SDL_SCANCODE_P,
    SDL_SCANCODE_Q,
    SDL_SCANCODE_R,
    SDL_SCANCODE_S,
    SDL_SCANCODE_T,
    SDL_SCANCODE_U,
    SDL_SCANCODE_V,
    SDL_SCANCODE_W,
    SDL_SCANCODE_X,
    SDL_SCANCODE_Y,
    SDL_SCANCODE_Z,
    SDL_SCANCODE_LEFTBRACKET,
    SDL_SCANCODE_BACKSLASH,
    SDL_SCANCODE_RIGHTBRACKET,
    SDL_SCANCODE_GRAVE,
    SDL_SCANCODE_ESCAPE,
    SDL_SCANCODE_RETURN,
    SDL_SCANCODE_TAB,
    SDL_SCANCODE_BACKSPACE,
    SDL_SCANCODE_INSERT,
    SDL_SCANCODE_DELETE,
    SDL_SCANCODE_RIGHT,
    SDL_SCANCODE_LEFT,
    SDL_SCANCODE_DOWN,
    SDL_SCANCODE_UP,
    SDL_SCANCODE_PAGEUP,
    SDL_SCANCODE_PAGEDOWN,
    SDL_SCANCODE_HOME,
    SDL_SCANCODE_END,
    SDL_SCANCODE_CAPSLOCK,
    SDL_SCANCODE_SCROLLLOCK,
    SDL_SCANCODE_NUMLOCKCLEAR,
    SDL_SCANCODE_PRINTSCREEN,
    SDL_SCANCODE_PAUSE,
    SDL_SCANCODE_F1,
    SDL_SCANCODE_F2,
    SDL_SCANCODE_F3,
    SDL_SCANCODE_F4,
    SDL_SCANCODE_F5,
    SDL_SCANCODE_F6,
    SDL_SCANCODE_F7,
    SDL_SCANCODE_F8,
    SDL_SCANCODE_F9,
    SDL_SCANCODE_F10,
    SDL_SCANCODE_F11,
    SDL_SCANCODE_F12,
    SDL_SCANCODE_KP_0,
    SDL_SCANCODE_KP_1,
    SDL_SCANCODE_KP_2,
    SDL_SCANCODE_KP_3,
    SDL_SCANCODE_KP_4,
    SDL_SCANCODE_KP_5,
    SDL_SCANCODE_KP_6,
    SDL_SCANCODE_KP_7,
    SDL_SCANCODE_KP_8,
    SDL_SCANCODE_KP_9,
    SDL_SCANCODE_KP_DECIMAL,
    SDL_SCANCODE_KP_DIVIDE,
    SDL_SCANCODE_KP_MULTIPLY,
    SDL_SCANCODE_KP_MINUS,
    SDL_SCANCODE_KP_PLUS,
    SDL_SCANCODE_KP_ENTER,
    SDL_SCANCODE_KP_EQUALS,
    SDL_SCANCODE_LSHIFT,
    SDL_SCANCODE_LCTRL,
    SDL_SCANCODE_LALT,
    SDL_SCANCODE_LGUI,
    SDL_SCANCODE_RSHIFT,
    SDL_SCANCODE_RCTRL,
    SDL_SCANCODE_RALT,
    SDL_SCANCODE_RGUI,
    SDL_SCANCODE_MENU,
};

Input::Input() {
  for (int32_t i = 0; i < KEY_COUNT; i++) {
    key_state_[i] = 0;
  }
}

Input::~Input() {}

void Input::UpdateState(const WindowInfo* window_info, int32_t frame_count) {
  frame_count_ = frame_count + 1;  // change frame_count to start from 1

  uint32_t mouse_state = SDL_GetGlobalMouseState(&mouse_x_, &mouse_y_);

  mouse_x_ = (mouse_x_ - (window_info->window_x + window_info->screen_x)) /
             window_info->screen_scale;
  mouse_y_ = (mouse_y_ - (window_info->window_y + window_info->screen_y)) /
             window_info->screen_scale;

  const uint8_t* sdl_key_state = SDL_GetKeyboardState(NULL);

  for (int32_t i = 0; i < SDL_KEY_COUNT; i++) {
    UpdateKeyState(i, sdl_key_state[SCANCODE_TABLE[i]]);
  }

  UpdateKeyState(KEY_SHIFT, sdl_key_state[SCANCODE_TABLE[KEY_LEFT_SHIFT]] ||
                                sdl_key_state[SCANCODE_TABLE[KEY_RIGHT_SHIFT]]);

  UpdateKeyState(KEY_CONTROL,
                 sdl_key_state[SCANCODE_TABLE[KEY_LEFT_CONTROL]] ||
                     sdl_key_state[SCANCODE_TABLE[KEY_RIGHT_CONTROL]]);

  UpdateKeyState(KEY_ALT, sdl_key_state[SCANCODE_TABLE[KEY_LEFT_ALT]] ||
                              sdl_key_state[SCANCODE_TABLE[KEY_RIGHT_ALT]]);

  UpdateKeyState(KEY_SUPER, sdl_key_state[SCANCODE_TABLE[KEY_LEFT_SUPER]] ||
                                sdl_key_state[SCANCODE_TABLE[KEY_RIGHT_SUPER]]);
}

bool Input::IsButtonOn(int32_t key) const {
  if (key < 0 || key >= KEY_COUNT) {
    return false;
  }

  return key_state_[key] > 0;
}

bool Input::IsButtonPressed(int32_t key,
                            int32_t hold_frame,
                            int32_t period_frame) const {
  if (key < 0 || key >= KEY_COUNT) {
    return false;
  }

  if (key_state_[key] == frame_count_) {
    return true;
  }

  if (period_frame > 0 &&
      (frame_count_ - (key_state_[key] + hold_frame)) % period_frame == 0) {
    return true;
  }

  return false;
}

bool Input::IsButtonReleased(int32_t key) const {
  if (key < 0 || key >= KEY_COUNT) {
    return false;
  }

  return key_state_[key] == -frame_count_;
}

void Input::SetMouseVisibility(int32_t visible) {
  is_mouse_visible_ = visible;
}

}  // namespace pyxelcore
