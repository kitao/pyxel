#ifndef PYXELCORE_GRAPHICS_H_
#define PYXELCORE_GRAPHICS_H_

namespace pyxelcore {

class Graphics {
public:
  Graphics(int width, int height);
  ~Graphics();

  int *GetFramebuffer() { return framebuffer_; }

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
  int width_;
  int height_;
  int *framebuffer_;

  int clip_x1_;
  int clip_y1_;
  int clip_x2_;
  int clip_y2_;

  int palette_[16];
}; // namespace pyxelcore

} // namespace pyxelcore

#endif // PYXELCORE_GRAPHICS_H_
