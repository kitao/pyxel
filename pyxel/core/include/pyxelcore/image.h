#ifndef PYXELCORE_IMAGE_H_
#define PYXELCORE_IMAGE_H_

#include "pyxelcore/rectangle.h"

namespace pyxelcore {

class Image {
 public:
  Image(int32_t width, int32_t height, int32_t* data = NULL);
  ~Image();

  const Rectangle& Recangle() const { return rect_; }

  int32_t Width() const { return rect_.Width(); }
  int32_t Height() const { return rect_.Height(); }
  int32_t* Data() const { return data_; }

  int32_t GetColor(int32_t x, int32_t y) const;
  void SetColor(int32_t x, int32_t y, int32_t color);
  void SetColor(int32_t x,
                int32_t y,
                const char** color_str,
                int32_t color_str_count);
  void LoadImage(int32_t x,
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

  void DrawImage(int32_t x,
                 int32_t y,
                 const Image* image,
                 const Rectangle& copy_rect,
                 const Rectangle& clip_rect,
                 const int32_t* palette_table = NULL,
                 int32_t color_key = -1);

 private:
  Rectangle rect_;
  int32_t* data_;
  bool need_to_delete_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_IMAGE_H_
