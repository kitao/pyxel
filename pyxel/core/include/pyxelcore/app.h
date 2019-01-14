#ifndef PYXELCORE_APP_H_
#define PYXELCORE_APP_H_

class SDL_Window;
class SDL_Renderer;
class SDL_Texture; // temporary

namespace pyxelcore {

class App {
public:
  int Width_Getter() { return width_; }
  int Height_Getter() { return height_; }
  int FrameCount_Getter() { return frame_count_; }

  App(int width, int height, char *caption, int scale, int *palette, int fps,
      int border_width, int border_color);
  void Run(void (*update)(), void (*draw)());
  void Quit();

private:
  int width_;
  int height_;
  int frame_count_;

  SDL_Renderer *renderer_;
  SDL_Window *window_;
  SDL_Texture *temp_texture_;
};

} // namespace pyxelcore

#endif // PYXELCORE_APP_H_
