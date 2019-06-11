#include "pyxelcore/recorder.h"

#include "pyxelcore/image.h"

#include "gif/gif.h"

namespace pyxelcore {

Recorder::Recorder(int32_t width, int32_t height, int32_t fps) {
  width_ = width;
  height_ = height;
  fps_ = fps;
  cur_frame_ = -1;
  start_frame_ = 0;
  frame_count_ = 0;

  for (int32_t i = 0; i < SCREEN_CAPTURE_COUNT; i++) {
    captured_images_[i] = new Image(width, height);
  }
}

Recorder::~Recorder() {
  for (int32_t i = 0; i < SCREEN_CAPTURE_COUNT; i++) {
    delete captured_images_[i];
  }
}

void Recorder::SaveScreenshot() {
  //
  std::cout << GetFilename() << std::endl;
}

void Recorder::ResetScreenCapture() {
  start_frame_ = (cur_frame_ + 1) % SCREEN_CAPTURE_COUNT;
  frame_count_ = 0;
}

void Recorder::SaveScreenCapture() {
  //
  std::cout << GetFilename() << std::endl;
}

void Recorder::Update(const Image* screen_image) {
  cur_frame_ = (cur_frame_ + 1) % SCREEN_CAPTURE_COUNT;
  frame_count_++;
  captured_images_[cur_frame_]->CopyImage(0, 0, screen_image, 0, 0, width_,
                                          height_);

  if (frame_count_ > SCREEN_CAPTURE_COUNT) {
    start_frame_ = (start_frame_ + 1) % SCREEN_CAPTURE_COUNT;
    frame_count_ = SCREEN_CAPTURE_COUNT;
  }
}

std::string Recorder::GetFilename() const {
#ifdef WIN32
  std::string desktop_path = getenv("USERPROFILE");
  desktop_path += "\\Desktop\\";
#else
  std::string desktop_path = getenv("HOME");
  desktop_path += "/Desktop/";
#endif
  //
  return desktop_path;
}

/*
    def _save_capture_image(self):
        image.save(self._get_capture_filename() + ".png", optimize=True)

    def _get_capture_filename():
        return os.path.join(
            get_desktop_path(),
datetime.datetime.now().strftime("pyxel-%y%m%d-%H%M%S")
*/

}  // namespace pyxelcore
