#ifndef PYXELCORE_INPUT_H_
#define PYXELCORE_INPUT_H_

#include <cstdint>

namespace pyxelcore {

class Input {
 public:
  Input();
  ~Input();

  int32_t MouseX() { return mouse_x_; }
  int32_t MouseY() { return mouse_y_; }

  bool Btn(int32_t key);
  bool Btnp(int32_t key, int32_t hold = 0, int32_t period = 0);
  bool Btnr(int32_t key);
  void Mouse(int32_t visible);

 private:
  int32_t mouse_x_;
  int32_t mouse_y_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_INPUT_H_
