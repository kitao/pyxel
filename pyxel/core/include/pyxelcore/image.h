#ifndef PYXELCORE_IMAGE_H_
#define PYXELCORE_IMAGE_H_

#include <cstddef>
#include <cstdint>

namespace pyxelcore {

class Tilemap;

class Image {
 public:
  Image(int32_t width, int32_t height, int32_t* data = NULL);
  ~Image();

  int32_t Width() { return width_; }
  int32_t Height() { return height_; }
  int32_t* Data() { return data_; }

  int32_t GetColor(int32_t x, int32_t y);
  void SetColor(int32_t x, int32_t y, int32_t color);
  void SetData(int32_t x,
               int32_t y,
               const int32_t* data,
               int32_t data_width,
               int32_t data_height);

  void LoadImage(int32_t x,
                 int32_t y,
                 const char* filename,
                 const int32_t* palette_color);
  void CopyImage(int32_t x,
                 int32_t y,
                 Image* image,
                 int32_t u,
                 int32_t v,
                 int32_t w,
                 int32_t h,
                 int32_t color_key = -1);

 private:
  int32_t width_;
  int32_t height_;
  int32_t* data_;
  bool need_to_delete_data_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_IMAGE_H_
