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
         const std::string& caption,
         int32_t scale,
         const pyxelcore::PaletteColor& palette_color,
         int32_t fps,
         int32_t quit_key,
         bool is_fullscreen);
  ~System();

  pyxelcore::Resource* Resource() const { return resource_; }
  pyxelcore::Input* Input() const { return input_; }
  pyxelcore::Graphics* Graphics() const { return graphics_; }
  pyxelcore::Audio* Audio() const { return audio_; }
  const pyxelcore::PaletteColor& PaletteColor() const { return palette_color_; }

  int32_t Width() const { return window_->ScreenWidth(); }
  int32_t Height() const { return window_->ScreenHeight(); }
  int32_t FrameCount() const { return frame_count_; }

  void Run(void (*update)(), void (*draw)());
  bool Quit();
  bool FlipScreen();
  void ShowScreen();

  std::string DropFile() const { return drop_file_; }
  void SetCaption(const std::string& caption);

 private:
  pyxelcore::Resource* resource_;
  pyxelcore::Input* input_;
  pyxelcore::Graphics* graphics_;
  pyxelcore::Audio* audio_;
  Window* window_;
  Recorder* recorder_;

  pyxelcore::PaletteColor palette_color_;
  int32_t quit_key_;
  int32_t fps_;
  int32_t frame_count_;
  double one_frame_time_;
  double next_update_time_;
  std::string drop_file_;
  bool is_loop_running_;
  bool is_quit_requested_;
  bool is_update_suspended_;

  Profiler fps_profiler_;
  Profiler update_profiler_;
  Profiler draw_profiler_;
  bool is_performance_monitor_on_;

  int32_t WaitForUpdateTime();
  bool UpdateFrame(void (*update)());
  void CheckSpecialInput();
  void DrawFrame(void (*draw)(), int32_t update_frame_count);
  void DrawPerformanceMonitor();
  void DrawMouseCursor();
};

}  // namespace pyxelcore

#endif  // PYXELCORE_SYSTEM_H_
