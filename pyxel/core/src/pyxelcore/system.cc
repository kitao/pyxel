#include "pyxelcore/system.h"

#include "pyxelcore/audio.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"
#include "pyxelcore/input.h"
#include "pyxelcore/resource.h"

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <algorithm>
#include <cstdio>

namespace pyxelcore {

const char* ICON_DATA[] = {
    "0000000110000000", "0000011F71100000", "00011FF11FF11000",
    "011FF111111FF110", "1AE1111111111C71", "1E1EE111111CC1C1",
    "1E111EE11CC111C1", "1E11111E711111C1", "1E111111C11111C1",
    "1E111111C11111C1", "1E111111C11111C1", "1AE11111C1111C71",
    "011EE111C11CC110", "00011EE1CCC11000", "0000011E71100000",
    "0000000110000000",
};

const char* MOUSE_CURSOR_DATA[] = {
    "00000011", "07776011", "07760111", "07676011",
    "06067601", "00106760", "11110601", "11111011",
};

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

  fps_ = std::max(fps, 1);
  border_width_ = std::max(DEFAULT_BORDER_WIDTH, 0);
  border_color_ = std::max(DEFAULT_BORDER_COLOR, 0);
  frame_count_ = 0;

  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_color_[i] = palette_color[i];
  }

  SetupWindow(caption, width, height, scale);
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
          if (event.window.event == SDL_WINDOWEVENT_MOVED ||
              event.window.event == SDL_WINDOWEVENT_RESIZED) {
            UpdateWindowInfo();
          }
        }
      }

      if (is_first_frame) {
        is_first_frame = false;
      } else {
        frame_count_++;
      }

      input_->UpdateState(&window_info_, frame_count_);
      update();
    }

    draw();
    RenderWindow();
  }
}

void System::SetupWindow(const char* caption,
                         int32_t width,
                         int32_t height,
                         int32_t scale) {
  SDL_Init(SDL_INIT_VIDEO);  // TODO: error handling
  IMG_Init(IMG_INIT_PNG);    // TODO: erro handling

  window_info_.screen_width = std::max(width, 1);
  window_info_.screen_height = std::max(height, 1);

  if (scale > 0) {
    window_info_.screen_scale = scale;
  } else {
    SDL_DisplayMode display_mode;
    SDL_GetDesktopDisplayMode(0, &display_mode);
    window_info_.screen_scale = std::min(
        (display_mode.w - border_width_ * 2) / window_info_.screen_width,
        (display_mode.h - border_width_ * 2) / window_info_.screen_height);
  }

  window_info_.window =
      SDL_CreateWindow(caption, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                       window_info_.screen_width * window_info_.screen_scale,
                       window_info_.screen_height * window_info_.screen_scale,
                       SDL_WINDOW_RESIZABLE);
  window_info_.renderer = SDL_CreateRenderer(window_info_.window, -1, 0);
  window_info_.screen_texture =
      SDL_CreateTexture(window_info_.renderer, SDL_PIXELFORMAT_RGB888,
                        SDL_TEXTUREACCESS_STREAMING, window_info_.screen_width,
                        window_info_.screen_height);

  SDL_SetWindowMinimumSize(window_info_.window, window_info_.screen_width,
                           window_info_.screen_height);

  UpdateWindowInfo();
}

void System::RenderWindow() {
  SDL_SetRenderDrawColor(window_info_.renderer, (border_color_ >> 16) & 0xff,
                         (border_color_ >> 8) & 0xff, border_color_ & 0xff,
                         255);
  SDL_RenderClear(window_info_.renderer);

  UpdateScreenTexture();

  SDL_Rect dest_rect = {window_info_.screen_x, window_info_.screen_y,
                        window_info_.screen_width * window_info_.screen_scale,
                        window_info_.screen_height * window_info_.screen_scale};
  SDL_RenderCopy(window_info_.renderer, window_info_.screen_texture, NULL,
                 &dest_rect);

  SDL_RenderPresent(window_info_.renderer);
}

void System::UpdateScreenTexture() {
  int32_t* pixels;
  int32_t pitch;

  SDL_LockTexture(window_info_.screen_texture, NULL, (void**)&pixels, &pitch);

  int32_t size = window_info_.screen_width * window_info_.screen_height;
  int32_t* screen_data_ = graphics_->ScreenImage()->Data();

  for (int32_t i = 0; i < size; i++) {
    pixels[i] = palette_color_[screen_data_[i]];
  }

  SDL_UnlockTexture(window_info_.screen_texture);
}

void System::UpdateWindowInfo() {
  SDL_GetWindowPosition(window_info_.window, &window_info_.window_x,
                        &window_info_.window_y);
  SDL_GetWindowSize(window_info_.window, &window_info_.window_width,
                    &window_info_.window_height);

  window_info_.screen_scale =
      std::min((window_info_.window_width - border_width_ * 2) /
                   window_info_.screen_width,
               (window_info_.window_height - border_width_ * 2) /
                   window_info_.screen_height);

  window_info_.screen_x =
      (window_info_.window_width -
       window_info_.screen_width * window_info_.screen_scale) /
      2;
  window_info_.screen_y =
      (window_info_.window_height -
       window_info_.screen_height * window_info_.screen_scale) /
      2;
}

void System::RaiseError(const char* message) {
  //
}

}  // namespace pyxelcore
