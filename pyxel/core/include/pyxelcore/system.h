#ifndef PYXELCORE_SYSTEM_H_
#define PYXELCORE_SYSTEM_H_

#include <string>

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

class Graphics;

class System {
public:
  System(Graphics *graphics, int width, int height, char *caption, int scale,
         int *palette, int fps, int border_width, int border_color);
  ~System();

  int width_getter() { return width_; }
  int height_getter() { return height_; }
  int frame_count_getter() { return frame_count_; }

  void run(void (*update)(), void (*draw)());
  void quit();

private:
  int width_;
  int height_;
  std::string caption_;
  int scale_;
  int palette_[16];
  int fps_;
  int border_width_;
  int border_color_;
  int frame_count_;

  Graphics *graphics_;

  SDL_Renderer *renderer_;
  SDL_Window *window_;
  // SDL_Texture *temp_texture_;
  SDL_Texture *screen_texture_;

  void UpdateScreenTexture();
};

} // namespace pyxelcore

#endif // PYXELCORE_SYSTEM_H_
