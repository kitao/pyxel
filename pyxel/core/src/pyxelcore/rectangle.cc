#include "pyxelcore/rectangle.h"

#include <algorithm>

namespace pyxelcore {

Rectangle::Rectangle()
    : left_(0), top_(0), right_(0), bottom_(0), width_(0), height_(0) {}

Rectangle::Rectangle(int32_t left, int32_t top, int32_t width, int32_t height) {
  left_ = left;
  top_ = top;
  width_ = std::max(width, 0);
  height_ = std::max(height, 0);
  right_ = left_ + width_ - 1;
  bottom_ = top_ + height_ - 1;
}

Rectangle Rectangle::FromPos(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  int32_t left = std::min(x1, x2);
  int32_t top = std::min(y1, y2);
  int32_t width = std::abs(x1 - x2);
  int32_t height = std::abs(y1 - y2);

  return Rectangle(left, top, width, height);
}

Rectangle Rectangle::FromSize(int32_t left,
                              int32_t top,
                              int32_t width,
                              int32_t height) {
  return Rectangle(left, top, width, height);
}

bool Rectangle::IsEmpty() const {
  return width_ == 0 && height_ == 0;
}

bool Rectangle::Includes(int32_t x, int32_t y) const {
  return x >= left_ && x <= right_ && y >= top_ && y <= bottom_;
}

Rectangle Rectangle::MoveTo(int32_t x, int32_t y) const {
  return Rectangle(x, y, width_, height_);
}

Rectangle Rectangle::Intersect(const Rectangle& rect) const {
  int32_t left = std::max(left_, rect.left_);
  int32_t top = std::max(top_, rect.top_);
  int32_t right = std::min(right_, rect.right_);
  int32_t bottom = std::min(bottom_, rect.bottom_);
  int32_t width = right - left + 1;
  int32_t height = bottom - top + 1;

  if (width > 0 && height > 0) {
    return Rectangle(left, top, width, height);
  } else {
    return Rectangle(0, 0, 0, 0);
  }
}

}  // namespace pyxelcore