#include "pyxelcore/system.h"

#include "pyxelcore/audio.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"
#include "pyxelcore/input.h"
#include "pyxelcore/resource.h"

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <cstdio>

namespace pyxelcore {

class PyxelQuit {};

System::System(int32_t width,
               int32_t height,
               const char* caption,
               int32_t scale,
               const int32_t* palette_color,
               int32_t fps,
               int32_t border_width,
               int32_t border_color) {
  resource_ = new pyxelcore::Resource();
  input_ = new pyxelcore::Input();
  graphics_ = new pyxelcore::Graphics(width, height);
  audio_ = new pyxelcore::Audio();
  window_ = new pyxelcore::Window(caption, width, height, scale, border_width,
                                  border_color);

  if (fps < 1) {
    PRINT_ERROR("invalid fps");
    fps = 1;
  }

  fps_ = fps;
  frame_count_ = 0;

  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_color_[i] = palette_color[i];
  }

  measured_update_count_ = 0;
  measured_total_update_time_ = 0;
  measured_average_update_time_ = 0.0f;

  measured_draw_count_ = 0;
  measured_total_draw_time_ = 0;
  measured_average_draw_time_ = 0.0f;

  measured_fps_count_ = 0;
  measured_fps_start_time_ = 0;
  measured_average_fps_ = 0.0f;
}

System::~System() {
  delete window_;
}

void System::Run(void (*update)(), void (*draw)()) {
  try {
    uint32_t cur_time = SDL_GetTicks();

    double one_frame_time = 1000.0f / fps_;
    double next_update_time = cur_time + one_frame_time;

    measured_fps_count_ = 1;
    measured_fps_start_time_ = cur_time;

    UpdateFrame(update);
    DrawFrame(draw);

    while (true) {
      double sleep_time = next_update_time - SDL_GetTicks();

      if (sleep_time > 0) {
        SDL_Delay(static_cast<int32_t>(sleep_time / 2));
        continue;
      }

      if (measured_fps_count_ == MEASURE_FRAME_COUNT) {
        uint32_t cur_time = SDL_GetTicks();

        measured_average_fps_ = MEASURE_FRAME_COUNT * 1000.0f /
                                (cur_time - measured_fps_start_time_);
        measured_fps_count_ = 0;
        measured_fps_start_time_ = cur_time;
      }
      measured_fps_count_++;

      int32_t update_frame_count =
          Min(static_cast<int32_t>(-sleep_time / one_frame_time),
              MAX_FRAME_SKIP_COUNT) +
          1;

      next_update_time += one_frame_time * update_frame_count;

      for (int32_t i = 0; i < update_frame_count; i++) {
        frame_count_++;
        UpdateFrame(update);
      }

      DrawFrame(draw);
    }
  } catch (PyxelQuit) {
    return;
  }
}

void System::Quit() {
  throw PyxelQuit();
}

void System::UpdateFrame(void (*update)()) {
  uint32_t start_time = SDL_GetTicks();

  if (window_->ProcessEvents()) {
    Quit();
  }

  input_->Update(window_, frame_count_);
  CheckSpecialInput();
  update();

  measured_total_update_time_ += SDL_GetTicks() - start_time;
  measured_update_count_++;

  if (measured_update_count_ == MEASURE_FRAME_COUNT) {
    measured_average_update_time_ =
        static_cast<float>(measured_total_update_time_) / MEASURE_FRAME_COUNT;
    measured_update_count_ = 0;
    measured_total_update_time_ = 0;
  }
}

void System::CheckSpecialInput() {
  if (input_->IsButtonOn(KEY_ALT)) {
    if (input_->IsButtonPressed(KEY_ENTER)) {
      // toggle fullscreen
    }

    if (input_->IsButtonPressed(KEY_0)) {
      // toggle performance monitor
    }

    if (input_->IsButtonPressed(KEY_1)) {
      // capture image
    }

    if (input_->IsButtonPressed(KEY_2)) {
      // reset animation capture
    }

    if (input_->IsButtonPressed(KEY_3)) {
      // save animation
    }
  }

  if (input_->IsButtonPressed(KEY_ESCAPE)) {
    Quit();
  }
}

void System::DrawFrame(void (*draw)()) {
  uint32_t start_time = SDL_GetTicks();

  draw();
  // self._draw_perf_monitor()
  // self._draw_mouse_cursor()

  window_->Render(graphics_->ScreenData(), palette_color_);

  measured_total_draw_time_ += SDL_GetTicks() - start_time;
  measured_draw_count_++;

  if (measured_draw_count_ == MEASURE_FRAME_COUNT) {
    measured_average_draw_time_ =
        static_cast<float>(measured_total_draw_time_) / MEASURE_FRAME_COUNT;
    measured_draw_count_ = 0;
    measured_total_draw_time_ = 0;
  }
}

}  // namespace pyxelcore
