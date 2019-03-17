#include "pyxelcore/input.h"

namespace pyxelcore {

Input::Input() {}

Input::~Input() {}

int Input::btn(int key) {
  return 0;
}

int Input::btnp(int key, int hold, int period) {
  return 0;
}

int Input::btnr(int key) {
  return 0;
}

void Input::mouse(int visible) {}

}  // namespace pyxelcore
