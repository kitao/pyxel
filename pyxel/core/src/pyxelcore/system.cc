#include "pyxelcore/system.h"

#include "pyxelcore/audio.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"
#include "pyxelcore/input.h"
#include "pyxelcore/resource.h"

namespace pyxelcore {

class PyxelQuit {};

System::System(int32_t width,
               int32_t height,
               const char* caption,
               int32_t scale,
               const int32_t* palette_color,
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

  resource_ = new pyxelcore::Resource();
  input_ = new pyxelcore::Input();
  graphics_ = new pyxelcore::Graphics(width, height);
  audio_ = new pyxelcore::Audio();
  window_ = new pyxelcore::Window(caption, width, height, scale, border_width,
                                  border_color);
  fps_ = fps;
  frame_count_ = 0;
  is_performance_monitor_on_ = false;

  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_color_[i] = palette_color[i];
  }
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

      fps_profiler_.End();
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

  draw_profiler_.End();
}

void System::DrawPerformanceMonitor() {
  if (!is_performance_monitor_on_) {
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
  if (!input_->IsMouseVisible()) {
    return;
  }

  graphics_->DrawImage(input_->MouseX(), input_->MouseY(),
                       IMAGE_BANK_FOR_SYSTEM, MOUSE_CURSOR_X, MOUSE_CURSOR_Y,
                       MOUSE_CURSOR_WIDTH, MOUSE_CURSOR_HEIGHT, 1);
}

/*
class App:
    def _draw_frame(self):
        draw_start_time = time.time()

        self._draw()

        self._draw_perf_monitor()
        self._draw_mouse_cursor()

        hs = self._hidpi_scale
        image = self._renderer.render(
            self._viewport_left * hs,
            self._viewport_bottom * hs,
            self._viewport_width * hs,
            self._viewport_height * hs,
            self._palette,
            self._border_color,
        )
        self._capture_images[self._capture_count % APP_GIF_CAPTURE_COUNT] =
image self._capture_count += 1

        self._measure_draw_time(draw_start_time)

    def _save_capture_image(self):
        index = (self._capture_count - 1) % APP_GIF_CAPTURE_COUNT
        image = self._get_capture_image(index)
        image.save(self._get_capture_filename() + ".png", optimize=True)

    def _save_capture_animation(self):
        image_count = min(
            self._capture_count - self._capture_start, APP_GIF_CAPTURE_COUNT
        )

        if image_count <= 0:
            return

        start_index = (self._capture_count - image_count) %
APP_GIF_CAPTURE_COUNT images = [self._get_capture_image(start_index)]

        for i in range(1, image_count):
            index = (start_index + i) % APP_GIF_CAPTURE_COUNT
            image = self._difference(
                self._get_capture_image(index - 1),
self._get_capture_image(index)
            )
            images.append(image)

        color_index = self._get_color_palette_index(image,
GIF_TRANSPARENCY_COLOR)

        images[0].save(
            self._get_capture_filename() + ".gif",
            save_all=True,
            append_images=images[1:],
            duration=self._one_frame_time * 1000,
            loop=0,
            optimize=False,
            transparency=color_index,
            disposal=1,
            palette=get_palette(fill=False),
        )

    @staticmethod
    def _get_capture_filename():
        return os.path.join(
            get_desktop_path(),
datetime.datetime.now().strftime("pyxel-%y%m%d-%H%M%S")
        )
*/

}  // namespace pyxelcore
