#include "pyxelcore/input.h"

namespace pyxelcore {

Input::Input() {}

Input::~Input() {}

bool Input::IsButtonOn(int32_t key) {
  return false;
}

bool Input::IsButtonPressed(int32_t key,
                            int32_t hold_frame,
                            int32_t period_frame) {
  return false;
}

bool Input::IsButtonReleased(int32_t key) {
  return false;
}

void Input::SetMouseVisibility(int32_t visible) {
  //
}

}  // namespace pyxelcore
