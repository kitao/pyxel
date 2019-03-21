#ifndef PYXELCORE_INPUT_H_
#define PYXELCORE_INPUT_H_

#include <cstdint>

namespace pyxelcore {

class Input {
 public:
  Input();
  ~Input();

  int32_t mouse_x_getter() { return mouse_x_; }
  int32_t mouse_y_getter() { return mouse_y_; }

  int32_t btn(int32_t key);
  int32_t btnp(int32_t key, int32_t hold, int32_t period);
  int32_t btnr(int32_t key);
  void mouse(int32_t visible);

 private:
  int32_t mouse_x_;
  int32_t mouse_y_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_INPUT_H_
