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

  width_ = width;
  height_ = height;
  caption_ = caption ? std::string(caption) : DEFAULT_CAPTION;
  scale_ = scale != -1 ? scale : DEFAULT_SCALE;

  palette_color = palette_color ? palette_color : DEFAULT_PALETTE;
  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_color_[i] = palette_color[i];
  }

  fps_ = std::max(fps != -1 ? fps : DEFAULT_FPS, 1);
  border_width_ = border_width != -1 ? border_width : DEFAULT_BORDER_WIDTH;
  border_color_ = border_color != -1 ? border_color : DEFAULT_BORDER_COLOR;

  SDL_Init(SDL_INIT_VIDEO);

  window_ = SDL_CreateWindow(caption_.c_str(), SDL_WINDOWPOS_CENTERED,
                             SDL_WINDOWPOS_CENTERED, width, height, 0);
  renderer_ = SDL_CreateRenderer(window_, -1, 0);
  screen_texture_ =
      SDL_CreateTexture(renderer_, SDL_PIXELFORMAT_RGB888,
                        SDL_TEXTUREACCESS_STREAMING, width, height);
}

System::~System() {}

void System::Run(void (*update)(), void (*draw)()) {
  SDL_Event ev;

  double one_frame_time = 1000.0f / fps_;
  double next_update_time = SDL_GetTicks();

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
      while (SDL_PollEvent(&ev)) {
        if (ev.type == SDL_QUIT)
          return;
      }

      update();
    }

    draw();

    SDL_SetRenderDrawColor(renderer_, 0, 0, 0, 255);
    SDL_RenderClear(renderer_);

    UpdateScreenTexture();
    SDL_RenderCopy(renderer_, screen_texture_, NULL, NULL);

    SDL_RenderPresent(renderer_);
  }
}

void System::UpdateScreenTexture() {
  int32_t* pixel;
  int32_t pitch;
  size_t size = width_ * height_;

  SDL_LockTexture(screen_texture_, NULL, (void**)&pixel, &pitch);

  int32_t* framebuffer = graphics_->Framebuffer();

  for (size_t i = 0; i < size; i++) {
    pixel[i] = palette_color_[framebuffer[i]];
  }

  SDL_UnlockTexture(screen_texture_);
}

}  // namespace pyxelcore
