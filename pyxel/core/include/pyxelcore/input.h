#ifndef PYXELCORE_INPUT_H_
#define PYXELCORE_INPUT_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Window;

class Input {
 public:
  Input();
  ~Input();

  int32_t MouseX() const { return mouse_x_; }
  int32_t MouseY() const { return mouse_y_; }
  int32_t MouseWheel() const { return mouse_wheel_; }

  bool IsButtonOn(int32_t key) const;
  bool IsButtonPressed(int32_t key,
                       int32_t hold_frame = 0,
                       int32_t period_frame = 0) const;
  bool IsButtonReleased(int32_t key) const;
  void SetMousePaused(int32_t is_paused);
  void SetMouseVisible(int32_t is_visible);

  bool IsMousePaused() const { return is_mouse_paused_; }
  bool IsMouseVisible() const { return is_mouse_visible_ || is_mouse_paused_; }

  void Update(Window* window, int32_t frame_count);

 private:
  SDL_GameController* gamepad1_;
  SDL_GameController* gamepad2_;
  SDL_Cursor* blank_cursor_;
  SDL_Cursor* normal_cursor_;

  int32_t frame_count_;
  int32_t mouse_x_;
  int32_t mouse_y_;
  int32_t mouse_wheel_;
  bool is_mouse_paused_;
  bool is_mouse_visible_;
  int32_t key_state_[KEY_COUNT];

  void UpdateKeyState(int32_t key, bool state);
};

inline void Input::UpdateKeyState(int32_t key, bool state) {
  if (state) {
    if (key_state_[key] <= 0) {
      key_state_[key] = frame_count_;
    }
  } else {
    if (key_state_[key] > 0) {
      key_state_[key] = -frame_count_;
    }
  }
}

}  // namespace pyxelcore

#endif  // PYXELCORE_INPUT_H_
