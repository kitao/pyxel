#include "pyxelcore/utilities.h"

#include <algorithm>

namespace pyxelcore {

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
                         int32_t clip_x1,
                         int32_t clip_y1,
                         int32_t clip_x2,
                         int32_t clip_y2) {
  if (clip_x1 >= 0 && clip_y1 >= 0 && clip_x2 >= 0 && clip_y2 >= 0) {
    dest_x = std::max(dest_x, clip_x1);
    dest_y = std::max(dest_y, clip_y1);
    dest_w = std::min(dest_w, clip_x2 - clip_x1 + 1);
    dest_h = std::min(dest_h, clip_y2 - clip_y1 + 1);
  }

  int32_t left_offset = std::max(std::max(-src_x, -dest_x), 0);
  int32_t top_offset = std::max(std::max(-src_y, -dest_y), 0);
  int32_t right_offset =
      std::max(std::max(src_x + copy_w - src_w, dest_x + copy_w - dest_w), 0);
  int32_t bottom_offset =
      std::max(std::max(src_y + copy_h - src_h, dest_y + copy_h - dest_h), 0);

  src_x += left_offset;
  src_y += top_offset;
  dest_x += left_offset;
  dest_y += top_offset;
  copy_w -= left_offset + right_offset;
  copy_h -= top_offset + bottom_offset;

  CopyRegion copy_region = {src_x, src_y, dest_x, dest_y, copy_w, copy_h};

  return copy_region;
}

}  // namespace pyxelcore
