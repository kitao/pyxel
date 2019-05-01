#include "pyxelcore/tilemap.h"

namespace pyxelcore {

Tilemap::Tilemap(int32_t width, int32_t height) {
  if (width < 1 || height < 1) {
    PRINT_ERROR("invalide tilemap size");
    width = Max(width, 1);
    height = Max(height, 1);
  }

  rect_ = Rectangle::FromSize(0, 0, width, height);
}

Tilemap::~Tilemap() {}

int32_t Tilemap::GetValue(int32_t x, int32_t y) const {
  if (!rect_.Includes(x, y)) {
    PRINT_ERROR("access to outside tilemap");
    return 0;
  }

  return data_[Width() * y + x];
}

void Tilemap::SetValue(int32_t x, int32_t y, int32_t value) {
  if (!rect_.Includes(x, y)) {
    PRINT_ERROR("access to outside image");
    return;
  }

  data_[Width() * y + x] = value;
}

void Tilemap::SetValue(int32_t x,
                       int32_t y,
                       const char** value_str,
                       int32_t value_str_count) {
  //
}

void Tilemap::CopyTilemap(int32_t x,
                          int32_t y,
                          const Tilemap* tilemap,
                          int32_t u,
                          int32_t v,
                          int32_t width,
                          int32_t height) {
  //
}

}  // namespace pyxelcore
