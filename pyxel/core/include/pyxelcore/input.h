#ifndef PYXELCORE_INPUT_H_
#define PYXELCORE_INPUT_H_

#include <cstdint>

namespace pyxelcore {

class Input {
 public:
  Input();
  ~Input();

  void UpdateState(int32_t frame_count);

  int32_t MouseX() const { return mouse_x_; }
  int32_t MouseY() const { return mouse_y_; }

  bool IsButtonOn(int32_t key) const;
  bool IsButtonPressed(int32_t key,
                       int32_t hold_frame = 0,
                       int32_t period_frame = 0) const;
  bool IsButtonReleased(int32_t key) const;
  void SetMouseVisibility(int32_t visible);

 private:
  int32_t mouse_x_;
  int32_t mouse_y_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_INPUT_H_
