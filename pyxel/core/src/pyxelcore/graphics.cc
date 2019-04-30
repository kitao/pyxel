#include "pyxelcore/graphics.h"

#include "pyxelcore/constants.h"
#include "pyxelcore/image.h"
#include "pyxelcore/utilities.h"

#include <cmath>

namespace pyxelcore {

const uint32_t FONT_DATA[] = {
    0x000000, 0x444040, 0xAA0000, 0xAEAEA0, 0x6C6C40, 0x824820, 0x4A4AC0,
    0x440000, 0x244420, 0x844480, 0xA4E4A0, 0x04E400, 0x000480, 0x00E000,
    0x000040, 0x224880, 0x6AAAC0, 0x4C4440, 0xC248E0, 0xC242C0, 0xAAE220,
    0xE8C2C0, 0x68EAE0, 0xE24880, 0xEAEAE0, 0xEAE2C0, 0x040400, 0x040480,
    0x248420, 0x0E0E00, 0x842480, 0xE24040, 0x4AA860, 0x4AEAA0, 0xCACAC0,
    0x688860, 0xCAAAC0, 0xE8E8E0, 0xE8E880, 0x68EA60, 0xAAEAA0, 0xE444E0,
    0x222A40, 0xAACAA0, 0x8888E0, 0xAEEAA0, 0xCAAAA0, 0x4AAA40, 0xCAC880,
    0x4AAE60, 0xCAECA0, 0x6842C0, 0xE44440, 0xAAAA60, 0xAAAA40, 0xAAEEA0,
    0xAA4AA0, 0xAA4440, 0xE248E0, 0x644460, 0x884220, 0xC444C0, 0x4A0000,
    0x0000E0, 0x840000, 0x06AA60, 0x8CAAC0, 0x068860, 0x26AA60, 0x06AC60,
    0x24E440, 0x06AE24, 0x8CAAA0, 0x404440, 0x2022A4, 0x8ACCA0, 0xC444E0,
    0x0EEEA0, 0x0CAAA0, 0x04AA40, 0x0CAAC8, 0x06AA62, 0x068880, 0x06C6C0,
    0x4E4460, 0x0AAA60, 0x0AAA40, 0x0AAEE0, 0x0A44A0, 0x0AA624, 0x0E24E0,
    0x64C460, 0x444440, 0xC464C0, 0x6C0000, 0xEEEEE0,
};

Graphics::Graphics(int32_t width, int32_t height) {
  screen_image_ = new Image(width, height);

  image_bank_ = new Image*[IMAGE_BANK_COUNT];
  for (int32_t i = 0; i < IMAGE_BANK_COUNT; i++) {
    image_bank_[i] = new Image(IMAGE_BANK_WIDTH, IMAGE_BANK_HEIGHT);
  }

  SetupFontImage();

  ResetClippingArea();
  ResetPalette();
  Clear(0);
}

Graphics::~Graphics() {
  for (int32_t i = 0; i < IMAGE_BANK_COUNT; i++) {
    delete image_bank_[i];
  }
  delete[] image_bank_;

  delete screen_image_;
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
  if (src_color < 0 || src_color >= COLOR_COUNT) {
    PRINT_ERROR("invalid color");
    return;
  }

  if (dest_color < 0 || dest_color >= COLOR_COUNT) {
    PRINT_ERROR("invalid color");
    return;
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

  if (Abs(x1 - x2) > Abs(y1 - y2)) {
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
                         int32_t image_index,
                         int32_t u,
                         int32_t v,
                         int32_t width,
                         int32_t height,
                         int32_t color_key) {
  Image* image = GetImageBank(image_index, true);
  Rectangle copy_rect = Rectangle::FromSize(u, v, width, height);

  screen_image_->DrawImage(x, y, image, copy_rect, clip_rect_, palette_table_,
                           color_key);
}

void Graphics::DrawTilemap(int32_t x,
                           int32_t y,
                           int32_t tilemap_index,
                           int32_t u,
                           int32_t v,
                           int32_t width,
                           int32_t height,
                           int32_t color_key) {
  Tilemap* tilemap = GetTilemapBank(tilemap_index);
  Rectangle copy_rect = Rectangle::FromSize(u, v, width, height);
  // TODO
}

void Graphics::DrawText(int32_t x, int32_t y, const char* text, int32_t color) {
  color = GetDrawColor(color);

  int32_t left = x;
  int32_t original_color = palette_table_[7];

  palette_table_[7] = color;

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

    if (*ch < MIN_FONT_CODE || *ch > MAX_FONT_CODE) {
      continue;
    }

    int32_t code = *ch - MIN_FONT_CODE;
    Rectangle copy_rect = Rectangle::FromSize(
        (code % FONT_ROW_COUNT) * FONT_WIDTH,
        (code / FONT_ROW_COUNT) * FONT_HEIGHT, FONT_WIDTH, FONT_HEIGHT);

    screen_image_->DrawImage(x, y, image_bank_[IMAGE_BANK_FOR_SYSTEM],
                             copy_rect, clip_rect_, palette_table_, 0);

    x += FONT_WIDTH;
  }

  palette_table_[7] = original_color;
}

void Graphics::SetupFontImage() {
  const int32_t FONT_COUNT = sizeof(FONT_DATA) / sizeof(FONT_DATA[0]);
  int32_t* data = image_bank_[IMAGE_BANK_FOR_SYSTEM]->Data();

  for (int32_t i = 0; i < FONT_COUNT; i++) {
    int32_t row = i / FONT_ROW_COUNT;
    int32_t col = i % FONT_ROW_COUNT;
    int32_t index = IMAGE_BANK_WIDTH * FONT_HEIGHT * row + FONT_WIDTH * col;
    uint32_t font = FONT_DATA[i];

    for (int32_t j = 0; j < FONT_HEIGHT; j++) {
      for (int32_t k = 0; k < FONT_WIDTH; k++) {
        data[index + k] = (font & 0x800000) ? 7 : 0;
        font <<= 1;
      }

      index += IMAGE_BANK_WIDTH;
    }
  }
}

}  // namespace pyxelcore
