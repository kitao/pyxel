#include "pyxelcore/app.h"

namespace pyxelcore {

void App::InitializeGraphics() {
  framebuffer_ = new int[width_ * height_];

  clip_x1_ = 0;
  clip_y1_ = 0;
  clip_x2_ = width_ - 1;
  clip_y2_ = height_ - 1;

  cls(0);
  pal();
}

void App::TerminateGraphics() { delete framebuffer_; }

void App::clip(int x1, int y1, int x2, int y2) {
  clip_x1_ = x1;
  clip_y1_ = y1;
  clip_x2_ = x2;
  clip_y2_ = y2;
}

void App::pal() {
  for (int i; i < 16; i++) {
    palette_[i] = i;
  }
}

void App::pal(int col1, int col2) { palette_[col1] = col2; }

void App::cls(int col) {
  int size = width_ * height_;

  for (int i = 0; i < size; i++) {
    framebuffer_[i] = col;
  }
}

} // namespace pyxelcore
