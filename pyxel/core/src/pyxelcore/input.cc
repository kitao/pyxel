#include "pyxelcore/input.h"

#include "pyxelcore/window.h"

#define GET_KEY_STATE(key) \
  sdl_scancode_state[SDL_GetScancodeFromKey(SDL_KEYCODE_TABLE[key])]

namespace pyxelcore {

Input::Input() {
  gamepad1_ = SDL_GameControllerOpen(0);
  gamepad2_ = SDL_GameControllerOpen(1);

  const uint8_t data[] = {0};
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
  frame_count_ = frame_count + 1;  // change frame_count to start from 1

  SDL_GetGlobalMouseState(&mouse_x_, &mouse_y_);

  mouse_x_ = (mouse_x_ - (window->WindowX() + window->ScreenX())) /
             window->ScreenScale();
  mouse_y_ = (mouse_y_ - (window->WindowY() + window->ScreenY())) /
             window->ScreenScale();
  mouse_wheel_ = window->GetMouseWheel();

  if (is_mouse_visible_) {
    SDL_ShowCursor(true);
    SDL_SetCursor(mouse_x_ >= 0 && mouse_x_ < window->ScreenWidth() &&
                          mouse_y_ >= 0 && mouse_y_ < window->ScreenHeight()
                      ? blank_cursor_
                      : normal_cursor_);
  } else {
    SDL_ShowCursor(false);
  }

  const uint8_t* sdl_scancode_state = SDL_GetKeyboardState(NULL);

  for (int32_t i = 0; i < SDL_KEYCODE_COUNT; i++) {
    UpdateKeyState(i, GET_KEY_STATE(i));
  }

  UpdateKeyState(KEY_SHIFT, GET_KEY_STATE(KEY_LEFT_SHIFT) ||
                                GET_KEY_STATE(KEY_RIGHT_SHIFT));

  UpdateKeyState(KEY_CONTROL, GET_KEY_STATE(KEY_LEFT_CONTROL) ||
                                  GET_KEY_STATE(KEY_RIGHT_CONTROL));

  UpdateKeyState(KEY_ALT,
                 GET_KEY_STATE(KEY_LEFT_ALT) || GET_KEY_STATE(KEY_RIGHT_ALT));

  UpdateKeyState(KEY_SUPER, GET_KEY_STATE(KEY_LEFT_SUPER) ||
                                GET_KEY_STATE(KEY_RIGHT_SUPER));

  uint32_t mouse_state = SDL_GetMouseState(NULL, NULL);

  UpdateKeyState(MOUSE_LEFT_BUTTON, mouse_state & SDL_BUTTON_LMASK);
  UpdateKeyState(MOUSE_MIDDLE_BUTTON, mouse_state & SDL_BUTTON_MMASK);
  UpdateKeyState(MOUSE_RIGHT_BUTTON, mouse_state & SDL_BUTTON_RMASK);

  if (gamepad1_) {
    for (int32_t i = 0; i < BUTTON_COUNT; i++) {
      UpdateKeyState(GAMEPAD_1_A + i, SDL_GameControllerGetButton(
                                          gamepad1_, SDL_BUTTON_TABLE[i]));
    }
  }

  if (gamepad2_) {
    for (int32_t i = 0; i < BUTTON_COUNT; i++) {
      UpdateKeyState(GAMEPAD_2_A + i, SDL_GameControllerGetButton(
                                          gamepad2_, SDL_BUTTON_TABLE[i]));
    }
  }
}

bool Input::IsButtonOn(int32_t key) const {
  if (key < 0 || key >= KEY_COUNT) {
    PYXEL_ERROR("invalid key");
  }

  return key_state_[key] > 0;
}

bool Input::IsButtonPressed(int32_t key,
                            int32_t hold_frame,
                            int32_t period_frame) const {
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
}

bool Input::IsButtonReleased(int32_t key) const {
  if (key < 0 || key >= KEY_COUNT) {
    PYXEL_ERROR("invalid key");
  }

  if (frame_count_ == 0) {
    return false;
  }

  return key_state_[key] == -frame_count_;
}

void Input::SetMouseVisible(int32_t is_visible) {
  is_mouse_visible_ = is_visible;
}

}  // namespace pyxelcore
