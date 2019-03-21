#include "pyxelcore/image.h"

namespace pyxelcore {

Image::Image(int width, int height) {
  width_ = width;
  height_ = height;
  data_ = new int[width * height];
}

Image::~Image() {
  delete[] data_;
}

int Image::get(int x, int y) {
  return 0;
}

void Image::set(int x, int y, int data) {}

void Image::set(int x, int y, int* data, int data_width, int data_height) {}

void Image::load(int x, int y, char* filename) {}

void Image::copy(int x, int y, int img, int u, int v, int w, int h) {}

}  // namespace pyxelcore
