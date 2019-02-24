#include "pyxelcore/graphics.h"

namespace pyxelcore {

Graphics::Graphics(int width, int height) {
  width_ = width;
  height_ = height;
  framebuffer_ = new int[width * height];

  clip_x1_ = 0;
  clip_y1_ = 0;
  clip_x2_ = width - 1;
  clip_y2_ = height - 1;

  Cls(0);
  Pal();
}

Graphics::~Graphics() { delete framebuffer_; }

void Graphics::Clip(int x1, int y1, int x2, int y2) {
  clip_x1_ = x1;
  clip_y1_ = y1;
  clip_x2_ = x2;
  clip_y2_ = y2;
}

void Graphics::Pal() {
  for (int i; i < 16; i++) {
    palette_[i] = i;
  }
}

void Graphics::Pal(int col1, int col2) { palette_[col1] = col2; }

void Graphics::Cls(int col) {
  int size = width_ * height_;

  for (int i = 0; i < size; i++) {
    framebuffer_[i] = col;
  }
}

} // namespace pyxelcore
