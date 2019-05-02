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
  Rectangle MoveTo(int32_t x, int32_t y) const;
  Rectangle Intersect(const Rectangle& rect) const;

  struct CopyArea {
    int32_t src_x;
    int32_t src_y;
    int32_t dst_x;
    int32_t dst_y;
    int32_t width;
    int32_t height;
  };

  CopyArea GetCopyArea(int32_t x,
                       int32_t y,
                       const Rectangle& src_rect,
                       const Rectangle& copy_rect) const;

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

  return Rectangle(left, top, right, bottom, right - left + 1,
                   bottom - top + 1);
}

inline Rectangle Rectangle::FromSize(int32_t left,
                                     int32_t top,
                                     int32_t width,
                                     int32_t height) {
  width = Max(width, 0);
  height = Max(height, 0);

  return Rectangle(left, top, left + width - 1, top + height - 1, width,
                   height);
}

inline bool Rectangle::IsEmpty() const {
  return width_ == 0 || height_ == 0;
}

inline bool Rectangle::Includes(int32_t x, int32_t y) const {
  return x >= left_ && x <= right_ && y >= top_ && y <= bottom_;
}

inline Rectangle Rectangle::MoveTo(int32_t x, int32_t y) const {
  return Rectangle(x, y, x + width_ - 1, y + height_ - 1, width_, height_);
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

inline Rectangle::CopyArea Rectangle::GetCopyArea(
    int32_t x,
    int32_t y,
    const Rectangle& src_rect,
    const Rectangle& copy_rect) const {
  Rectangle cr = copy_rect.Intersect(src_rect);

  int32_t dst_x = x + Max(cr.Left() - copy_rect.Left(), 0);
  int32_t dst_y = y + Max(cr.Top() - copy_rect.Top(), 0);
  Rectangle dst = cr.MoveTo(dst_x, dst_y).Intersect(*this);

  int32_t src_x = copy_rect.Left() + Max(dst.Left() - x, 0);
  int32_t src_y = copy_rect.Top() + Max(dst.Top() - y, 0);

  CopyArea copy_area = {
      src_x, src_y, dst.Left(), dst.Top(), dst.Width(), dst.Height(),
  };

  return copy_area;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_RECTANGLE_H_