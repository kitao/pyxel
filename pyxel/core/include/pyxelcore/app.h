#ifndef PYXELCORE_APP_H_
#define PYXELCORE_APP_H_

class SDL_Window;
class SDL_Renderer;
class SDL_Texture; // temporary

namespace pyxelcore {

class App {
public:
  //
  // System
  //
  int GetWidth() { return width_; }
  int GetHeight() { return height_; }
  int GetFrameCount() { return frame_count_; }

  App(int width, int height, char *caption, int scale, int *palette, int fps,
      int border_width, int border_color);
  void Run(void (*update)(), void (*draw)());
  void Quit();

  //
  // Resource
  //
  void Save(char *filename);
  void Load(char *filename);

  //
  // Input
  //
  int GetMouseX();
  int GetMouseY();

  int Btn(int key);
  int Btnp(int key, int hold, int period);
  int Btnr(int key);
  void Mouse(int visible);

  //
  // Graphics
  //
  void *Image(int img, int system);
  void *Tilemap(int tm);
  void Clip(int x1, int y1, int x2, int y2);
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

  //
  // Audio
  //
  void *Sound(int snd, int system);
  void *Music(int msc);
  void Play(int ch, int snd, int loop);
  void Playm(int msc, int loop);
  void Stop(int ch);

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
