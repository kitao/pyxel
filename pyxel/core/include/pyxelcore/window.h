#ifndef PYXELCORE_WINDOW_H_
#define PYXELCORE_WINDOW_H_

#include "pyxelcore/common.h"

// Events on the window might need system-level actions to be performed
#define WINDOW_ACTION_NONE          0
#define WINDOW_ACTION_CLOSE         (1u << 0)  // close the window
#define WINDOW_ACTION_PAUSE_CURSOR  (1u << 1)  // window is inactive, so don't respond to cursor movements
#define WINDOW_ACTION_RESUME_CURSOR (1u << 2)  // start moving cursor again

namespace pyxelcore {

class Window {
 public:
  Window(const std::string& caption,
         int32_t screen_width,
         int32_t screen_height,
         int32_t screen_scale,
         const PaletteColor& palette_color);

  int32_t WindowX() const { return window_x_; }
  int32_t WindowY() const { return window_y_; }
  int32_t ScreenX() const { return screen_x_; }
  int32_t ScreenY() const { return screen_y_; }
  int32_t ScreenWidth() const { return screen_width_; }
  int32_t ScreenHeight() const { return screen_height_; }
  int32_t ScreenScale() const { return screen_scale_; }

  void ToggleFullscreen();
  uint32_t ProcessEvents();
  void Render(int32_t** screen_data);
  int32_t GetMouseWheel();
  std::string GetDropFile();
  void SetCaption(const std::string& caption);

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
  PaletteColor palette_color_;
  bool is_fullscreen_;
  int32_t mouse_wheel_;
  std::string drop_file_;

  uint32_t ProcessWindowEvent(SDL_Event event);
  void SetupWindowIcon() const;
  void UpdateWindowInfo();
  void UpdateScreenTexture(int32_t** screen_data);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_WINDOW_H_
