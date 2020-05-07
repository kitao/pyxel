#ifndef PYXELCORE_GIF_WRITER_H_
#define PYXELCORE_GIF_WRITER_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Image;

class GifWriter {
 public:
  GifWriter(const std::string& filename,
            int32_t width,
            int32_t height,
            const PaletteColor& palette_color);
  ~GifWriter();

  void AddFrame(const Image* image, int32_t delay_time);
  void EndFrame();

 private:
  int32_t width_;
  int32_t height_;
  std::ofstream ofs_;
  int32_t* last_frame_data_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_GIF_WRITER_H_
