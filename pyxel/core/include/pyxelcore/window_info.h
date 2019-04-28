#ifndef PYXELCORE_WINDOW_INFO_H_
#define PYXELCORE_WINDOW_INFO_H_

#include <cstdint>

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

struct WindowInfo {
  int32_t window_x;
  int32_t window_y;
  int32_t window_width;
  int32_t window_height;

  int32_t screen_x;
  int32_t screen_y;
  int32_t screen_width;
  int32_t screen_height;
  int32_t screen_scale;

  SDL_Window* window;
  SDL_Renderer* renderer;
  SDL_Texture* screen_texture;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_WINDOW_INFO_H_
