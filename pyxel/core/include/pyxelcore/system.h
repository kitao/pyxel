#ifndef PYXELCORE_SYSTEM_H_
#define PYXELCORE_SYSTEM_H_

#include <string>

#include "pyxelcore/graphics.h"

class SDL_Window;
class SDL_Renderer;
class SDL_Texture; // temporary

namespace pyxelcore {

class System {
public:
  System(Graphics *graphics, int width, int height, char *caption, int scale,
         int *palette, int fps, int border_width, int border_color);
  ~System();

  int Width() { return width_; }
  int Height() { return height_; }
  int FrameCount() { return frame_count_; }

  void Run(void (*update)(), void (*draw)());
  void Quit();

private:
  int width_;
  int height_;
  std::string caption_;
  int scale;
  int palette_[16];
  int fps_;
  int border_width_;
  int border_color_;

  int frame_count_;

  SDL_Renderer *renderer_;
  SDL_Window *window_;
  SDL_Texture *temp_texture_;

  SDL_Texture *screen_texture_;

  Graphics *graphics_;

  void UpdateScreenTexture();
};

} // namespace pyxelcore

#endif // PYXELCORE_SYSTEM_H_
