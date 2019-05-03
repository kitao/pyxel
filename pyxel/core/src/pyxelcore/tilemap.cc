#include "pyxelcore/tilemap.h"

#include <string>

namespace pyxelcore {

Tilemap::Tilemap(int32_t width, int32_t height) {
  if (width < 1 || height < 1) {
    PRINT_ERROR("invalid tilemap size");
    width = Max(width, 1);
    height = Max(height, 1);
  }

  rect_ = Rectangle::FromSize(0, 0, width, height);
  data_ = new int32_t[width * height];
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

  return data_[Width() * y + x];
}

void Tilemap::SetValue(int32_t x, int32_t y, int32_t value) {
  if (!rect_.Includes(x, y)) {
    return;
  }

  if (value < 0 || value >= TILEMAP_CHIP_COUNT) {
    PRINT_ERROR("invalid value");
  }

  data_[Width() * y + x] = value;
}

void Tilemap::SetData(int32_t x,
                      int32_t y,
                      const char** data,
                      int32_t data_count) {
  int32_t width = strlen(data[0]) / 3;
  int32_t height = data_count;
  Tilemap* tilemap = new Tilemap(width, height);
  int32_t* dst_data = tilemap->data_;

  for (int32_t i = 0; i < height; i++) {
    int32_t index = width * i;
    std::string str = data[i];

    for (int32_t j = 0; j < width; j++) {
      int32_t value = std::stoi(str.substr(j * 3, 3), nullptr, 16);

      if (value < 0 || value >= TILEMAP_CHIP_COUNT) {
        PRINT_ERROR("invalid value");
        value = 0;
      }

      dst_data[index + j] = value;
    }
  }

  CopyTilemap(x, y, tilemap, 0, 0, width, height);

  delete tilemap;
}

void Tilemap::CopyTilemap(int32_t x,
                          int32_t y,
                          const Tilemap* tilemap,
                          int32_t u,
                          int32_t v,
                          int32_t width,
                          int32_t height) {
  Rectangle::CopyArea copy_area = rect_.GetCopyArea(
      x, y, tilemap->rect_, Rectangle::FromSize(u, v, width, height));

  int32_t copy_w = copy_area.copy_w;
  int32_t copy_h = copy_area.copy_h;

  if (copy_w <= 0 || copy_h <= 0) {
    return;
  }

  int32_t src_width = tilemap->Width();
  int32_t* src_data = tilemap->data_;

  int32_t dst_width = Width();
  int32_t* dst_data = data_;

  for (int32_t i = 0; i < copy_h; i++) {
    int32_t src_index = src_width * (copy_area.src_y + i) + copy_area.src_x;
    int32_t dst_index = dst_width * (copy_area.dst_y + i) + copy_area.dst_x;

    for (int32_t j = 0; j < copy_w; j++) {
      dst_data[dst_index + j] = src_data[src_index + j];
    }
  }
}

}  // namespace pyxelcore
