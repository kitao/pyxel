#include "pyxelcore/graphics.h"

#include "pyxelcore/constants.h"
#include "pyxelcore/image.h"

#include <algorithm>

namespace pyxelcore {

Graphics::Graphics(int32_t width, int32_t height) {
  screen_ = new Image(width, height, COLOR_COUNT);

  image_bank_ = new Image*[IMAGE_BANK_COUNT];

  for (int32_t i = 0; i < IMAGE_BANK_COUNT; i++) {
    image_bank_[i] =
        new Image(IMAGE_BANK_WIDTH, IMAGE_BANK_HEIGHT, COLOR_COUNT);
  }
}

Graphics::~Graphics() {
  for (int32_t i = 0; i < IMAGE_BANK_COUNT; i++) {
    delete image_bank_[i];
  }

  delete[] image_bank_;
  delete screen_;
}

Image* Graphics::GetImage(int32_t img, bool system) {
  if (img < 0 || img >= IMAGE_BANK_COUNT) {
    // error
  }

  if (img == IMAGE_BANK_COUNT - 1 and !system) {
    // error
  }

  return image_bank_[img];
}

Tilemap* Graphics::GetTilemap(int32_t tm) {
  return NULL;
}

void Graphics::DrawTilemap(int32_t x,
                           int32_t y,
                           int32_t tm,
                           int32_t u,
                           int32_t v,
                           int32_t width,
                           int32_t height,
                           int32_t colkey) {
  //
}

void Graphics::DrawText(int32_t x, int32_t y, const char* text, int32_t color) {
  //
}

}  // namespace pyxelcore
