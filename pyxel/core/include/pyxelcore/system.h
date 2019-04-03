#ifndef PYXELCORE_SYSTEM_H_
#define PYXELCORE_SYSTEM_H_

#include <cstdint>
#include <string>

#include "pyxelcore/constants.h"
#include "pyxelcore/image.h"

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

class System {
  //
  // System
  //
 public:
  System(Image* screen,
         const char* caption = NULL,
         int32_t scale = -1,
         const int32_t* palette_color = NULL,
         int32_t fps = -1,
         int32_t border_width = -1,
         int32_t border_color = -1);
  ~System();

  int32_t Width() { return screen_->Width(); }
  int32_t Height() { return screen_->Height(); }
  int32_t FrameCount() { return frame_count_; }

  void Run(void (*update)(), void (*draw)());
  void Quit();

 private:
  Image* screen_;
  std::string caption_;
  int32_t scale_;
  int32_t palette_color_[COLOR_COUNT];
  int32_t fps_;
  int32_t border_width_;
  int32_t border_color_;
  int32_t frame_count_;

  SDL_Renderer* renderer_;
  SDL_Window* window_;
  SDL_Texture* screen_texture_;

  void UpdateScreenTexture();
};

}  // namespace pyxelcore

#endif  // PYXELCORE_SYSTEM_H_
