#ifndef PYXELCORE_UTILITY_H_
#define PYXELCORE_UTILITY_H_

#include <cstdint>

namespace pyxelcore {

struct CopyRegion {
  int32_t src_x;
  int32_t src_y;
  int32_t dest_x;
  int32_t dest_y;
  int32_t copy_w;
  int32_t copy_h;
};

CopyRegion GetCopyRegion(int32_t src_x,
                         int32_t src_y,
                         int32_t src_w,
                         int32_t src_h,
                         int32_t dest_x,
                         int32_t dest_y,
                         int32_t dest_w,
                         int32_t dest_h,
                         int32_t copy_w,
                         int32_t copy_h,
                         int32_t clip_x1 = -1,
                         int32_t clip_y1 = -1,
                         int32_t clip_x2 = -1,
                         int32_t clip_y2 = -1);

void RaiseError(const char* message);

}  // namespace pyxelcore

#endif  // PYXELCORE_UTILITY_H_
