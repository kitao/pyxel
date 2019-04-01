#include "pyxelcore/image.h"

#include "pyxelcore/constants.h"
#include "pyxelcore/tilemap.h"

#include <algorithm>

namespace pyxelcore {

Image::Image(int32_t width, int32_t height, int32_t* data) {
  width_ = width;
  height_ = height;

  if (data) {
    data_ = data;
    need_to_delete_data_ = false;
  } else {
    data_ = new int32_t[width * height];
    need_to_delete_data_ = true;
  }
}

Image::~Image() {
  if (need_to_delete_data_) {
    delete[] data_;
  }
}

int32_t Image::GetColor(int32_t x, int32_t y) {
  if (x < 0 || y < 0 || x >= width_ || y >= height_) {
    // error
  }

  return data_[width_ * y + x];
}

void Image::SetColor(int32_t x, int32_t y, int32_t color) {
  //
}

void Image::SetData(int32_t x,
                    int32_t y,
                    const int32_t* data,
                    int32_t data_width,
                    int32_t data_height) {
  //
}

void Image::LoadImage(int32_t x, int32_t y, const char* filename) {
  //
}

void Image::CopyImage(int32_t x,
                      int32_t y,
                      Image* image,
                      int32_t u,
                      int32_t v,
                      int32_t w,
                      int32_t h) {
  //
}

}  // namespace pyxelcore
