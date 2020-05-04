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

class ExitPyxel {};

System::System(int32_t width,
               int32_t height,
               const std::string& caption,
               int32_t scale,
               const pyxelcore::PaletteColor& palette_color,
               int32_t fps,
               int32_t quit_key,
               bool is_fullscreen)
    : fps_profiler_(MEASURE_FRAME_COUNT),
      update_profiler_(MEASURE_FRAME_COUNT),
      draw_profiler_(MEASURE_FRAME_COUNT) {
  if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMECONTROLLER) !=
      0) {
    PYXEL_ERROR("failed to initialize SDL");
  }

  if (IMG_Init(IMG_INIT_PNG) != IMG_INIT_PNG) {
    PYXEL_ERROR("failed to initialize SDL_image");
  }

  if (width < 1 || width > MAX_SCREEN_SIZE || height < 1 ||
      height > MAX_SCREEN_SIZE) {
    PYXEL_ERROR("invalid screen size");
  }

  if (fps < 1) {
    PYXEL_ERROR("invalid fps");
  }

  input_ = new pyxelcore::Input();
  graphics_ = new pyxelcore::Graphics(width, height);
  audio_ = new pyxelcore::Audio();
  resource_ = new pyxelcore::Resource(graphics_, audio_);
  window_ = new Window(caption, width, height, scale, palette_color);
  recorder_ = new Recorder(width, height, palette_color, fps);

  palette_color_ = palette_color;
  fps_ = fps;
  quit_key_ = quit_key;
  frame_count_ = 0;
  one_frame_time_ = 1000.0f / fps_;
  next_update_time_ = SDL_GetTicks();
  is_update_suspended_ = false;
  drop_file_ = "";
  is_performance_monitor_on_ = false;

  fps_profiler_.Start();

  if (is_fullscreen) {
    window_->ToggleFullscreen();
  }
}

System::~System() {
  delete recorder_;
  delete window_;
  delete resource_;
  delete audio_;
  delete graphics_;
  delete input_;

  IMG_Quit();
  SDL_Quit();
}

void System::Run(void (*update)(), void (*draw)()) {
  try {
    next_update_time_ = SDL_GetTicks() + one_frame_time_;
    is_update_suspended_ = true;

    UpdateFrame(update);
    DrawFrame(draw, 1);

    while (true) {
      double sleep_time = WaitForUpdateTime();

      fps_profiler_.End();
      fps_profiler_.Start();

      int32_t update_frame_count;

      if (is_update_suspended_) {
        is_update_suspended_ = false;
        update_frame_count = 1;
        next_update_time_ = SDL_GetTicks() + one_frame_time_;
      } else {
        update_frame_count =
            Min(static_cast<int32_t>(-sleep_time / one_frame_time_),
                MAX_FRAME_SKIP_COUNT) +
            1;
        next_update_time_ += one_frame_time_ * update_frame_count;
      }

      for (int32_t i = 0; i < update_frame_count; i++) {
        frame_count_++;
        UpdateFrame(update);
      }

      DrawFrame(draw, update_frame_count);
    }
  } catch (ExitPyxel) {
    delete this;
  }
}

void System::Quit() {
  throw ExitPyxel();
}

bool System::FlipScreen() {
  try {
    WaitForUpdateTime();
    next_update_time_ += one_frame_time_;

    fps_profiler_.End();
    fps_profiler_.Start();

    frame_count_++;
    UpdateFrame(nullptr);
    DrawFrame(nullptr, 1);

    return false;
  } catch (ExitPyxel) {
    delete this;
  }

  return true;
}

void System::ShowScreen() {
  while (true) {
    if (FlipScreen()) {
      break;
    }
  }
}

void System::SetCaption(const std::string& caption) {
  window_->SetCaption(caption);
}

int32_t System::WaitForUpdateTime() {
  while (true) {
    double sleep_time = next_update_time_ - SDL_GetTicks();

    if (sleep_time <= 0) {
      return sleep_time;
    }

    SDL_Delay(static_cast<int32_t>(sleep_time / 2));
  }
}

void System::UpdateFrame(void (*update)()) {
  update_profiler_.Start();

  if (window_->ProcessEvents()) {
    Quit();
  }

  drop_file_ = window_->GetDropFile();
  input_->Update(window_, frame_count_);
  CheckSpecialInput();

  if (update) {
    update();
  }

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

  if (input_->IsButtonPressed(quit_key_)) {
    Quit();
  }
}

void System::DrawFrame(void (*draw)(), int32_t update_frame_count) {
  draw_profiler_.Start();

  if (draw) {
    draw();
  }

  DrawPerformanceMonitor();
  DrawMouseCursor();

  window_->Render(graphics_->ScreenImage()->Data());
  recorder_->Update(graphics_->ScreenImage(), update_frame_count);

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

  int32_t mouse_x = input_->MouseX();
  int32_t mouse_y = input_->MouseY();

  if (mouse_x < 0 || mouse_x >= window_->ScreenWidth() || mouse_y < 0 ||
      mouse_y >= window_->ScreenHeight()) {
    return;
  }

  STORE_CLIP_AREA_AND_PALETTE();

  graphics_->ResetPalette();
  graphics_->DrawImage(mouse_x, mouse_y, IMAGE_BANK_FOR_SYSTEM, MOUSE_CURSOR_X,
                       MOUSE_CURSOR_Y, MOUSE_CURSOR_WIDTH, MOUSE_CURSOR_HEIGHT,
                       1);

  RESTORE_CLIP_AREA_AND_PALETTE();
}

}  // namespace pyxelcore
