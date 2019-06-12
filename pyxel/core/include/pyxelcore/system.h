#ifndef PYXELCORE_SYSTEM_H_
#define PYXELCORE_SYSTEM_H_

#include "pyxelcore/profiler.h"
#include "pyxelcore/window.h"

namespace pyxelcore {

class Resource;
class Input;
class Graphics;
class Audio;
class Recorder;

class System {
 public:
  System(int32_t width,
         int32_t height,
         const std::string& caption = DEFAULT_CAPTION,
         int32_t scale = DEFAULT_SCALE,
         const pyxelcore::PaletteColor& palette_color = DEFAULT_PALETTE,
         int32_t fps = DEFAULT_FPS,
         int32_t border_width = DEFAULT_BORDER_WIDTH,
         int32_t border_color = DEFAULT_BORDER_COLOR);
  ~System();

  pyxelcore::Resource* Resource() const { return resource_; }
  pyxelcore::Input* Input() const { return input_; }
  pyxelcore::Graphics* Graphics() const { return graphics_; }
  pyxelcore::Audio* Audio() const { return audio_; }
  const pyxelcore::PaletteColor& PaletteColor() const { return palette_color_; }

  int32_t Width() const { return window_->ScreenWidth(); }
  int32_t Height() const { return window_->ScreenHeight(); }
  int32_t FrameCount() const { return frame_count_; }
  const char* DropFile() const {
    return drop_file_.size() > 0 ? drop_file_.c_str() : nullptr;
  }

  void Run(void (*update)(), void (*draw)());
  void Quit();
  void SetCaption(const std::string& caption);

 private:
  pyxelcore::Resource* resource_;
  pyxelcore::Input* input_;
  pyxelcore::Graphics* graphics_;
  pyxelcore::Audio* audio_;
  Window* window_;
  Recorder* recorder_;

  int32_t fps_;
  int32_t frame_count_;
  bool is_update_suspended_;
  std::string drop_file_;
  pyxelcore::PaletteColor palette_color_;

  Profiler fps_profiler_;
  Profiler update_profiler_;
  Profiler draw_profiler_;
  bool is_performance_monitor_on_;

  void UpdateFrame(void (*update)());
  void CheckSpecialInput();
  void DrawFrame(void (*draw)());
  void DrawPerformanceMonitor();
  void DrawMouseCursor();
};

}  // namespace pyxelcore

#endif  // PYXELCORE_SYSTEM_H_
