#include "pyxelcore/tilemap.h"

namespace pyxelcore {

Tilemap::Tilemap(int32_t width, int32_t height) {
  if (width < 1 || height < 1) {
    PRINT_ERROR("invalid tilemap size");
    width = Max(width, 1);
    height = Max(height, 1);
  }

  width_ = width;
  height_ = height;
  rect_ = Rectangle::FromSize(0, 0, width, height);
  data_ = new int32_t[width * height]();
  image_index_ = 0;
}

Tilemap::~Tilemap() {
  delete data_;
}

int32_t Tilemap::GetValue(int32_t x, int32_t y) const {
  if (!rect_.Includes(x, y)) {
    PRINT_ERROR("access to outside tilemap");
    return 0;
  }

  return data_[width_ * y + x];
}

void Tilemap::SetValue(int32_t x, int32_t y, int32_t value) {
  if (!rect_.Includes(x, y)) {
    return;
  }

  if (value < 0 || value >= TILEMAP_CHIP_COUNT) {
    PRINT_ERROR("invalid value");
  }

  data_[width_ * y + x] = value;
}

void Tilemap::SetValue(int32_t x,
                       int32_t y,
                       const char** value,
                       int32_t value_length) {
  int32_t width = strlen(value[0]) / 3;
  int32_t height = value_length;

  if (width < 1 || height < 1) {
    PRINT_ERROR("invalid value size");
    return;
  }

  Tilemap tilemap = Tilemap(width, height);
  int32_t* data = tilemap.data_;

  for (int32_t i = 0; i < height; i++) {
    int32_t index = width * i;
    std::string str = value[i];

    for (int32_t j = 0; j < width; j++) {
      int32_t value = std::stoi(str.substr(j * 3, 3), nullptr, 16);

      if (value < 0 || value >= TILEMAP_CHIP_COUNT) {
        PRINT_ERROR("invalid value");
        value = 0;
      }

      data[index + j] = value;
    }
  }

  CopyTilemap(x, y, &tilemap, 0, 0, width, height);
}

void Tilemap::CopyTilemap(int32_t x,
                          int32_t y,
                          const Tilemap* tilemap,
                          int32_t u,
                          int32_t v,
                          int32_t width,
                          int32_t height) {
  Rectangle::CopyArea copy_area =
      rect_.GetCopyArea(x, y, tilemap->Rectangle(), u, v, width, height);

  if (copy_area.IsEmpty()) {
    return;
  }

  int32_t src_width = tilemap->width_;
  int32_t* src_data = tilemap->data_;

  int32_t dst_width = width_;
  int32_t* dst_data = data_;

  for (int32_t i = 0; i < copy_area.height; i++) {
    int32_t src_index = src_width * (copy_area.v + i) + copy_area.u;
    int32_t dst_index = dst_width * (copy_area.y + i) + copy_area.x;

    for (int32_t j = 0; j < copy_area.width; j++) {
      dst_data[dst_index + j] = src_data[src_index + j];
    }
  }
}

}  // namespace pyxelcore
