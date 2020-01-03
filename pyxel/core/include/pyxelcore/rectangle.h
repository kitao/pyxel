#ifndef PYXELCORE_RECTANGLE_H_
#define PYXELCORE_RECTANGLE_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Rectangle {
 public:
  Rectangle();
  Rectangle(int32_t left, int32_t top, int32_t width, int32_t height);

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

 private:
  int32_t left_;
  int32_t top_;
  int32_t right_;
  int32_t bottom_;
  int32_t width_;
  int32_t height_;
};

inline Rectangle::Rectangle() {
  left_ = 0;
  top_ = 0;
  right_ = 0;
  bottom_ = 0;
  width_ = 0;
  height_ = 0;
}

inline Rectangle::Rectangle(int32_t left,
                            int32_t top,
                            int32_t width,
                            int32_t height) {
  left_ = left;
  top_ = top;
  width_ = Max(width, 0);
  height_ = Max(height, 0);
  right_ = left + width - 1;
  bottom_ = top + height - 1;
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
    return Rectangle(left, top, width, height);
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
  int32_t left_cut = Max(src.left_ - u, left_ - x, 0);
  int32_t right_cut =
      Max(u + width - 1 - src.right_, x + width - 1 - right_, 0);
  int32_t top_cut = Max(src.top_ - v, top_ - y, 0);
  int32_t bottom_cut =
      Max(v + height - 1 - src.bottom_, y + height - 1 - bottom_, 0);

  CopyArea copy_area = {
      u + (flip_x ? right_cut : left_cut),
      v + (flip_y ? bottom_cut : top_cut),
      x + left_cut,
      y + top_cut,
      Max(width - left_cut - right_cut, 0),
      Max(height - top_cut - bottom_cut, 0),
  };

  return copy_area;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_RECTANGLE_H_