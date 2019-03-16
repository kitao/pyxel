#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <cstdio>

#include "pyxelcore/graphics.h"
#include "pyxelcore/system.h"

namespace pyxelcore {

System::System(Graphics *graphics, int width, int height, char *caption,
               int scale, int *palette, int fps, int border_width,
               int border_color) {
  graphics_ = graphics;
  width_ = width;
  height_ = height;
  caption_ = std::string(caption);
  scale_ = scale;

  for (int i = 0; i < 16; i++) {
    palette_[i] = palette[i];
  }

  fps_ = std::max(fps, 1);
  border_width_ = border_width;
  border_color_ = border_color;

  SDL_Init(SDL_INIT_VIDEO);

  window_ = SDL_CreateWindow(caption_.c_str(), SDL_WINDOWPOS_CENTERED,
                             SDL_WINDOWPOS_CENTERED, width_, height_, 0);
  renderer_ = SDL_CreateRenderer(window_, -1, 0);

  // int flags = IMG_INIT_PNG;
  // int initted = IMG_Init(flags);
  // if ((initted & flags) != flags) {
  //  printf("IMG_Init: Failed to init required jpg and png support!\n");
  //  printf("IMG_Init: %s\n", IMG_GetError());
  //  return;
  // }

  // SDL_Surface *image = IMG_Load("../examples/assets/pyxel_logo_152x64.png");

  // if (!image) {
  //  printf("IMG_Load: %s\n", IMG_GetError());
  //  return;
  // }

  // temp_texture_ = SDL_CreateTextureFromSurface(renderer_, image);

  screen_texture_ =
      SDL_CreateTexture(renderer_, SDL_PIXELFORMAT_RGB888,
                        SDL_TEXTUREACCESS_STREAMING, width_, height_);
}

System::~System() {}

void System::run(void (*update)(), void (*draw)()) {
  SDL_Event ev;

  double one_frame_time = 1000.0f / fps_;
  double next_update_time = SDL_GetTicks();

  while (1) {
    double sleep_time = next_update_time - SDL_GetTicks();

    if (sleep_time > 0) {
      SDL_Delay(static_cast<int>(sleep_time / 2));
      continue;
    }

    int update_frame_count =
        std::min(static_cast<int>(-sleep_time / one_frame_time) + 1, 10);

    next_update_time += one_frame_time * update_frame_count;

    for (int i = 0; i < update_frame_count; i++) {
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

void System::quit() {}

void System::UpdateScreenTexture() {
  int *pixel;
  int pitch;
  size_t size = width_ * height_;

  SDL_LockTexture(screen_texture_, NULL, (void **)&pixel, &pitch);

  int *framebuffer = graphics_->Framebuffer();

  for (size_t i = 0; i < size; i++) {
    pixel[i] = palette_[framebuffer[i]];
  }

  SDL_UnlockTexture(screen_texture_);
}

} // namespace pyxelcore
