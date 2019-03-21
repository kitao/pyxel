#ifndef PYXELCORE_GRAPHICS_H_
#define PYXELCORE_GRAPHICS_H_

#include <cstdint>

namespace pyxelcore {

class Image;

class Graphics {
 public:
  Graphics(int32_t width, int32_t height);
  ~Graphics();

  int* Framebuffer() { return framebuffer_; }

  void* image(int32_t img, int32_t system);
  void* tilemap(int32_t tm);
  void clip();
  void clip(int32_t x1, int32_t y1, int32_t x2, int32_t y2);
  void pal();
  void pal(int32_t col1, int32_t col2);
  void cls(int32_t col);
  void pix(int32_t x, int32_t y, int32_t col);
  void line(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col);
  void rect(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col);
  void rectb(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col);
  void circ(int32_t x, int32_t y, int32_t r, int32_t col);
  void circb(int32_t x, int32_t y, int32_t r, int32_t col);
  void blt(int32_t x,
           int32_t y,
           int32_t img,
           int32_t u,
           int32_t v,
           int32_t w,
           int32_t h,
           int32_t colkey);
  void bltm(int32_t x,
            int32_t y,
            int32_t tm,
            int32_t u,
            int32_t v,
            int32_t w,
            int32_t h,
            int32_t colkey);
  void text(int32_t x, int32_t y, int32_t s, int32_t col);

 private:
  int32_t width_;
  int32_t height_;

  int32_t* framebuffer_;

  int32_t clip_x1_;
  int32_t clip_y1_;
  int32_t clip_x2_;
  int32_t clip_y2_;

  int32_t pal_[16];

  Image* image_[4];
};

}  // namespace pyxelcore

#endif  // PYXELCORE_GRAPHICS_H_
