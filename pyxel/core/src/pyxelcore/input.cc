#include "pyxelcore/input.h"

#include "pyxelcore/window.h"

namespace pyxelcore {

Input::Input() {
  is_mouse_visible_ = false;

  for (int32_t i = 0; i < KEY_COUNT; i++) {
    key_state_[i] = 0;
  }
}

Input::~Input() {}

void Input::Update(const Window* window, int32_t frame_count) {
  frame_count_ = frame_count + 1;  // change frame_count to start from 1

  SDL_GetGlobalMouseState(&mouse_x_, &mouse_y_);

  mouse_x_ = (mouse_x_ - (window->WindowX() + window->ScreenX())) /
             window->ScreenScale();
  mouse_y_ = (mouse_y_ - (window->WindowY() + window->ScreenY())) /
             window->ScreenScale();

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

  uint32_t mouse_state = SDL_GetMouseState(NULL, NULL);

  UpdateKeyState(MOUSE_LEFT_BUTTON, mouse_state & SDL_BUTTON_LMASK);
  UpdateKeyState(MOUSE_MIDDLE_BUTTON, mouse_state & SDL_BUTTON_MMASK);
  UpdateKeyState(MOUSE_RIGHT_BUTTON, mouse_state & SDL_BUTTON_RMASK);
}

bool Input::IsButtonOn(int32_t key) const {
  if (key < 0 || key >= KEY_COUNT) {
    PRINT_ERROR("invalid key");
    return false;
  }

  return key_state_[key] > 0;
}

bool Input::IsButtonPressed(int32_t key,
                            int32_t hold_frame,
                            int32_t period_frame) const {
  if (key < 0 || key >= KEY_COUNT) {
    PRINT_ERROR("invalid key");
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
    PRINT_ERROR("invalid key");
    return false;
  }

  return key_state_[key] == -frame_count_;
}

void Input::SetMouseVisible(int32_t is_visible) {
  is_mouse_visible_ = is_visible;
}

/*
    def _update_gamepad(self):
        for i in range(2):
            if i == 0:
                states, count = glfw.get_joystick_buttons(glfw.JOYSTICK_1)
                offset = pyxel.GAMEPAD_1_A
            else:
                states, count = glfw.get_joystick_buttons(glfw.JOYSTICK_2)
                offset = pyxel.GAMEPAD_2_A

            for j in range(count):
                action = states[j]
                button = offset + j

                if action == glfw.PRESS:
                    self._key_state[button] = pyxel.frame_count
                elif action == glfw.RELEASE:
                    if self._key_state.get(button) == pyxel.frame_count:
                        self._key_state[button] = -pyxel.frame_count - 1
                    else:
                        self._key_state[button] = -pyxel.frame_count
*/

}  // namespace pyxelcore
