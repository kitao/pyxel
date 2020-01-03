#ifndef PYXELCORE_PROFILER_H_
#define PYXELCORE_PROFILER_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Profiler {
 public:
  Profiler(int32_t measure_frame_count);

  float AverageTime() const { return average_time_; }
  float AverageFPS() const { return average_fps_; }

  void Start();
  void End();

 private:
  int32_t measure_frame_count_;
  int32_t frame_count_;
  int32_t start_time_;
  int32_t total_time_;
  float average_time_;
  float average_fps_;
};

inline Profiler::Profiler(int32_t measure_frame_count) {
  measure_frame_count_ = Max(measure_frame_count, 1);
  frame_count_ = 0;
  start_time_ = 0;
  total_time_ = 0;
  average_time_ = 0.0f;
  average_fps_ = 0.0f;
}

inline void Profiler::Start() {
  start_time_ = SDL_GetTicks();
}

inline void Profiler::End() {
  total_time_ += SDL_GetTicks() - start_time_;
  frame_count_++;

  if (frame_count_ >= measure_frame_count_) {
    average_time_ = static_cast<float>(total_time_) / frame_count_;
    average_fps_ = 1000.0f / average_time_;

    frame_count_ = 0;
    total_time_ = 0;
  }
}

}  // namespace pyxelcore

#endif  // PYXELCORE_PROFILER_H_
