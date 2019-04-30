#ifndef PYXELCORE_RECTANGLE_H_
#define PYXELCORE_RECTANGLE_H_

#include "pyxelcore/utilities.h"

namespace pyxelcore {

class Rectangle {
 public:
  Rectangle();

  int32_t Left() const { return left_; }
  int32_t Top() const { return top_; }
  int32_t Right() const { return right_; }
  int32_t Bottom() const { return bottom_; }
  int32_t Width() const { return width_; }
  int32_t Height() const { return height_; }

  bool IsEmpty() const;
  bool Includes(int32_t x, int32_t y) const;
  Rectangle MoveTo(int32_t x, int32_t y) const;
  Rectangle Intersect(const Rectangle& rect) const;

  static Rectangle FromPos(int32_t x1, int32_t y1, int32_t x2, int32_t y2);
  static Rectangle FromSize(int32_t left,
                            int32_t top,
                            int32_t width,
                            int32_t height);

 protected:
  Rectangle(int32_t left, int32_t top, int32_t width, int32_t height);

 private:
  int32_t left_;
  int32_t top_;
  int32_t right_;
  int32_t bottom_;
  int32_t width_;
  int32_t height_;
};

inline Rectangle::Rectangle() {
  left_ = INT16_MIN;
  top_ = INT16_MIN;
  right_ = INT16_MAX;
  bottom_ = INT16_MAX;
  width_ = INT16_MAX - INT16_MIN;
  height_ = INT16_MAX - INT16_MIN;
}

inline Rectangle::Rectangle(int32_t left,
                            int32_t top,
                            int32_t width,
                            int32_t height) {
  left_ = left;
  top_ = top;
  width_ = Max(width, 0);
  height_ = Max(height, 0);
  right_ = left_ + width_ - 1;
  bottom_ = top_ + height_ - 1;
}

inline Rectangle Rectangle::FromPos(int32_t x1,
                                    int32_t y1,
                                    int32_t x2,
                                    int32_t y2) {
  int32_t left = Min(x1, x2);
  int32_t top = Min(y1, y2);
  int32_t width = Abs(x1 - x2);
  int32_t height = Abs(y1 - y2);

  return Rectangle(left, top, width, height);
}

inline Rectangle Rectangle::FromSize(int32_t left,
                                     int32_t top,
                                     int32_t width,
                                     int32_t height) {
  return Rectangle(left, top, width, height);
}

inline bool Rectangle::IsEmpty() const {
  return width_ == 0 && height_ == 0;
}

inline bool Rectangle::Includes(int32_t x, int32_t y) const {
  return x >= left_ && x <= right_ && y >= top_ && y <= bottom_;
}

inline Rectangle Rectangle::MoveTo(int32_t x, int32_t y) const {
  return Rectangle(x, y, width_, height_);
}

inline Rectangle Rectangle::Intersect(const Rectangle& rect) const {
  int32_t left = Max(left_, rect.left_);
  int32_t top = Max(top_, rect.top_);
  int32_t right = Min(right_, rect.right_);
  int32_t bottom = Min(bottom_, rect.bottom_);
  int32_t width = right - left + 1;
  int32_t height = bottom - top + 1;

  if (width > 0 && height > 0) {
    return Rectangle(left, top, width, height);
  } else {
    return Rectangle(0, 0, 0, 0);
  }
}

}  // namespace pyxelcore

#endif  // PYXELCORE_RECTANGLE_H_