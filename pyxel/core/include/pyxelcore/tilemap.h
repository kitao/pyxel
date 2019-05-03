#ifndef PYXELCORE_TILEMAP_H_
#define PYXELCORE_TILEMAP_H_

#include "pyxelcore/rectangle.h"

namespace pyxelcore {

class Tilemap {
 public:
  Tilemap(int32_t width, int32_t height);
  ~Tilemap();

  const class Rectangle& Rectangle() const { return rect_; }

  int32_t Width() const { return rect_.Width(); }
  int32_t Height() const { return rect_.Height(); }
  int32_t* Data() const { return data_; }
  int32_t ImageIndex() const { return image_index_; }
  void ImageIndex(int32_t image_index);

  int32_t GetValue(int32_t x, int32_t y) const;
  void SetValue(int32_t x, int32_t y, int32_t value);
  void SetValue(int32_t x, int32_t y, const char** value, int32_t value_count);
  void CopyTilemap(int32_t x,
                   int32_t y,
                   const Tilemap* tilemap,
                   int32_t u,
                   int32_t v,
                   int32_t width,
                   int32_t height);

 private:
  class Rectangle rect_;
  int32_t* data_;
  int32_t image_index_;
};

inline void Tilemap::ImageIndex(int32_t image_index) {
  if (image_index < 0 || image_index >= IMAGE_BANK_COUNT) {
    PRINT_ERROR("invalid image index");
    return;
  }

  image_index_ = image_index;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_TILEMAP_H_
