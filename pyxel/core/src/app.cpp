#include "pyxelcore/app.h"
#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <cstdio>

namespace pyxelcore {

App::App(int width, int height, char *caption, int scale, int *palette, int fps,
         int border_width, int border_color) {
  SDL_Init(SDL_INIT_VIDEO);

  window_ = SDL_CreateWindow(caption, SDL_WINDOWPOS_CENTERED,
                             SDL_WINDOWPOS_CENTERED, width, height, 0);
  renderer_ = SDL_CreateRenderer(window_, -1, 0);

  int flags = IMG_INIT_PNG;
  int initted = IMG_Init(flags);
  if ((initted & flags) != flags) {
    printf("IMG_Init: Failed to init required jpg and png support!\n");
    printf("IMG_Init: %s\n", IMG_GetError());
    return;
  }

  SDL_Surface *image = IMG_Load("../examples/assets/pyxel_logo_152x64.png");

  if (!image) {
    printf("IMG_Load: %s\n", IMG_GetError());
    return;
  }

  temp_texture_ = SDL_CreateTextureFromSurface(renderer_, image);
}

void App::Run(void (*update)(), void (*draw)()) {
  SDL_Event ev;

  while (1) {
    SDL_SetRenderDrawColor(renderer_, 0, 0, 0, 255);
    SDL_RenderClear(renderer_);

    while (SDL_PollEvent(&ev)) {
      if (ev.type == SDL_QUIT)
        return;
    }

    int iw, ih;
    SDL_QueryTexture(temp_texture_, NULL, NULL, &iw, &ih);
    SDL_Rect image_rect = (SDL_Rect){0, 0, iw, ih};
    SDL_Rect draw_rect = (SDL_Rect){50, 50, iw, ih};
    SDL_RenderCopy(renderer_, temp_texture_, &image_rect, &draw_rect);

    SDL_SetRenderDrawColor(renderer_, 255, 0, 0, 255);
    SDL_RenderDrawLine(renderer_, 10, 10, 400, 400);

    SDL_RenderPresent(renderer_);
  }
}

} // namespace pyxelcore
