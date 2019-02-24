#ifndef PYXELCORE_APP_H_
#define PYXELCORE_APP_H_

#include <string>

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

class App {
public:
  App(int width, int height, char *caption, int scale, int *palette, int fps,
      int border_width, int border_color);
  ~App();

  //
  // System
  //
public:
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

  SDL_Renderer *renderer_;
  SDL_Window *window_;
  SDL_Texture *temp_texture_;
  SDL_Texture *screen_texture_;

  void InitializeSystem();
  void TerminateSystem();
  void UpdateScreenTexture();

  //
  // Resource
  //
public:
  void save(char *filename);
  void load(char *filename);

private:
  void InitializeResource();
  void TerminateResource();

  //
  // Input
  //
public:
  int mouse_x_getter() { return mouse_x_; }
  int mouse_y_getter() { return mouse_y_; }

  int btn(int key);
  int btnp(int key, int hold, int period);
  int btnr(int key);
  void mouse(int visible);

private:
  int mouse_x_;
  int mouse_y_;

  void InitializeInput();
  void TerminateInput();

  //
  // Graphics
  //
public:
  void *image(int img, int system);
  void *tilemap(int tm);
  void clip(int x1, int y1, int x2, int y2);
  void pal();
  void pal(int col1, int col2);
  void cls(int col);
  void pix(int x, int y, int col);
  void line(int x1, int y1, int x2, int y2, int col);
  void rect(int x1, int y1, int x2, int y2, int col);
  void rectb(int x1, int y1, int x2, int y2, int col);
  void circ(int x, int y, int r, int col);
  void circb(int x, int y, int r, int col);
  void blt(int x, int y, int img, int u, int v, int w, int h, int colkey);
  void bltm(int x, int y, int tm, int u, int v, int w, int h, int colkey);
  void text(int x, int y, int s, int col);

private:
  int *framebuffer_;

  int clip_x1_;
  int clip_y1_;
  int clip_x2_;
  int clip_y2_;

  int pal_[16];

  void InitializeGraphics();
  void TerminateGraphics();

  //
  // Audio
  //
public:
  void *sound(int snd, int system);
  void *music(int msc);
  void play(int ch, int snd, int loop);
  void playm(int msc, int loop);
  void stop(int ch);

private:
  void InitializeAudio();
  void TerminateAudio();
};

} // namespace pyxelcore

#endif // PYXELCORE_APP_H_
