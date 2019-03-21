#ifndef PYXELCORE_IMAGE_H_
#define PYXELCORE_IMAGE_H_

#include <cstdint>

namespace pyxelcore {

class Image {
 public:
  Image(int32_t width, int32_t height);
  ~Image();

  int32_t Width() { return width_; }
  int32_t Height() { return height_; }
  int* Data() { return data_; }

  int32_t Get(int32_t x, int32_t y);
  void Set(int32_t x, int32_t y, int32_t data);
  void Set(int32_t x,
           int32_t y,
           const int* data,
           int32_t data_width,
           int32_t data_height);
  void Load(int32_t x, int32_t y, const char* filename);
  void Copy(int32_t x,
            int32_t y,
            int32_t img,
            int32_t u,
            int32_t v,
            int32_t w,
            int32_t h);

 private:
  int32_t width_;
  int32_t height_;
  int* data_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_IMAGE_H_
