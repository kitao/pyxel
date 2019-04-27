#include "pyxelcore/input.h"

#include <SDL2/SDL.h>

namespace pyxelcore {

Input::Input() {}

Input::~Input() {}

void Input::UpdateState(int32_t frame_count) {
  uint32_t mouse_state = SDL_GetMouseState(&mouse_x_, &mouse_y_);
}

bool Input::IsButtonOn(int32_t key) const {
  return false;
}

bool Input::IsButtonPressed(int32_t key,
                            int32_t hold_frame,
                            int32_t period_frame) const {
  return false;
}

bool Input::IsButtonReleased(int32_t key) const {
  return false;
}

void Input::SetMouseVisibility(int32_t visible) {
  //
}

}  // namespace pyxelcore
