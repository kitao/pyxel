#ifndef PYXELCORE_REGION_H_
#define PYXELCORE_REGION_H_

#include <cstdint>

namespace pyxelcore {

class Region {
 public:
  static const Region ZERO;

  Region() {}

  int32_t Left() { return left_; }
  int32_t Top() { return top_; }
  int32_t Right() { return right_; }
  int32_t Bottom() { return bottom_; }
  int32_t Width() { return width_; }
  int32_t Height() { return height_; }

  bool operator==(const Region& region) const;
  Region operator&(const Region& region) const;
  Region MoveTo(int32_t x, int32_t y) const;
  bool Includes(int32_t x, int32_t y) const;

  static Region FromPos(int32_t x1, int32_t y1, int32_t x2, int32_t y2);
  static Region FromSize(int32_t left,
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

  Region(int32_t left,
         int32_t top,
         int32_t right,
         int32_t bottom,
         int32_t width,
         int32_t height);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_REGION_H_
