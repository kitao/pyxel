#ifndef PYXELCORE_APP_H_
#define PYXELCORE_APP_H_

#include <string>

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

class Graphics;
class Audio;

class App {
public:
  App(int width, int height, char *caption, int scale, int *palette, int fps,
      int border_width, int border_color);
  ~App();

  //
  // System
  //
  int width_getter() { return width_; }
  int height_getter() { return height_; }
  int frame_count_getter() { return frame_count_; }

  void run(void (*update)(), void (*draw)());
  void quit();

  //
  // Resource
  //
  void save(char *filename);
  void load(char *filename);

  //
  // Input
  //
  int mouse_x_getter() { return mouse_x_; }
  int mouse_y_getter() { return mouse_y_; }

  int btn(int key);
  int btnp(int key, int hold, int period);
  int btnr(int key);
  void mouse(int visible);

  //
  // Graphics
  //
  Graphics *Graphics() { return graphics_; }

  //
  // Audio
  //
  Audio *Audio() { return audio_; }

private:
  //
  // System
  //
  int width_;
  int height_;
  std::string caption_;
  int scale_;
  int palette_[16];
  int fps_;
  int border_width_;
  int border_color_;
  int frame_count_;

  SDL_Renderer *renderer_;
  SDL_Window *window_;
  // SDL_Texture *temp_texture_;
  SDL_Texture *screen_texture_;

  void InitializeSystem();
  void TerminateSystem();
  void UpdateScreenTexture();

  //
  // Resource
  //
  void InitializeResource();
  void TerminateResource();

  //
  // Input
  //
  int mouse_x_;
  int mouse_y_;

  void InitializeInput();
  void TerminateInput();

  //
  // Graphics
  //
  class Graphics *graphics_;

  //
  // Audio
  //
  class Audio *audio_;
};

} // namespace pyxelcore

#endif // PYXELCORE_APP_H_
