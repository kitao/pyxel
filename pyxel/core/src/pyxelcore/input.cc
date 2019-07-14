#include "pyxelcore/input.h"

#include "pyxelcore/window.h"

namespace pyxelcore {

Input::Input() {
  gamepad1_ = SDL_GameControllerOpen(0);
  gamepad2_ = SDL_GameControllerOpen(1);

  is_mouse_visible_ = false;

  for (int32_t i = 0; i < KEY_COUNT; i++) {
    key_state_[i] = 0;
  }
}

Input::~Input() {
  if (gamepad1_) {
    SDL_GameControllerClose(gamepad1_);
  }

  if (gamepad2_) {
    SDL_GameControllerClose(gamepad2_);
  }
}

void Input::Update(const Window* window, int32_t frame_count) {
  frame_count_ = frame_count + 1;  // change frame_count to start from 1

  SDL_GetGlobalMouseState(&mouse_x_, &mouse_y_);

  mouse_x_ = (mouse_x_ - (window->WindowX() + window->ScreenX())) /
             window->ScreenScale();
  mouse_y_ = (mouse_y_ - (window->WindowY() + window->ScreenY())) /
             window->ScreenScale();

  SDL_ShowCursor(mouse_x_ < 0 || mouse_x_ >= window->ScreenWidth() ||
                 mouse_y_ < 0 || mouse_y_ >= window->ScreenHeight());

  const uint8_t* sdl_key_state = SDL_GetKeyboardState(NULL);

  for (int32_t i = 0; i < SDL_KEY_COUNT; i++) {
    UpdateKeyState(i, sdl_key_state[SDL_KEY_TABLE[i]]);
  }

  UpdateKeyState(KEY_SHIFT, sdl_key_state[SDL_KEY_TABLE[KEY_LEFT_SHIFT]] ||
                                sdl_key_state[SDL_KEY_TABLE[KEY_RIGHT_SHIFT]]);

  UpdateKeyState(KEY_CONTROL,
                 sdl_key_state[SDL_KEY_TABLE[KEY_LEFT_CONTROL]] ||
                     sdl_key_state[SDL_KEY_TABLE[KEY_RIGHT_CONTROL]]);

  UpdateKeyState(KEY_ALT, sdl_key_state[SDL_KEY_TABLE[KEY_LEFT_ALT]] ||
                              sdl_key_state[SDL_KEY_TABLE[KEY_RIGHT_ALT]]);

  UpdateKeyState(KEY_SUPER, sdl_key_state[SDL_KEY_TABLE[KEY_LEFT_SUPER]] ||
                                sdl_key_state[SDL_KEY_TABLE[KEY_RIGHT_SUPER]]);

  uint32_t mouse_state = SDL_GetMouseState(NULL, NULL);

  UpdateKeyState(MOUSE_LEFT_BUTTON, mouse_state & SDL_BUTTON_LMASK);
  UpdateKeyState(MOUSE_MIDDLE_BUTTON, mouse_state & SDL_BUTTON_MMASK);
  UpdateKeyState(MOUSE_RIGHT_BUTTON, mouse_state & SDL_BUTTON_RMASK);

  if (gamepad1_) {
    for (int32_t i = 0; i < SDL_BUTTON_COUNT; i++) {
      bool button_state =
          SDL_GameControllerGetButton(gamepad1_, SDL_BUTTON_TABLE[i]);
      UpdateKeyState(GAMEPAD_1_A + i, button_state);
    }
  }

  if (gamepad2_) {
    for (int32_t i = 0; i < SDL_BUTTON_COUNT; i++) {
      bool button_state =
          SDL_GameControllerGetButton(gamepad2_, SDL_BUTTON_TABLE[i]);
      UpdateKeyState(GAMEPAD_2_A + i, button_state);
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

  return key_state_[key] == -frame_count_;
}

void Input::SetMouseVisible(int32_t is_visible) {
  is_mouse_visible_ = is_visible;
}

}  // namespace pyxelcore
