#include "pyxelcore/image.h"

namespace pyxelcore {

Image::Image(int32_t width, int32_t height) {
  width_ = width;
  height_ = height;
  data_ = new int32_t[width * height];
}

Image::~Image() {
  delete[] data_;
}

int32_t Image::get(int32_t x, int32_t y) {
  return 0;
}

void Image::set(int32_t x, int32_t y, int32_t data) {}

void Image::set(int32_t x,
                int32_t y,
                const int32_t* data,
                int32_t data_width,
                int32_t data_height) {}

void Image::load(int32_t x, int32_t y, const char* filename) {}

void Image::copy(int32_t x,
                 int32_t y,
                 int32_t img,
                 int32_t u,
                 int32_t v,
                 int32_t w,
                 int32_t h) {}

}  // namespace pyxelcore
