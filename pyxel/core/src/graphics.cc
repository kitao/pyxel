#include <algorithm>

#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"

namespace pyxelcore {

Graphics::Graphics(int width, int height) {
  width_ = width;
  height_ = height;

  framebuffer_ = new int[width_ * height_];

  for (int i = 0; i < 4; i++) {
    image_[i] = new Image(256, 256);
  }

  clip();
  pal();

  cls(0);
}

Graphics::~Graphics() {
  for (int i = 0; i < 4; i++) {
    delete image_[i];
  }

  delete framebuffer_;
}

void* Graphics::image(int img, int system) {
  return image_[img];
}

void* Graphics::tilemap(int tm) {
  return NULL;
}

void Graphics::clip() {
  clip_x1_ = 0;
  clip_y1_ = 0;
  clip_x2_ = width_ - 1;
  clip_y2_ = height_ - 1;
}

void Graphics::clip(int x1, int y1, int x2, int y2) {
  clip_x1_ = std::max(x1, 0);
  clip_y1_ = std::max(y1, 0);
  clip_x2_ = std::min(x2, width_ - 1);
  clip_y2_ = std::min(y2, height_ - 1);
}

void Graphics::pal() {
  for (int i; i < 16; i++) {
    pal_[i] = i;
  }
}

void Graphics::pal(int col1, int col2) {
  if (col1 < 0 || col1 > 15 || col2 < 0 || col2 > 15) {
    return;
  }

  pal_[col1] = col2;
}

void Graphics::cls(int col) {
  size_t size = width_ * height_;

  for (size_t i = 0; i < size; i++) {
    framebuffer_[i] = col;
  }
}

void Graphics::pix(int x, int y, int col) {
  if (x < 0 || x >= width_ || y < 0 || y >= height_) {
    return;
  }

  if (col < 0 || col > 15) {
    return;
  }

  framebuffer_[width_ * y + x] = col;
}

void Graphics::line(int x1, int y1, int x2, int y2, int col) {}

void Graphics::rect(int x1, int y1, int x2, int y2, int col) {
  if (col < 0 || col > 15) {
    return;
  }

  int left = std::max(std::min(x1, x2), 0);
  int top = std::max(std::min(y1, y2), 0);
  int right = std::min(std::max(x1, x2) + 1, width_);
  int bottom = std::min(std::max(y1, y2) + 1, height_);

  for (int i = top; i < bottom; i++) {
    size_t line_head = width_ * i;

    for (int j = left; j < right; j++) {
      framebuffer_[line_head + j] = col;
    }
  }
}

void Graphics::rectb(int x1, int y1, int x2, int y2, int col) {
  if (col < 0 || col > 15) {
    return;
  }

  int left = std::max(std::min(x1, x2), 0);
  int top = std::max(std::min(y1, y2), 0);
  int right = std::min(std::max(x1, x2) + 1, width_);
  int bottom = std::min(std::max(y1, y2) + 1, height_);

  if (x1 >= 0 && x1 < width_) {
    for (int i = top; i < bottom; i++) {
      framebuffer_[width_ * i + x1] = col;
    }
  }

  if (x2 >= 0 && x2 < width_) {
    for (int i = top; i < bottom; i++) {
      framebuffer_[width_ * i + x2] = col;
    }
  }

  if (y1 >= 0 && y1 < height_) {
    size_t line_head = width_ * y1;

    for (int i = left; i < right; i++) {
      framebuffer_[line_head + i] = col;
    }
  }

  if (y2 >= 0 && y2 < height_) {
    size_t line_head = width_ * y2;

    for (int i = left; i < right; i++) {
      framebuffer_[line_head + i] = col;
    }
  }
}

void Graphics::circ(int x, int y, int r, int col) {}

void Graphics::circb(int x, int y, int r, int col) {}

void Graphics::blt(int x,
                   int y,
                   int img,
                   int u,
                   int v,
                   int w,
                   int h,
                   int colkey) {
  // int iw, ih;
  // SDL_QueryTexture(temp_texture_, NULL, NULL, &iw, &ih);
  // SDL_Rect image_rect = (SDL_Rect){0, 0, iw, ih};
  // SDL_Rect draw_rect = (SDL_Rect){50, 50, iw, ih};
  // SDL_RenderCopy(renderer_, temp_texture_, &image_rect, &draw_rect);

  // SDL_SetRenderDrawColor(renderer_, 255, 255, 0, 255);
  // SDL_RenderDrawLine(renderer_, 10, 10, 400, 400);
}

void Graphics::bltm(int x,
                    int y,
                    int tm,
                    int u,
                    int v,
                    int w,
                    int h,
                    int colkey) {}

void Graphics::text(int x, int y, int s, int col) {}

}  // namespace pyxelcore
