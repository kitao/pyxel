#include "pyxelcore/app.h"

namespace pyxelcore {

App::App(int width, int height, char *caption, int scale, int *palette, int fps,
         int border_width, int border_color) {
  width_ = width;
  height_ = height;
  caption_ = std::string(caption);
  scale_ = scale;

  for (int i = 0; i < 16; i++) {
    palette_[i] = palette[i];
  }

  fps_ = fps;
  border_width_ = border_width;
  border_color_ = border_color;

  InitializeSystem();
  InitializeResource();
  InitializeInput();
  InitializeGraphics();
  InitializeAudio();
}

App::~App() {
  TerminateSystem();
  TerminateResource();
  TerminateInput();
  TerminateGraphics();
  TerminateAudio();
}

} // namespace pyxelcore
