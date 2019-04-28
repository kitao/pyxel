#include "pyxelcore/graphics.h"

#include "pyxelcore/constants.h"
#include "pyxelcore/image.h"

#include <cmath>

namespace pyxelcore {

Graphics::Graphics(int32_t width, int32_t height) {
  screen_image_ = new Image(width, height);

  image_bank_ = new Image*[IMAGE_COUNT];
  for (int32_t i = 0; i < IMAGE_COUNT; i++) {
    image_bank_[i] = new Image(IMAGE_WIDTH, IMAGE_HEIGHT);
  }

  SetupFontImage();

  ResetClippingArea();
  ResetPalette();
  Clear(0);
}

Graphics::~Graphics() {
  for (int32_t i = 0; i < IMAGE_COUNT; i++) {
    delete image_bank_[i];
  }
  delete[] image_bank_;

  delete screen_image_;
}

Image* Graphics::GetImage(int32_t image_index, bool system) const {
  if (image_index < 0 || image_index >= IMAGE_COUNT) {
    // error
  }

  if (image_index == IMAGE_FOR_SYSTEM && !system) {
    // error
  }

  return image_bank_[image_index];
}

Tilemap* Graphics::GetTilemap(int32_t tilemap_index) const {
  if (tilemap_index < 0 || tilemap_index >= TILEMAP_COUNT) {
    // error
  }

  return tilemap_bank_[tilemap_index];
}

void Graphics::ResetClippingArea() {
  clip_rect_ = *static_cast<Rectangle*>(screen_image_);
}

void Graphics::SetClippingArea(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  clip_rect_ = Rectangle::FromPos(x1, y1, x2, y2).Intersect(*screen_image_);
}

void Graphics::ResetPalette() {
  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_table_[i] = i;
  }
}

void Graphics::SetPalette(int32_t src_color, int32_t dest_color) {
  if (src_color < 0 || src_color >= COLOR_COUNT || dest_color < 0 ||
      dest_color >= COLOR_COUNT) {
    // error
  }

  palette_table_[src_color] = dest_color;
}

void Graphics::Clear(int32_t color) {
  color = GetDrawColor(color);

  int32_t size = screen_image_->Width() * screen_image_->Height();
  int32_t* data = screen_image_->Data();

  for (int32_t i = 0; i < size; i++) {
    data[i] = color;
  }
}

void Graphics::DrawPoint(int32_t x, int32_t y, int32_t color) {
  SetPixel(x, y, GetDrawColor(color));
}

void Graphics::DrawLine(int32_t x1,
                        int32_t y1,
                        int32_t x2,
                        int32_t y2,
                        int32_t color) {
  color = GetDrawColor(color);

  if (x1 == x2 && y1 == y2) {
    SetPixel(x1, y1, color);
    return;
  }

  if (std::abs(x1 - x2) > std::abs(y1 - y2)) {
    int32_t start_x, start_y;
    int32_t end_x, end_y;

    if (x1 < x2) {
      start_x = x1;
      start_y = y1;
      end_x = x2;
      end_y = y2;
    } else {
      start_x = x2;
      start_y = y2;
      end_x = x1;
      end_y = y1;
    }

    int32_t length = end_x - start_x + 1;
    float alpha = static_cast<float>((end_y - start_y)) /
                  static_cast<float>((end_x - start_x));

    for (int32_t i = 0; i < length; i++) {
      SetPixel(start_x + i, start_y + alpha * i + 0.5f, color);
    }
  } else {
    int32_t start_x, start_y;
    int32_t end_x, end_y;

    if (y1 < y2) {
      start_x = x1;
      start_y = y1;
      end_x = x2;
      end_y = y2;
    } else {
      start_x = x2;
      start_y = y2;
      end_x = x1;
      end_y = y1;
    }

    int32_t length = end_y - start_y + 1;
    float alpha = static_cast<float>((end_x - start_x)) /
                  static_cast<float>((end_y - start_y));

    for (int32_t i = 0; i < length; i++) {
      SetPixel(start_x + alpha * i + 0.5f, start_y + i, color);
    }
  }
}

void Graphics::DrawRectangle(int32_t x1,
                             int32_t y1,
                             int32_t x2,
                             int32_t y2,
                             int32_t color) {
  color = GetDrawColor(color);

  Rectangle draw_rect =
      Rectangle::FromPos(x1, y1, x2, y2).Intersect(clip_rect_);

  if (draw_rect.IsEmpty()) {
    return;
  }

  int32_t* data = screen_image_->Data();

  for (int32_t i = draw_rect.Top(); i <= draw_rect.Bottom(); i++) {
    int32_t index = screen_image_->Width() * i;

    for (int32_t j = draw_rect.Left(); j <= draw_rect.Right(); j++) {
      data[index + j] = color;
    }
  }
}

void Graphics::DrawRectangleBorder(int32_t x1,
                                   int32_t y1,
                                   int32_t x2,
                                   int32_t y2,
                                   int32_t color) {
  color = GetDrawColor(color);

  Rectangle draw_rect = Rectangle::FromPos(x1, y1, x2, y2);

  if (draw_rect.Intersect(clip_rect_).IsEmpty()) {
    return;
  }

  for (int32_t i = draw_rect.Left(); i <= draw_rect.Right(); i++) {
    SetPixel(i, y1, color);
    SetPixel(i, y2, color);
  }

  for (int32_t i = draw_rect.Top(); i <= draw_rect.Bottom(); i++) {
    SetPixel(x1, i, color);
    SetPixel(x2, i, color);
  }
}

void Graphics::DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color) {
  color = GetDrawColor(color);

  if (radius == 0) {
    SetPixel(x, y, color);
    return;
  }

  for (int32_t dx = 0; dx <= radius; dx++) {
    int32_t dy = std::sqrt(radius * radius - dx * dx) + 0.5f;

    if (dx > dy) {
      continue;
    }

    for (int32_t i = -dy; i <= dy; i++) {
      SetPixel(x - dx, y + i, color);
      SetPixel(x + dx, y + i, color);
      SetPixel(x + i, y - dx, color);
      SetPixel(x + i, y + dx, color);
    }
  }
}

void Graphics::DrawCircleBorder(int32_t x,
                                int32_t y,
                                int32_t radius,
                                int32_t color) {
  color = GetDrawColor(color);

  if (radius == 0) {
    SetPixel(x, y, color);
    return;
  }

  for (int32_t dx = 0; dx <= radius; dx++) {
    int32_t dy = std::sqrt(radius * radius - dx * dx) + 0.5f;

    if (dx > dy) {
      continue;
    }

    SetPixel(x - dx, y - dy, color);
    SetPixel(x + dx, y - dy, color);
    SetPixel(x - dx, y + dy, color);
    SetPixel(x + dx, y + dy, color);

    SetPixel(x - dy, y - dx, color);
    SetPixel(x + dy, y - dx, color);
    SetPixel(x - dy, y + dx, color);
    SetPixel(x + dy, y + dx, color);
  }
}

void Graphics::DrawImage(int32_t x,
                         int32_t y,
                         const Image* image,
                         const Rectangle& copy_rect,
                         int32_t color_key) {
  screen_image_->CopyImage(x, y, image, copy_rect, clip_rect_, palette_table_,
                           color_key);
}

void Graphics::DrawTilemap(int32_t x,
                           int32_t y,
                           const Tilemap* tilemap,
                           const Rectangle& copy_rect,
                           int32_t colkey) {
  //
}

void Graphics::DrawText(int32_t x, int32_t y, const char* text, int32_t color) {
  color = GetDrawColor(color);

  int32_t left = x;

  for (const char* ch = text; *ch != '\0'; ch++) {
    if (*ch == 10) {  // new line
      x = left;
      y += FONT_HEIGHT;

      continue;
    }

    if (*ch == 32) {  // space
      x += FONT_WIDTH;

      continue;
    }

    if (*ch < FONT_MIN_CODE || *ch > FONT_MAX_CODE) {
      continue;
    }

    int32_t code = *ch - FONT_MIN_CODE;
    Rectangle copy_rect = Rectangle::FromSize(
        (code % FONT_ROW_COUNT) * FONT_WIDTH,
        (code / FONT_ROW_COUNT) * FONT_HEIGHT, FONT_WIDTH, FONT_HEIGHT);

    screen_image_->CopyImage(x, y, image_bank_[IMAGE_FOR_SYSTEM], copy_rect,
                             clip_rect_, palette_table_);

    x += FONT_WIDTH;
  }
}

void Graphics::SetupFontImage() {
  const int32_t FONT_COUNT = sizeof(FONT_DATA) / sizeof(FONT_DATA[0]);
  int32_t* data = image_bank_[IMAGE_FOR_SYSTEM]->Data();

  for (int32_t i = 0; i < FONT_COUNT; i++) {
    int32_t row = i / FONT_ROW_COUNT;
    int32_t col = i % FONT_ROW_COUNT;
    int32_t index = IMAGE_WIDTH * FONT_HEIGHT * row + FONT_WIDTH * col;
    uint32_t font = FONT_DATA[i];

    for (int32_t j = 0; j < FONT_HEIGHT; j++) {
      for (int32_t k = 0; k < FONT_WIDTH; k++) {
        data[index + k] = (font & 0x800000) ? 7 : 0;
        font <<= 1;
      }

      index += IMAGE_WIDTH;
    }
  }
}

int32_t Graphics::GetDrawColor(int32_t color) const {
  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  return palette_table_[color];
}

}  // namespace pyxelcore
