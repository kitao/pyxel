#include "pyxelcore/tilemap.h"

namespace pyxelcore {

Tilemap::Tilemap(int32_t width, int32_t height)
    : Rectangle(0, 0, width, height) {
  //
}

Tilemap::~Tilemap() {
  //
}

int32_t Tilemap::GetValue(int32_t x, int32_t y) const {
  if (!Includes(x, y)) {
    return 0;
  }

  return data_[Width() * y + x];
}

}  // namespace pyxelcore
