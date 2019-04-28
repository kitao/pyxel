#ifndef PYXELCORE_IMAGE_H_
#define PYXELCORE_IMAGE_H_

#include "pyxelcore/rectangle.h"

#include <cstddef>
#include <cstdint>

namespace pyxelcore {

class Image : public Rectangle {
 public:
  Image(int32_t width, int32_t height, int32_t* data = NULL);
  ~Image();

  int32_t* Data() const { return data_; }

  int32_t GetColor(int32_t x, int32_t y) const;
  void SetColor(int32_t x, int32_t y, int32_t color);
  void SetColor(int32_t x, int32_t y, const char** str, int32_t str_count);
  void LoadImage(int32_t x,
                 int32_t y,
                 const char* filename,
                 const int32_t* palette_color);
  void CopyImage(int32_t x,
                 int32_t y,
                 const Image* image,
                 const Rectangle& copy_rect,
                 const Rectangle& clip_rect,
                 const int32_t* palette_table = NULL,
                 int32_t color_key = -1);

 private:
  bool need_to_delete_;
  int32_t* data_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_IMAGE_H_
