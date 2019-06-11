#include "pyxelcore/system.h"

#include "pyxelcore/audio.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"
#include "pyxelcore/input.h"
#include "pyxelcore/recorder.h"
#include "pyxelcore/resource.h"

#define STORE_CLIP_AREA_AND_PALETTE()                    \
  int32_t cur_palette_table[COLOR_COUNT];                \
  for (int32_t i = 0; i < COLOR_COUNT; i++) {            \
    cur_palette_table[i] = graphics_->PaletteTable()[i]; \
  }                                                      \
                                                         \
  Rectangle cur_clip_area = graphics_->ClipArea();       \
  graphics_->ResetClipArea()

#define RESTORE_CLIP_AREA_AND_PALETTE()                             \
  for (int32_t i = 0; i < COLOR_COUNT; i++) {                       \
    graphics_->SetPalette(i, cur_palette_table[i]);                 \
  }                                                                 \
                                                                    \
  graphics_->SetClipArea(cur_clip_area.Left(), cur_clip_area.Top(), \
                         cur_clip_area.Width(), cur_clip_area.Height())

namespace pyxelcore {

class PyxelQuit {};

System::System(int32_t width,
               int32_t height,
               const std::string& caption,
               int32_t scale,
               const pyxelcore::PaletteColor& palette_color,
               int32_t fps,
               int32_t border_width,
               int32_t border_color)
    : fps_profiler_(MEASURE_FRAME_COUNT),
      update_profiler_(MEASURE_FRAME_COUNT),
      draw_profiler_(MEASURE_FRAME_COUNT) {
  if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO) != 0) {
    PRINT_ERROR("failed to initialize SDL");
    exit(1);
  }

  if (IMG_Init(IMG_INIT_PNG) != IMG_INIT_PNG) {
    PRINT_ERROR("failed to initialize SDL_image");
    exit(1);
  }

  if (width < MIN_SCREEN_SIZE || width > MAX_SCREEN_SIZE ||
      height < MIN_SCREEN_SIZE || height > MAX_SCREEN_SIZE) {
    PRINT_ERROR("invalid screen size");
    width = Clamp(width, MIN_SCREEN_SIZE, MAX_SCREEN_SIZE);
    height = Clamp(height, MIN_SCREEN_SIZE, MAX_SCREEN_SIZE);
  }

  if (fps < 1) {
    PRINT_ERROR("invalid fps");
    fps = 1;
  }

  input_ = new pyxelcore::Input();
  graphics_ = new pyxelcore::Graphics(width, height);
  audio_ = new pyxelcore::Audio();
  resource_ = new pyxelcore::Resource(graphics_, audio_);
  window_ =
      new Window(caption, width, height, scale, border_width, border_color);
  recorder_ = new Recorder(width, height, palette_color, fps);

  palette_color_ = palette_color;
  fps_ = fps;
  frame_count_ = 0;
  is_update_suspended_ = false;
  is_performance_monitor_on_ = false;
}

System::~System() {
  IMG_Quit();
  SDL_Quit();

  delete recorder_;
  delete window_;
  delete resource_;
  delete audio_;
  delete graphics_;
  delete input_;
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

      fps_profiler_.End();
      fps_profiler_.Start();

      int32_t update_frame_count;

      if (is_update_suspended_) {
        is_update_suspended_ = false;
        update_frame_count = 1;
        next_update_time = SDL_GetTicks() + one_frame_time;
      } else {
        update_frame_count =
            Min(static_cast<int32_t>(-sleep_time / one_frame_time),
                MAX_FRAME_SKIP_COUNT) +
            1;
        next_update_time += one_frame_time * update_frame_count;
      }

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

  update_profiler_.End();
}

void System::CheckSpecialInput() {
  if (input_->IsButtonOn(KEY_ALT)) {
    if (input_->IsButtonPressed(KEY_ENTER)) {
      window_->ToggleFullscreen();
    }

    if (input_->IsButtonPressed(KEY_0)) {
      is_performance_monitor_on_ = !is_performance_monitor_on_;
    }

    if (input_->IsButtonPressed(KEY_1)) {
      recorder_->SaveScreenshot();
      is_update_suspended_ = true;
    }

    if (input_->IsButtonPressed(KEY_2)) {
      recorder_->ResetScreenCapture();
    }

    if (input_->IsButtonPressed(KEY_3)) {
      recorder_->SaveScreenCapture();
      is_update_suspended_ = true;
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

  window_->Render(graphics_->ScreenImage()->Data(), palette_color_);
  recorder_->Update(graphics_->ScreenImage());

  draw_profiler_.End();
}

void System::DrawPerformanceMonitor() {
  if (!is_performance_monitor_on_) {
    return;
  }

  STORE_CLIP_AREA_AND_PALETTE();

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

  RESTORE_CLIP_AREA_AND_PALETTE();
}

void System::DrawMouseCursor() {
  if (!input_->IsMouseVisible()) {
    return;
  }

  STORE_CLIP_AREA_AND_PALETTE();

  graphics_->ResetPalette();
  graphics_->DrawImage(input_->MouseX(), input_->MouseY(),
                       IMAGE_BANK_FOR_SYSTEM, MOUSE_CURSOR_X, MOUSE_CURSOR_Y,
                       MOUSE_CURSOR_WIDTH, MOUSE_CURSOR_HEIGHT, 1);

  RESTORE_CLIP_AREA_AND_PALETTE();
}

}  // namespace pyxelcore
