#include <algorithm>

#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"

namespace pyxelcore {

Graphics::Graphics(int32_t width, int32_t height) {
  width_ = width;
  height_ = height;

  framebuffer_ = new int32_t[width_ * height_];

  for (int32_t i = 0; i < 4; i++) {
    image_[i] = new pyxelcore::Image(256, 256);
  }

  Clip();
  Pal();

  Cls(0);
}

Graphics::~Graphics() {
  for (int32_t i = 0; i < 4; i++) {
    delete image_[i];
  }

  delete framebuffer_;
}

void* Graphics::Image(int32_t img, int32_t system) {
  return image_[img];
}

void* Graphics::Tilemap(int32_t tm) {
  return NULL;
}

void Graphics::Clip() {
  clip_x1_ = 0;
  clip_y1_ = 0;
  clip_x2_ = width_ - 1;
  clip_y2_ = height_ - 1;
}

void Graphics::Clip(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  clip_x1_ = std::max(x1, 0);
  clip_y1_ = std::max(y1, 0);
  clip_x2_ = std::min(x2, width_ - 1);
  clip_y2_ = std::min(y2, height_ - 1);
}

void Graphics::Pal() {
  for (int32_t i; i < 16; i++) {
    pal_[i] = i;
  }
}

void Graphics::Pal(int32_t col1, int32_t col2) {
  if (col1 < 0 || col1 > 15 || col2 < 0 || col2 > 15) {
    return;
  }

  pal_[col1] = col2;
}

void Graphics::Cls(int32_t col) {
  size_t size = width_ * height_;

  for (size_t i = 0; i < size; i++) {
    framebuffer_[i] = col;
  }
}

void Graphics::Pix(int32_t x, int32_t y, int32_t col) {
  if (x < 0 || x >= width_ || y < 0 || y >= height_) {
    return;
  }

  if (col < 0 || col > 15) {
    return;
  }

  framebuffer_[width_ * y + x] = col;
}

void Graphics::Line(int32_t x1,
                    int32_t y1,
                    int32_t x2,
                    int32_t y2,
                    int32_t col) {}

void Graphics::Rect(int32_t x1,
                    int32_t y1,
                    int32_t x2,
                    int32_t y2,
                    int32_t col) {
  if (col < 0 || col > 15) {
    return;
  }

  int32_t left = std::max(std::min(x1, x2), 0);
  int32_t top = std::max(std::min(y1, y2), 0);
  int32_t right = std::min(std::max(x1, x2) + 1, width_);
  int32_t bottom = std::min(std::max(y1, y2) + 1, height_);

  for (int32_t i = top; i < bottom; i++) {
    size_t line_head = width_ * i;

    for (int32_t j = left; j < right; j++) {
      framebuffer_[line_head + j] = col;
    }
  }
}

void Graphics::Rectb(int32_t x1,
                     int32_t y1,
                     int32_t x2,
                     int32_t y2,
                     int32_t col) {
  if (col < 0 || col > 15) {
    return;
  }

  int32_t left = std::max(std::min(x1, x2), 0);
  int32_t top = std::max(std::min(y1, y2), 0);
  int32_t right = std::min(std::max(x1, x2) + 1, width_);
  int32_t bottom = std::min(std::max(y1, y2) + 1, height_);

  if (x1 >= 0 && x1 < width_) {
    for (int32_t i = top; i < bottom; i++) {
      framebuffer_[width_ * i + x1] = col;
    }
  }

  if (x2 >= 0 && x2 < width_) {
    for (int32_t i = top; i < bottom; i++) {
      framebuffer_[width_ * i + x2] = col;
    }
  }

  if (y1 >= 0 && y1 < height_) {
    size_t line_head = width_ * y1;

    for (int32_t i = left; i < right; i++) {
      framebuffer_[line_head + i] = col;
    }
  }

  if (y2 >= 0 && y2 < height_) {
    size_t line_head = width_ * y2;

    for (int32_t i = left; i < right; i++) {
      framebuffer_[line_head + i] = col;
    }
  }
}

void Graphics::Circ(int32_t x, int32_t y, int32_t r, int32_t col) {}

void Graphics::Circb(int32_t x, int32_t y, int32_t r, int32_t col) {}

void Graphics::Blt(int32_t x,
                   int32_t y,
                   int32_t img,
                   int32_t u,
                   int32_t v,
                   int32_t w,
                   int32_t h,
                   int32_t colkey) {
  // int32_t iw, ih;
  // SDL_QueryTexture(temp_texture_, NULL, NULL, &iw, &ih);
  // SDL_Rect image_rect = (SDL_Rect){0, 0, iw, ih};
  // SDL_Rect draw_rect = (SDL_Rect){50, 50, iw, ih};
  // SDL_RenderCopy(renderer_, temp_texture_, &image_rect, &draw_rect);

  // SDL_SetRenderDrawColor(renderer_, 255, 255, 0, 255);
  // SDL_RenderDrawLine(renderer_, 10, 10, 400, 400);
}

void Graphics::Bltm(int32_t x,
                    int32_t y,
                    int32_t tm,
                    int32_t u,
                    int32_t v,
                    int32_t w,
                    int32_t h,
                    int32_t colkey) {}

void Graphics::Text(int32_t x, int32_t y, int32_t s, int32_t col) {}

}  // namespace pyxelcore
