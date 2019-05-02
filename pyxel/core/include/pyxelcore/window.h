#ifndef PYXELCORE_WINDOW_H_
#define PYXELCORE_WINDOW_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Window {
 public:
  Window(const char* caption,
         int32_t screen_width,
         int32_t screen_height,
         int32_t screen_scale,
         int32_t border_width,
         int32_t border_color);

  int32_t WindowX() const { return window_x_; }
  int32_t WindowY() const { return window_y_; }
  int32_t ScreenX() const { return screen_x_; }
  int32_t ScreenY() const { return screen_y_; }
  int32_t ScreenWidth() const { return screen_width_; }
  int32_t ScreenHeight() const { return screen_height_; }
  int32_t ScreenScale() const { return screen_scale_; }

  void ToggleFullscreen();
  bool ProcessEvents();
  void Render(const int32_t* screen_data, const int32_t* palette_color);

 private:
  SDL_Window* window_;
  SDL_Renderer* renderer_;
  SDL_Texture* screen_texture_;

  int32_t window_x_;
  int32_t window_y_;
  int32_t screen_x_;
  int32_t screen_y_;
  int32_t screen_width_;
  int32_t screen_height_;
  int32_t screen_scale_;
  int32_t border_color_;
  bool is_fullscreen_;

  void UpdateWindowInfo();
  void UpdateScreenTexture(const int32_t* screen_data,
                           const int32_t* palette_color);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_WINDOW_H_
