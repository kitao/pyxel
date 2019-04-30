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

  is_performance_monitor_on = false;
}

System::~System() {
  delete window_;
}

void System::Run(void (*update)(), void (*draw)()) {
  try {
    uint32_t cur_time = SDL_GetTicks();

    double one_frame_time = 1000.0f / fps_;
    double next_update_time = cur_time + one_frame_time;

    fps_profiler_.Start();

    UpdateFrame(update);
    DrawFrame(draw);

    while (true) {
      double sleep_time = next_update_time - SDL_GetTicks();

      if (sleep_time > 0) {
        SDL_Delay(static_cast<int32_t>(sleep_time / 2));
        continue;
      }

      fps_profiler_.End(MEASURE_FRAME_COUNT);
      fps_profiler_.Start();

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
  update_profiler_.Start();

  if (window_->ProcessEvents()) {
    Quit();
  }

  input_->Update(window_, frame_count_);
  CheckSpecialInput();
  update();

  update_profiler_.End(MEASURE_FRAME_COUNT);
}

void System::CheckSpecialInput() {
  if (input_->IsButtonOn(KEY_ALT)) {
    if (input_->IsButtonPressed(KEY_ENTER)) {
      window_->ToggleFullscreen();
    }

    if (input_->IsButtonPressed(KEY_0)) {
      is_performance_monitor_on = !is_performance_monitor_on;
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
  draw_profiler_.Start();

  draw();
  DrawPerformanceMonitor();
  DrawMouseCursor();

  window_->Render(graphics_->ScreenData(), palette_color_);

  draw_profiler_.End(MEASURE_FRAME_COUNT);
}

void System::DrawPerformanceMonitor() {
  if (!is_performance_monitor_on) {
    return;
  }

  char buf[16];

  snprintf(buf, sizeof(buf), "%.2f", fps_profiler_.AverageFPS());
  graphics_->DrawText(1, 0, buf, 1);
  graphics_->DrawText(0, 0, buf, 9);

  snprintf(buf, sizeof(buf), "%.2f", update_profiler_.AverageTime());
  graphics_->DrawText(1, 6, buf, 1);
  graphics_->DrawText(0, 6, buf, 9);

  snprintf(buf, sizeof(buf), "%.2f", draw_profiler_.AverageTime());
  graphics_->DrawText(1, 12, buf, 1);
  graphics_->DrawText(0, 12, buf, 9);
}

void System::DrawMouseCursor() {
  /*
        if not self._is_mouse_visible:
            return

        pyxel.blt(
            pyxel.mouse_x,
            pyxel.mouse_y,
            3,
            MOUSE_CURSOR_IMAGE_X,
            MOUSE_CURSOR_IMAGE_Y,
            MOUSE_CURSOR_WIDTH,
            MOUSE_CURSOR_HEIGHT,
            1,
        )
  */
}

}  // namespace pyxelcore
