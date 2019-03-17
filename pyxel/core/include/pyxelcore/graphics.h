#ifndef PYXELCORE_GRAPHICS_H_
#define PYXELCORE_GRAPHICS_H_

namespace pyxelcore {

class Image;

class Graphics {
public:
  Graphics(int width, int height);
  ~Graphics();

  int *Framebuffer() { return framebuffer_; }

  void *image(int img, int system);
  void *tilemap(int tm);
  void clip();
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
  int width_;
  int height_;

  int *framebuffer_;

  int clip_x1_;
  int clip_y1_;
  int clip_x2_;
  int clip_y2_;

  int pal_[16];

  Image *image_[4];
};

} // namespace pyxelcore

#endif // PYXELCORE_GRAPHICS_H_
