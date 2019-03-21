#ifndef PYXELCORE_IMAGE_H_
#define PYXELCORE_IMAGE_H_

#include <cstdint>

namespace pyxelcore {

class Image {
 public:
  Image(int32_t width, int32_t height);
  ~Image();

  int32_t width() { return width_; }
  int32_t height() { return height_; }
  int* data() { return data_; }

  int32_t get(int32_t x, int32_t y);
  void set(int32_t x, int32_t y, int32_t data);
  void set(int32_t x,
           int32_t y,
           const int* data,
           int32_t data_width,
           int32_t data_height);
  void load(int32_t x, int32_t y, const char* filename);
  void copy(int32_t x,
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
