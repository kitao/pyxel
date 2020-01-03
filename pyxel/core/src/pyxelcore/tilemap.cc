#include "pyxelcore/tilemap.h"

namespace pyxelcore {

Tilemap::Tilemap(int32_t width, int32_t height) {
  if (width < 1 || height < 1) {
    PYXEL_ERROR("invalid tilemap size");
  }

  width_ = width;
  height_ = height;
  rect_ = pyxelcore::Rectangle(0, 0, width, height);
  image_index_ = 0;

  data_ = new int32_t*[height];
  data_[0] = new int32_t[width * height]();
  for (int32_t i = 1; i < height; i++) {
    data_[i] = data_[0] + width * i;
  }
}

Tilemap::~Tilemap() {
  delete data_[0];
  delete data_;
}

int32_t Tilemap::GetValue(int32_t x, int32_t y) const {
  if (!rect_.Includes(x, y)) {
    PYXEL_ERROR("access to outside tilemap");
  }

  return data_[y][x];
}

void Tilemap::SetValue(int32_t x, int32_t y, int32_t value) {
  if (!rect_.Includes(x, y)) {
    return;
  }

  if (value < 0 || value >= TILEMAP_CHIP_COUNT) {
    PYXEL_ERROR("invalid value");
  }

  data_[y][x] = value;
}

void Tilemap::SetData(int32_t x,
                      int32_t y,
                      const TilemapString& tilemap_string) {
  int32_t width = tilemap_string[0].size() / 3;
  int32_t height = tilemap_string.size();

  if (width < 1 || height < 1) {
    PYXEL_ERROR("invalid value size");
  }

  Tilemap tilemap = Tilemap(width, height);
  int32_t** dst_data = tilemap.data_;

  for (int32_t i = 0; i < height; i++) {
    std::string str = tilemap_string[i];
    int32_t* dst_line = dst_data[i];

    for (int32_t j = 0; j < width; j++) {
      int32_t value = std::stoi(str.substr(j * 3, 3), nullptr, 16);

      if (value < 0 || value >= TILEMAP_CHIP_COUNT) {
        PYXEL_ERROR("invalid value");
      }

      dst_line[j] = value;
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

  int32_t** src_data = tilemap->data_;
  int32_t** dst_data = data_;

  for (int32_t i = 0; i < copy_area.height; i++) {
    int32_t* src_line = src_data[copy_area.v + i];
    int32_t* dst_line = dst_data[copy_area.y + i];

    for (int32_t j = 0; j < copy_area.width; j++) {
      dst_line[copy_area.x + j] = src_line[copy_area.u + j];
    }
  }
}

}  // namespace pyxelcore
