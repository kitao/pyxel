#ifndef PYXELCORE_RECTANGLE_H_
#define PYXELCORE_RECTANGLE_H_

#include "pyxelcore/common.h"

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
  Rectangle Intersect(const Rectangle& rect) const;

  struct CopyArea {
    int32_t u;
    int32_t v;
    int32_t x;
    int32_t y;
    int32_t width;
    int32_t height;

    bool IsEmpty() { return width == 0 || height == 0; }
  };

  CopyArea GetCopyArea(int32_t x,
                       int32_t y,
                       const Rectangle& src,
                       int32_t u,
                       int32_t v,
                       int32_t width,
                       int32_t height,
                       bool flip_x = false,
                       bool flip_y = false) const;

  static Rectangle FromPos(int32_t left,
                           int32_t top,
                           int32_t right,
                           int32_t bottom);
  static Rectangle FromSize(int32_t left,
                            int32_t top,
                            int32_t width,
                            int32_t height);

 private:
  int32_t left_;
  int32_t top_;
  int32_t right_;
  int32_t bottom_;
  int32_t width_;
  int32_t height_;

  Rectangle(int32_t left,
            int32_t top,
            int32_t right,
            int32_t bottom,
            int32_t width,
            int32_t height);
};

inline Rectangle::Rectangle()
    : left_(0), top_(0), right_(0), bottom_(0), width_(0), height_(0) {}

inline Rectangle::Rectangle(int32_t left,
                            int32_t top,
                            int32_t right,
                            int32_t bottom,
                            int32_t width,
                            int32_t height)
    : left_(left),
      top_(top),
      right_(right),
      bottom_(bottom),
      width_(width),
      height_(height) {}

inline Rectangle Rectangle::FromPos(int32_t left,
                                    int32_t top,
                                    int32_t right,
                                    int32_t bottom) {
  if (left > right) {
    int32_t tmp = left;
    left = right;
    right = tmp;
  }

  if (top > bottom) {
    int32_t tmp = top;
    top = bottom;
    bottom = tmp;
  }

  int32_t width = right - left + 1;
  int32_t height = bottom - top + 1;

  return Rectangle(left, top, right, bottom, width, height);
}

inline Rectangle Rectangle::FromSize(int32_t left,
                                     int32_t top,
                                     int32_t width,
                                     int32_t height) {
  width = Max(width, 0);
  height = Max(height, 0);

  int32_t right = Max(left + width - 1, 0);
  int32_t bottom = Max(top + height - 1, 0);

  return Rectangle(left, top, right, bottom, width, height);
}

inline bool Rectangle::IsEmpty() const {
  return width_ == 0 || height_ == 0;
}

inline bool Rectangle::Includes(int32_t x, int32_t y) const {
  return x >= left_ && x <= right_ && y >= top_ && y <= bottom_;
}

inline Rectangle Rectangle::Intersect(const Rectangle& rect) const {
  int32_t left = Max(left_, rect.left_);
  int32_t top = Max(top_, rect.top_);
  int32_t right = Min(right_, rect.right_);
  int32_t bottom = Min(bottom_, rect.bottom_);
  int32_t width = right - left + 1;
  int32_t height = bottom - top + 1;

  if (width > 0 && height > 0) {
    return Rectangle(left, top, right, bottom, width, height);
  } else {
    return Rectangle();
  }
}

inline Rectangle::CopyArea Rectangle::GetCopyArea(int32_t x,
                                                  int32_t y,
                                                  const Rectangle& src,
                                                  int32_t u,
                                                  int32_t v,
                                                  int32_t width,
                                                  int32_t height,
                                                  bool flip_x,
                                                  bool flip_y) const {
  CopyArea copy_area;

  if (flip_x) {
    int32_t left = Max(src.left_ - u, left_ - x, 0);
    int32_t right = Min(src.right_ - u, right_ - x, width - 1);

    copy_area.u = left + u;
    copy_area.x = left + x;
    copy_area.width = Max(right - left + 1, 0);
    // left = Max(u + width - 1 - src.right_, left_ - x, 0);
    // right = Min(u + width - 1 - src.left_, right_ - x, width - 1);
  } else {
    int32_t left = Max(src.left_ - u, left_ - x, 0);
    int32_t right = Min(src.right_ - u, right_ - x, width - 1);

    copy_area.u = left + u;
    copy_area.x = left + x;
    copy_area.width = Max(right - left + 1, 0);
  }

  if (flip_y) {
    int32_t top = Max(src.top_ - v, top_ - y, 0);
    int32_t bottom = Min(src.bottom_ - v, bottom_ - y, height - 1);

    copy_area.v = top + v;
    copy_area.y = top + y;
    copy_area.height = Max(bottom - top + 1, 0);
    // top = Max(v + height - 1 - src.bottom_, y + height - 1 - bottom_, 0);
    // bottom = Min(v + height - 1 - src.top_, y + height - 1 - top_, height -
    // 1);
  } else {
    int32_t top = Max(src.top_ - v, top_ - y, 0);
    int32_t bottom = Min(src.bottom_ - v, bottom_ - y, height - 1);

    copy_area.v = top + v;
    copy_area.y = top + y;
    copy_area.height = Max(bottom - top + 1, 0);
  }

  return copy_area;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_RECTANGLE_H_