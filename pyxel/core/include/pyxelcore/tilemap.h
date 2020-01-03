#ifndef PYXELCORE_TILEMAP_H_
#define PYXELCORE_TILEMAP_H_

#include "pyxelcore/rectangle.h"

namespace pyxelcore {

class Tilemap {
 public:
  Tilemap(int32_t width, int32_t height);
  ~Tilemap();

  const pyxelcore::Rectangle& Rectangle() const { return rect_; }

  int32_t Width() const { return width_; }
  int32_t Height() const { return height_; }
  int32_t** Data() const { return data_; }
  int32_t ImageIndex() const { return image_index_; }
  void ImageIndex(int32_t image_index);

  int32_t GetValue(int32_t x, int32_t y) const;
  void SetValue(int32_t x, int32_t y, int32_t value);
  void SetData(int32_t x, int32_t y, const TilemapString& tilemap_string);
  void CopyTilemap(int32_t x,
                   int32_t y,
                   const Tilemap* tilemap,
                   int32_t u,
                   int32_t v,
                   int32_t width,
                   int32_t height);

 private:
  int32_t width_;
  int32_t height_;
  pyxelcore::Rectangle rect_;
  int32_t** data_;
  int32_t image_index_;
};

inline void Tilemap::ImageIndex(int32_t image_index) {
  if (image_index < 0 || image_index >= TOTAL_IMAGE_BANK_COUNT) {
    PYXEL_ERROR("invalid image index");
  }

  image_index_ = image_index;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_TILEMAP_H_
