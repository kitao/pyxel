#include "pyxelcore/graphics.h"

#include "pyxelcore/image.h"
#include "pyxelcore/tilemap.h"

#include <cmath>

namespace pyxelcore {

Graphics::Graphics(int32_t width, int32_t height) {
  screen_image_ = new Image(width, height);

  image_bank_ = new Image*[IMAGE_BANK_COUNT];
  for (int32_t i = 0; i < IMAGE_BANK_COUNT; i++) {
    image_bank_[i] = new Image(IMAGE_BANK_WIDTH, IMAGE_BANK_HEIGHT);
  }

  tilemap_bank_ = new Tilemap*[TILEMAP_BANK_COUNT];
  for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
    tilemap_bank_[i] = new Tilemap(TILEMAP_BANK_WIDTH, TILEMAP_BANK_HEIGHT);
  }

  SetupMouseCursor();
  SetupFont();

  ResetClipArea();
  ResetPalette();
  ClearScreen(0);
}

Graphics::~Graphics() {
  for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
    delete tilemap_bank_[i];
  }
  delete[] tilemap_bank_;

  for (int32_t i = 0; i < IMAGE_BANK_COUNT; i++) {
    delete image_bank_[i];
  }
  delete[] image_bank_;

  delete screen_image_;
}

void Graphics::ResetClipArea() {
  clip_area_ = screen_image_->Rectangle();
}

void Graphics::SetClipArea(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  clip_area_ =
      Rectangle::FromPos(x1, y1, x2, y2).Intersect(screen_image_->Rectangle());
}

void Graphics::ResetPalette() {
  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_table_[i] = i;
  }
}

void Graphics::SetPalette(int32_t src_color, int32_t dst_color) {
  if (src_color < 0 || src_color >= COLOR_COUNT || dst_color < 0 ||
      dst_color >= COLOR_COUNT) {
    PRINT_ERROR("invalid color");
    return;
  }

  palette_table_[src_color] = dst_color;
}

void Graphics::ClearScreen(int32_t color) {
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
    float alpha = static_cast<float>(end_y - start_y) / (end_x - start_x);

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
    float alpha = static_cast<float>(end_x - start_x) / (end_y - start_y);

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
      Rectangle::FromPos(x1, y1, x2, y2).Intersect(clip_area_);

  if (draw_rect.IsEmpty()) {
    return;
  }

  int32_t left = draw_rect.Left();
  int32_t top = draw_rect.Top();
  int32_t right = draw_rect.Right();
  int32_t bottom = draw_rect.Bottom();
  int32_t width = screen_image_->Width();
  int32_t* data = screen_image_->Data();

  for (int32_t i = top; i <= bottom; i++) {
    int32_t index = width * i;

    for (int32_t j = left; j <= right; j++) {
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

  if (draw_rect.Intersect(clip_area_).IsEmpty()) {
    return;
  }

  int32_t left = draw_rect.Left();
  int32_t top = draw_rect.Top();
  int32_t right = draw_rect.Right();
  int32_t bottom = draw_rect.Bottom();

  for (int32_t i = left; i <= right; i++) {
    SetPixel(i, y1, color);
    SetPixel(i, y2, color);
  }

  for (int32_t i = top; i <= bottom; i++) {
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

  int32_t sq_radius = radius * radius;

  for (int32_t dx = 0; dx <= radius; dx++) {
    int32_t dy = std::sqrt(sq_radius - dx * dx) + 0.5f;

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

  int32_t sq_radius = radius * radius;

  for (int32_t dx = 0; dx <= radius; dx++) {
    int32_t dy = std::sqrt(sq_radius - dx * dx) + 0.5f;

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

  if (color_key != -1 && (color_key < 0 || color_key >= COLOR_COUNT)) {
    PRINT_ERROR("invalid color");
    color_key = -1;
  }

  Rectangle dst_rect = screen_image_->Rectangle().Intersect(clip_area_);
  Rectangle copy_rect = Rectangle::FromSize(u, v, Abs(width), Abs(height));
  Rectangle::CopyArea copy_area =
      dst_rect.GetCopyArea(x, y, image->Rectangle(), copy_rect);

  int32_t copy_width = copy_area.copy_width;
  int32_t copy_height = copy_area.copy_height;

  if (copy_width <= 0 || copy_height <= 0) {
    return;
  }

  int32_t src_x = copy_area.src_x;
  int32_t src_y = copy_area.src_y;
  int32_t src_width = image->Width();
  int32_t* src_data = image->Data();

  int32_t dst_x = copy_area.dst_x;
  int32_t dst_y = copy_area.dst_y;
  int32_t dst_width = screen_image_->Width();
  int32_t* dst_data = screen_image_->Data();

  int32_t sign_x, sign_y;
  int32_t offset_x, offset_y;

  if (width < 0) {
    width = -width;
    sign_x = -1;
    offset_x = width - 1;
  } else {
    sign_x = 1;
    offset_x = 0;
  }

  if (height < 0) {
    height = -height;
    sign_y = -1;
    offset_y = height - 1;
  } else {
    sign_y = 1;
    offset_y = 0;
  }

  if (color_key == -1) {
    for (int32_t i = 0; i < copy_height; i++) {
      int32_t src_index = src_width * (src_y + i) + src_x;
      int32_t dst_index = dst_width * (dst_y + sign_y * i + offset_y) + dst_x;

      for (int32_t j = 0; j < copy_width; j++) {
        dst_data[dst_index + sign_x * j + offset_x] =
            palette_table_[src_data[src_index + j]];
      }
    }
  } else {
    for (int32_t i = 0; i < copy_height; i++) {
      int32_t src_index = src_width * (src_y + i) + src_x;
      int32_t dst_index = dst_width * (dst_y + sign_y * i + offset_y) + dst_x;

      for (int32_t j = 0; j < copy_width; j++) {
        int32_t src_color = src_data[src_index + j];

        if (src_color != color_key) {
          dst_data[dst_index + sign_x * j + offset_x] =
              palette_table_[src_color];
        }
      }
    }
  }
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
  int32_t image_index = tilemap->ImageIndex();

  Rectangle copy_rect = Rectangle::FromSize(u, v, width, height);
  Rectangle dst_rect =
      Rectangle::FromPos(INT16_MIN, INT16_MIN, INT16_MAX, INT16_MAX);
  Rectangle::CopyArea copy_area =
      dst_rect.GetCopyArea(x, y, tilemap->Rectangle(), copy_rect);

  int32_t copy_width = copy_area.copy_width;
  int32_t copy_height = copy_area.copy_height;

  if (copy_width <= 0 || copy_height <= 0) {
    return;
  }

  int32_t src_x = copy_area.src_x;
  int32_t src_y = copy_area.src_y;
  int32_t src_width = tilemap->Width();
  int32_t* src_data = tilemap->Data();

  for (int32_t i = 0; i < copy_height; i++) {
    int32_t index = src_width * (src_y + i) + src_x;

    for (int32_t j = 0; j < copy_width; j++) {
      int32_t value = src_data[index + j];
      int32_t u = (value % (IMAGE_BANK_WIDTH / TILEMAP_CHIP_WIDTH)) *
                  TILEMAP_CHIP_WIDTH;
      int32_t v = (value / (IMAGE_BANK_HEIGHT / TILEMAP_CHIP_HEIGHT)) *
                  TILEMAP_CHIP_HEIGHT;

      DrawImage(x + TILEMAP_CHIP_WIDTH * j, y + TILEMAP_CHIP_HEIGHT * i,
                image_index, u, v, TILEMAP_CHIP_WIDTH, TILEMAP_CHIP_HEIGHT,
                color_key);
    }
  }
}

void Graphics::DrawText(int32_t x, int32_t y, const char* text, int32_t color) {
  color = GetDrawColor(color);

  int32_t cur_color = palette_table_[FONT_COLOR];
  palette_table_[FONT_COLOR] = color;

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

    if (*ch < MIN_FONT_CODE || *ch > MAX_FONT_CODE) {
      continue;
    }

    int32_t code = *ch - MIN_FONT_CODE;
    int32_t u = (code % FONT_ROW_COUNT) * FONT_WIDTH;
    int32_t v = (code / FONT_ROW_COUNT) * FONT_HEIGHT;

    DrawImage(x, y, IMAGE_BANK_FOR_SYSTEM, FONT_X + u, FONT_Y + v, FONT_WIDTH,
              FONT_HEIGHT, 0);

    x += FONT_WIDTH;
  }

  palette_table_[FONT_COLOR] = cur_color;
}

void Graphics::SetupMouseCursor() {
  const char** mouse_cursor_data =
      NewPointerArrayFromArray2D(MOUSE_CURSOR_DATA);

  image_bank_[IMAGE_BANK_FOR_SYSTEM]->SetData(
      MOUSE_CURSOR_X, MOUSE_CURSOR_Y, mouse_cursor_data, MOUSE_CURSOR_HEIGHT);

  delete[] mouse_cursor_data;
}

void Graphics::SetupFont() {
  const int32_t FONT_COUNT = sizeof(FONT_DATA) / sizeof(FONT_DATA[0]);
  int32_t* data = image_bank_[IMAGE_BANK_FOR_SYSTEM]->Data();

  for (int32_t i = 0; i < FONT_COUNT; i++) {
    int32_t row = i / FONT_ROW_COUNT;
    int32_t col = i % FONT_ROW_COUNT;
    int32_t index = IMAGE_BANK_WIDTH * (FONT_HEIGHT * row + FONT_Y) +
                    FONT_WIDTH * col + FONT_X;
    uint32_t font = FONT_DATA[i];

    for (int32_t j = 0; j < FONT_HEIGHT; j++) {
      for (int32_t k = 0; k < FONT_WIDTH; k++) {
        data[index + k] = (font & 0x800000) ? FONT_COLOR : 0;
        font <<= 1;
      }

      index += IMAGE_BANK_WIDTH;
    }
  }
}

}  // namespace pyxelcore
