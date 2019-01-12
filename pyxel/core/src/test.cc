#include "pyxelcore.h"
#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>
#include <stdio.h>

int test(int width, int height) {
  SDL_Init(SDL_INIT_VIDEO);

  SDL_Window *window = SDL_CreateWindow(
      "Hey", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, width, height, 0);
  SDL_Renderer *renderer = SDL_CreateRenderer(window, -1, 0);

  int flags = IMG_INIT_PNG;
  int initted = IMG_Init(flags);
  if ((initted & flags) != flags) {
    printf("IMG_Init: Failed to init required jpg and png support!\n");
    printf("IMG_Init: %s\n", IMG_GetError());
  }

  SDL_Surface *image = IMG_Load("../examples/assets/pyxel_logo_152x64.png");

  if (!image) {
    printf("IMG_Load: %s\n", IMG_GetError());
  }

  SDL_Texture *image_texture;
  image_texture = SDL_CreateTextureFromSurface(renderer, image);

  SDL_Event ev;
  while (1) {
    SDL_SetRenderDrawColor(renderer, 0, 0, 0, 255);
    SDL_RenderClear(renderer);

    while (SDL_PollEvent(&ev)) {
      if (ev.type == SDL_QUIT)
        return 0;
    }

    int iw, ih;
    SDL_QueryTexture(image_texture, NULL, NULL, &iw, &ih);
    SDL_Rect image_rect = (SDL_Rect){0, 0, iw, ih};
    SDL_Rect draw_rect = (SDL_Rect){50, 50, iw, ih};
    SDL_RenderCopy(renderer, image_texture, &image_rect, &draw_rect);

    SDL_SetRenderDrawColor(renderer, 255, 0, 0, 255);
    SDL_RenderDrawLine(renderer, 10, 10, 400, 400);

    SDL_RenderPresent(renderer);
  }

  return 0;
}
