#ifndef PYXELCORE_RECORDER_H_
#define PYXELCORE_RECORDER_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Image;

class Recorder {
 public:
  Recorder(int32_t width,
           int32_t height,
           const PaletteColor& palette_color,
           int32_t fps);
  ~Recorder();

  void SaveScreenshot();
  void ResetScreenCapture();
  void SaveScreenCapture();

  void Update(const Image* screen_image, int32_t update_frame_count);

 private:
  int32_t width_;
  int32_t height_;
  PaletteColor palette_color_;
  int32_t fps_;
  int32_t cur_frame_;
  int32_t start_frame_;
  int32_t frame_count_;
  Image* captured_images_[SCREEN_CAPTURE_COUNT];
  int32_t captured_frames_[SCREEN_CAPTURE_COUNT];

  std::string GetBaseName() const;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_RECORDER_H_
