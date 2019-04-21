#ifndef PYXELCORE_RECTANGLE_H_
#define PYXELCORE_RECTANGLE_H_

#include <cstdint>

namespace pyxelcore {

class Rectangle {
 public:
  Rectangle();

  static Rectangle FromPos(int32_t x1, int32_t y1, int32_t x2, int32_t y2);
  static Rectangle FromSize(int32_t left,
                            int32_t top,
                            int32_t width,
                            int32_t height);

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

}  // namespace pyxelcore

#endif  // PYXELCORE_RECTANGLE_H_