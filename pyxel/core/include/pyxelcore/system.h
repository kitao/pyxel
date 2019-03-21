#ifndef PYXELCORE_SYSTEM_H_
#define PYXELCORE_SYSTEM_H_

#include <cstdint>
#include <string>

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

class Graphics;

class System {
 public:
  System(Graphics* graphics,
         int32_t width,
         int32_t height,
         const char* caption,
         int32_t scale,
         const int* palette,
         int32_t fps,
         int32_t border_width,
         int32_t border_color);
  ~System();

  int32_t Width() { return width_; }
  int32_t Height() { return height_; }
  int32_t FrameCount() { return frame_count_; }

  void Run(void (*update)(), void (*draw)());
  void Quit();

 private:
  int32_t width_;
  int32_t height_;
  std::string caption_;
  int32_t scale_;
  int32_t palette_[16];
  int32_t fps_;
  int32_t border_width_;
  int32_t border_color_;
  int32_t frame_count_;

  Graphics* graphics_;

  SDL_Renderer* renderer_;
  SDL_Window* window_;
  // SDL_Texture *temp_texture_;
  SDL_Texture* screen_texture_;

  void UpdateScreenTexture();
};

}  // namespace pyxelcore

#endif  // PYXELCORE_SYSTEM_H_
