#include "pyxelcore/app.h"

#include "pyxelcore/image.h"

namespace pyxelcore {

void App::LoadImage(Image* image, int32_t x, int32_t y, const char* filename) {
  //
}

void App::LoadAsset(const char* filename) {
  // int32_t flags = IMG_INIT_PNG;
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
}

void App::SaveAsset(const char* filename) {
  //
}

}  // namespace pyxelcore
