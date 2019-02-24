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
  int Width() { return width_; }
  int Height() { return height_; }
  int FrameCount() { return frame_count_; }

  void Run(void (*update)(), void (*draw)());
  void Quit();

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
  void Save(char *filename);
  void Load(char *filename);

private:
  void InitializeResource();
  void TerminateResource();

  //
  // Input
  //
public:
  int MouseX();
  int MouseY();

  int Btn(int key);
  int Btnp(int key, int hold, int period);
  int Btnr(int key);
  void Mouse(int visible);

private:
  void InitializeInput();
  void TerminateInput();

  //
  // Graphics
  //
public:
  void *Image(int img, int system);
  void *Tilemap(int tm);
  void Clip(int x1, int y1, int x2, int y2);
  void Pal();
  void Pal(int col1, int col2);
  void Cls(int col);
  void Pix(int x, int y, int col);
  void Line(int x1, int y1, int x2, int y2, int col);
  void Rect(int x1, int y1, int x2, int y2, int col);
  void Rectb(int x1, int y1, int x2, int y2, int col);
  void Circ(int x, int y, int r, int col);
  void Circb(int x, int y, int r, int col);
  void Blt(int x, int y, int img, int u, int v, int w, int h, int colkey);
  void Bltm(int x, int y, int tm, int u, int v, int w, int h, int colkey);
  void Text(int x, int y, int s, int col);

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
  void *Sound(int snd, int system);
  void *Music(int msc);
  void Play(int ch, int snd, int loop);
  void Playm(int msc, int loop);
  void Stop(int ch);

private:
  void InitializeAudio();
  void TerminateAudio();
};

} // namespace pyxelcore

#endif // PYXELCORE_APP_H_
