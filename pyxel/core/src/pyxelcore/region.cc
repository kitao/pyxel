#include "pyxelcore/region.h"

#include <algorithm>

namespace pyxelcore {

const Region Region::ZERO = Region(0, 0, 0, 0, 0, 0);

Region::Region(int32_t left,
               int32_t top,
               int32_t right,
               int32_t bottom,
               int32_t width,
               int32_t height) {
  left_ = left;
  top_ = top;
  right_ = right;
  bottom_ = bottom;
  width_ = width;
  height_ = height;
}

Region Region::FromPos(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  int32_t left = std::min(x1, x2);
  int32_t top = std::min(y1, y2);
  int32_t right = std::max(x1, x2);
  int32_t bottom = std::max(y1, y2);
  int32_t width = right - left + 1;
  int32_t height = bottom - top + 1;

  return Region(left, top, right, bottom, width, height);
}

Region Region::FromSize(int32_t left,
                        int32_t top,
                        int32_t width,
                        int32_t height) {
  int32_t right = left + width - 1;
  int32_t bottom = top + height - 1;

  return Region(left, top, right, bottom, width, height);
}

bool Region::operator==(const Region& region) const {
  return left_ == region.left_ && top_ == region.top_ &&
         right_ == region.right_ && bottom_ == region.bottom_;
}

Region Region::operator&(const Region& region) const {
  int32_t left = std::max(left_, region.left_);
  int32_t top = std::max(top_, region.top_);
  int32_t right = std::min(right_, region.right_);
  int32_t bottom = std::min(bottom_, region.bottom_);
  int32_t width = right - left + 1;
  int32_t height = bottom - top + 1;

  if (width > 0 && height > 0) {
    return Region(left, top, right, bottom, width, height);
  } else {
    return Region::ZERO;
  }
}

Region Region::MoveTo(int32_t x, int32_t y) const {
  int32_t right = x + width_ - 1;
  int32_t bottom = y + height_ - 1;

  return Region(x, y, right, bottom, width_, height_);
}

bool Region::Includes(int32_t x, int32_t y) const {
  return x >= left_ && x <= right_ && y >= top_ && y <= bottom_;
}

}  // namespace pyxelcore