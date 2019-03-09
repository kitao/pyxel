#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <cstdio>

#include "pyxelcore/app.h"
#include "pyxelcore/graphics.h"

namespace pyxelcore {

void App::InitializeSystem() {
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

void App::TerminateSystem() {}

void App::run(void (*update)(), void (*draw)()) {
  SDL_Event ev;

  while (1) {
    while (SDL_PollEvent(&ev)) {
      if (ev.type == SDL_QUIT)
        return;
    }

    update();

    draw();

    SDL_SetRenderDrawColor(renderer_, 0, 0, 0, 255);
    SDL_RenderClear(renderer_);

    UpdateScreenTexture();
    SDL_RenderCopy(renderer_, screen_texture_, NULL, NULL);

    // int iw, ih;
    // SDL_QueryTexture(temp_texture_, NULL, NULL, &iw, &ih);
    // SDL_Rect image_rect = (SDL_Rect){0, 0, iw, ih};
    // SDL_Rect draw_rect = (SDL_Rect){50, 50, iw, ih};
    // SDL_RenderCopy(renderer_, temp_texture_, &image_rect, &draw_rect);

    // SDL_SetRenderDrawColor(renderer_, 255, 255, 0, 255);
    // SDL_RenderDrawLine(renderer_, 10, 10, 400, 400);

    SDL_RenderPresent(renderer_);
  }
}

void App::quit() {}

void App::UpdateScreenTexture() {
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
