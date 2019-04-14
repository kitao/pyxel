#include "pyxelcore/system.h"

#include "pyxelcore/audio.h"
#include "pyxelcore/constants.h"
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

  scale_ = scale != 0 ? scale : std::max(DEFAULT_SCALE, 1);
  if (scale_ == 0) {
    SDL_DisplayMode display_mode;
    SDL_GetDesktopDisplayMode(0, &display_mode);
    scale_ = std::min((display_mode.w - border_width_ * 2) / width_,
                      (display_mode.h - border_width_ * 2) / height_);
  }

  palette_color = palette_color ? palette_color : DEFAULT_PALETTE;
  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_color_[i] = palette_color[i];
  }

  fps_ = std::max(fps != -1 ? fps : DEFAULT_FPS, 1);
  border_width_ = border_width != -1 ? border_width : DEFAULT_BORDER_WIDTH;
  border_color_ = border_color != -1 ? border_color : DEFAULT_BORDER_COLOR;
  frame_count_ = 0;

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
            window_width_ = event.window.data1;
            window_height_ = event.window.data2;

            scale_ = std::min((window_width_ - border_width_ * 2) / width_,
                              (window_height_ - border_width_ * 2) / height_);
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

  window_ = SDL_CreateWindow(caption_.c_str(), SDL_WINDOWPOS_CENTERED,
                             SDL_WINDOWPOS_CENTERED, width_ * scale_,
                             height_ * scale_, SDL_WINDOW_RESIZABLE);
  renderer_ = SDL_CreateRenderer(window_, -1, 0);
  screen_texture_ =
      SDL_CreateTexture(renderer_, SDL_PIXELFORMAT_RGB888,
                        SDL_TEXTUREACCESS_STREAMING, width_, height_);

  SDL_SetWindowMinimumSize(window_, width_, height_);

  SDL_GetWindowSize(window_, &window_width_, &window_height_);
}

void System::RenderWindow() {
  SDL_SetRenderDrawColor(renderer_, (border_color_ >> 16) & 0xff,
                         (border_color_ >> 8) & 0xff, border_color_ & 0xff,
                         255);
  SDL_RenderClear(renderer_);

  UpdateScreenTexture();

  SDL_Rect dest_rect = {(window_width_ - width_ * scale_) / 2,
                        (window_height_ - height_ * scale_) / 2,
                        width_ * scale_, height_ * scale_};
  SDL_RenderCopy(renderer_, screen_texture_, NULL, &dest_rect);

  SDL_RenderPresent(renderer_);
}

void System::UpdateScreenTexture() {
  int32_t* pixels;
  int32_t pitch;

  SDL_LockTexture(screen_texture_, NULL, (void**)&pixels, &pitch);

  size_t size = width_ * height_;
  int32_t* screen_data_ = graphics_->Screen()->Data();

  for (size_t i = 0; i < size; i++) {
    pixels[i] = palette_color_[screen_data_[i]];
  }

  SDL_UnlockTexture(screen_texture_);
}

}  // namespace pyxelcore
