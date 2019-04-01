#include "pyxelcore/app.h"

namespace pyxelcore {
bool IsButtonReleased(int32_t key);
void SetMouseVisibility(int32_t visible);

bool App::IsButtonOn(int32_t key) {
  return false;
}

bool App::IsButtonPressed(int32_t key,
                          int32_t hold_frame,
                          int32_t period_frame) {
  return false;
}

bool App::IsButtonReleased(int32_t key) {
  return false;
}

void App::SetMouseVisibility(int32_t visible) {
  //
}

}  // namespace pyxelcore
