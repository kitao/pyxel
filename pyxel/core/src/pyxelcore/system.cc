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

  width_ = std::max(width, 1);
  height_ = std::max(height, 1);
  caption_ = caption ? std::string(caption) : DEFAULT_CAPTION;
  fps_ = std::max(fps != -1 ? fps : DEFAULT_FPS, 1);
  border_width_ = border_width != -1 ? border_width : DEFAULT_BORDER_WIDTH;
  border_color_ = border_color != -1 ? border_color : DEFAULT_BORDER_COLOR;
  frame_count_ = 0;

  window_info_.screen_scale = scale != -1 ? scale : DEFAULT_SCALE;

  palette_color = palette_color ? palette_color : DEFAULT_PALETTE;
  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_color_[i] = palette_color[i];
  }

  SetupWindow();
}

System::~System() {}

void System::Run(void (*update)(), void (*draw)()) {
  double one_frame_time = 1000.0f / fps_;
  double next_update_time = SDL_GetTicks();
  bool is_first_frame = true;

  while (1) {
    double sleep_time = next_update_time - SDL_GetTicks();

    if (sleep_time > 0) {
      SDL_Delay(static_cast<int32_t>(sleep_time / 2));
      continue;
    }

    int32_t update_frame_count =
        std::min(static_cast<int32_t>(-sleep_time / one_frame_time) + 1, 10);

    next_update_time += one_frame_time * update_frame_count;

    for (int32_t i = 0; i < update_frame_count; i++) {
      SDL_Event event;

      while (SDL_PollEvent(&event)) {
        if (event.type == SDL_QUIT) {
          return;
        } else if (event.type == SDL_WINDOWEVENT) {
          if (event.window.event == SDL_WINDOWEVENT_RESIZED) {
            UpdateWindowInfo();
            /*window_width_ = event.window.data1;
            window_height_ = event.window.data2;
            scale_ = std::min((window_width_ - border_width_ * 2) / width_,
                              (window_height_ - border_width_ * 2) / height_);
            screen_x_ = (window_width_ - width_ * scale_) / 2;
            screen_y_ = (window_height_ - height_ * scale_) / 2;*/
          }
        }
      }

      if (is_first_frame) {
        is_first_frame = false;
      } else {
        frame_count_++;
      }

      input_->UpdateState(frame_count_);
      update();
    }

    draw();
    RenderWindow();
  }
}

void System::SetupWindow() {
  SDL_Init(SDL_INIT_VIDEO);  // TODO: error handling
  IMG_Init(IMG_INIT_PNG);    // TODO: erro handling

  if (window_info_.screen_scale == 0) {
    SDL_DisplayMode display_mode;
    SDL_GetDesktopDisplayMode(0, &display_mode);
    window_info_.screen_scale =
        std::min((display_mode.w - border_width_ * 2) / width_,
                 (display_mode.h - border_width_ * 2) / height_);
  }

  window_info_.window = SDL_CreateWindow(
      caption_.c_str(), SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
      width_ * window_info_.screen_scale, height_ * window_info_.screen_scale,
      SDL_WINDOW_RESIZABLE);
  window_info_.renderer = SDL_CreateRenderer(window_info_.window, -1, 0);
  window_info_.screen_texture =
      SDL_CreateTexture(window_info_.renderer, SDL_PIXELFORMAT_RGB888,
                        SDL_TEXTUREACCESS_STREAMING, width_, height_);

  SDL_SetWindowMinimumSize(window_info_.window, width_, height_);

  UpdateWindowInfo();
}

void System::RenderWindow() {
  SDL_SetRenderDrawColor(window_info_.renderer, (border_color_ >> 16) & 0xff,
                         (border_color_ >> 8) & 0xff, border_color_ & 0xff,
                         255);
  SDL_RenderClear(window_info_.renderer);

  UpdateScreenTexture();

  SDL_Rect dest_rect = {window_info_.screen_x, window_info_.screen_y,
                        width_ * window_info_.screen_scale,
                        height_ * window_info_.screen_scale};
  SDL_RenderCopy(window_info_.renderer, window_info_.screen_texture, NULL,
                 &dest_rect);

  SDL_RenderPresent(window_info_.renderer);
}

void System::UpdateScreenTexture() {
  int32_t* pixels;
  int32_t pitch;

  SDL_LockTexture(window_info_.screen_texture, NULL, (void**)&pixels, &pitch);

  size_t size = width_ * height_;
  int32_t* screen_data_ = graphics_->ScreenImage()->Data();

  for (size_t i = 0; i < size; i++) {
    pixels[i] = palette_color_[screen_data_[i]];
  }

  SDL_UnlockTexture(window_info_.screen_texture);
}

void System::UpdateWindowInfo() {
  SDL_GetWindowSize(window_info_.window, &window_info_.window_width,
                    &window_info_.window_height);

  window_info_.screen_scale =
      std::min((window_info_.window_width - border_width_ * 2) / width_,
               (window_info_.window_height - border_width_ * 2) / height_);

  window_info_.screen_x =
      (window_info_.window_width - width_ * window_info_.screen_scale) / 2;
  window_info_.screen_y =
      (window_info_.window_height - height_ * window_info_.screen_scale) / 2;
}

void System::RaiseError(const char* message) {
  //
}

}  // namespace pyxelcore
