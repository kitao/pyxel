#ifndef PYXELCORE_IMAGE_H_
#define PYXELCORE_IMAGE_H_

#include "pyxelcore/rectangle.h"

namespace pyxelcore {

class Image {
 public:
  Image(int32_t width, int32_t height);
  ~Image();

  const pyxelcore::Rectangle& Rectangle() const { return rect_; }

  int32_t Width() const { return width_; }
  int32_t Height() const { return height_; }
  int32_t* Data() const { return data_; }

  int32_t GetValue(int32_t x, int32_t y) const;
  void SetValue(int32_t x, int32_t y, int32_t value);
  void SetData(int32_t x, int32_t y, const char** data, int32_t data_length);
  bool LoadImage(int32_t x,
                 int32_t y,
                 const char* filename,
                 const int32_t* palette_color);
  void CopyImage(int32_t x,
                 int32_t y,
                 const Image* image,
                 int32_t u,
                 int32_t v,
                 int32_t width,
                 int32_t height);

 private:
  int32_t width_;
  int32_t height_;
  class Rectangle rect_;
  int32_t* data_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_IMAGE_H_
