#include "pyxelcore/graphics.h"

#include "pyxelcore/image.h"
#include "pyxelcore/tilemap.h"

namespace pyxelcore {

Graphics::Graphics(int32_t width, int32_t height) {
  image_bank_ = new Image*[TOTAL_IMAGE_BANK_COUNT];
  for (int32_t i = 0; i < TOTAL_IMAGE_BANK_COUNT; i++) {
    image_bank_[i] = new Image(IMAGE_BANK_WIDTH, IMAGE_BANK_HEIGHT);
  }

  tilemap_bank_ = new Tilemap*[TILEMAP_BANK_COUNT];
  for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
    tilemap_bank_[i] = new Tilemap(TILEMAP_BANK_WIDTH, TILEMAP_BANK_HEIGHT);
  }

  screen_width_ = width;
  screen_height_ = height;
  screen_data_ = image_bank_[IMAGE_BANK_FOR_SCREEN]->Data();

  SetupMouseCursor();
  SetupFont();

  ResetClipArea();
  ResetPalette();
  ClearScreen(0);
}

Graphics::~Graphics() {
  for (int32_t i = 0; i < TOTAL_IMAGE_BANK_COUNT; i++) {
    delete image_bank_[i];
  }
  delete[] image_bank_;

  for (int32_t i = 0; i < TILEMAP_BANK_COUNT; i++) {
    delete tilemap_bank_[i];
  }
  delete[] tilemap_bank_;
}

void Graphics::ResetClipArea() {
  clip_area_ = Rectangle(0, 0, screen_width_, screen_height_);
}

void Graphics::SetClipArea(int32_t x,
                           int32_t y,
                           int32_t width,
                           int32_t height) {
  clip_area_ = Rectangle(0, 0, screen_width_, screen_height_)
                   .Intersect(Rectangle(x, y, width, height));
}

void Graphics::ResetPalette() {
  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_table_[i] = i;
  }
}

void Graphics::SetPalette(int32_t src_color, int32_t dst_color) {
  if (src_color < 0 || src_color >= COLOR_COUNT || dst_color < 0 ||
      dst_color >= COLOR_COUNT) {
    PYXEL_ERROR("invalid color");
  }

  palette_table_[src_color] = dst_color;
}

void Graphics::ClearScreen(int32_t color) {
  int32_t draw_color = GET_DRAW_COLOR(color);

  for (int32_t i = 0; i < screen_height_; i++) {
    int32_t* dst_line = screen_data_[i];

    for (int32_t j = 0; j < screen_width_; j++) {
      dst_line[j] = draw_color;
    }
  }
}

int32_t Graphics::GetPoint(int32_t x, int32_t y) {
  if (x < 0 || y < 0 || x >= screen_width_ || y >= screen_height_) {
    return 0;
  }

  return screen_data_[y][x];
}

void Graphics::SetPoint(int32_t x, int32_t y, int32_t color) {
  SetPixel(x, y, GET_DRAW_COLOR(color));
}

void Graphics::DrawLine(int32_t x1,
                        int32_t y1,
                        int32_t x2,
                        int32_t y2,
                        int32_t color) {
  int32_t draw_color = GET_DRAW_COLOR(color);

  if (x1 == x2 && y1 == y2) {
    SetPixel(x1, y1, draw_color);
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
      SetPixel(start_x + i, start_y + alpha * i + 0.5f, draw_color);
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
      SetPixel(start_x + alpha * i + 0.5f, start_y + i, draw_color);
    }
  }
}

void Graphics::DrawRectangle(int32_t x,
                             int32_t y,
                             int32_t width,
                             int32_t height,
                             int32_t color) {
  int32_t draw_color = GET_DRAW_COLOR(color);
  Rectangle draw_rect = Rectangle(x, y, width, height).Intersect(clip_area_);

  if (draw_rect.IsEmpty()) {
    return;
  }

  int32_t left = draw_rect.Left();
  int32_t top = draw_rect.Top();
  int32_t right = draw_rect.Right();
  int32_t bottom = draw_rect.Bottom();

  for (int32_t i = top; i <= bottom; i++) {
    int32_t* dst_line = screen_data_[i];

    for (int32_t j = left; j <= right; j++) {
      dst_line[j] = draw_color;
    }
  }
}

void Graphics::DrawRectangleBorder(int32_t x,
                                   int32_t y,
                                   int32_t width,
                                   int32_t height,
                                   int32_t color) {
  int32_t draw_color = GET_DRAW_COLOR(color);
  Rectangle draw_rect = Rectangle(x, y, width, height);

  if (draw_rect.Intersect(clip_area_).IsEmpty()) {
    return;
  }

  int32_t left = draw_rect.Left();
  int32_t top = draw_rect.Top();
  int32_t right = draw_rect.Right();
  int32_t bottom = draw_rect.Bottom();

  for (int32_t i = left; i <= right; i++) {
    SetPixel(i, top, draw_color);
    SetPixel(i, bottom, draw_color);
  }

  for (int32_t i = top; i <= bottom; i++) {
    SetPixel(left, i, draw_color);
    SetPixel(right, i, draw_color);
  }
}

void Graphics::DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color) {
  int32_t draw_color = GET_DRAW_COLOR(color);

  if (radius == 0) {
    SetPixel(x, y, draw_color);
    return;
  }

  int32_t sq_radius = radius * radius;

  for (int32_t dx = 0; dx <= radius; dx++) {
    int32_t dy = std::sqrt(sq_radius - dx * dx) + 0.5f;

    if (dx > dy) {
      continue;
    }

    for (int32_t i = -dy; i <= dy; i++) {
      SetPixel(x - dx, y + i, draw_color);
      SetPixel(x + dx, y + i, draw_color);
      SetPixel(x + i, y - dx, draw_color);
      SetPixel(x + i, y + dx, draw_color);
    }
  }
}

void Graphics::DrawCircleBorder(int32_t x,
                                int32_t y,
                                int32_t radius,
                                int32_t color) {
  int32_t draw_color = GET_DRAW_COLOR(color);

  if (radius == 0) {
    SetPixel(x, y, draw_color);
    return;
  }

  int32_t sq_radius = radius * radius;

  for (int32_t dx = 0; dx <= radius; dx++) {
    int32_t dy = std::sqrt(sq_radius - dx * dx) + 0.5f;

    if (dx > dy) {
      continue;
    }

    SetPixel(x - dx, y - dy, draw_color);
    SetPixel(x + dx, y - dy, draw_color);
    SetPixel(x - dx, y + dy, draw_color);
    SetPixel(x + dx, y + dy, draw_color);

    SetPixel(x - dy, y - dx, draw_color);
    SetPixel(x + dy, y - dx, draw_color);
    SetPixel(x - dy, y + dx, draw_color);
    SetPixel(x + dy, y + dx, draw_color);
  }
}

void Graphics::DrawTriangle(int32_t x1,
                            int32_t y1,
                            int32_t x2,
                            int32_t y2,
                            int32_t x3,
                            int32_t y3,
                            int32_t color) {
  int32_t draw_color = GET_DRAW_COLOR(color);

  // rank as y3 > y2 > y1
  if (y1 > y2) {
    std::swap(y1, y2);
    std::swap(x1, x2);
  }
  if (y1 > y3) {
    std::swap(y1, y3);
    std::swap(x1, x3);
  }
  if (y2 > y3) {
    std::swap(y2, y3);
    std::swap(x2, x3);
  }
  // slide bottom-up from y1 to y3
  float alpha12 = (y2 == y1) ? 0 : static_cast<float>(x2 - x1) / (y2 - y1);
  float alpha13 = (y3 == y1) ? 0 : static_cast<float>(x3 - x1) / (y3 - y1);
  float alpha23 = (y3 == y2) ? 0 : static_cast<float>(x3 - x2) / (y3 - y2);
  int32_t x_intersection = x1 + alpha13 * (y2 - y1) + 0.5f;
  int32_t y_slider = y1;
  for (; y_slider <= y2; y_slider++) {
    int32_t x_slider, x_end;

    if (x_intersection < x2) {
      x_slider = x_intersection + alpha13 * (y_slider - y2) + 0.5f;
      x_end = x2 + alpha12 * (y_slider - y2) + 0.5f;
    } else {
      x_slider = x2 + alpha12 * (y_slider - y2) + 0.5f;
      x_end = x_intersection + alpha13 * (y_slider - y2) + 0.5f;
    }

    for (; x_slider <= x_end; x_slider++) {
      SetPixel(x_slider, y_slider, draw_color);
    }
  }
  for (; y_slider <= y3; y_slider++) {
    int32_t x_slider, x_end;

    if (x_intersection < x2) {
      x_slider = x_intersection + alpha13 * (y_slider - y2) + 0.5f;
      x_end = x2 + alpha23 * (y_slider - y2) + 0.5f;
    } else {
      x_slider = x2 + alpha23 * (y_slider - y2) + 0.5f;
      x_end = x_intersection + alpha13 * (y_slider - y2) + 0.5f;
    }

    for (; x_slider <= x_end; x_slider++) {
      SetPixel(x_slider, y_slider, draw_color);
    }
  }
}

void Graphics::DrawTriangleBorder(int32_t x1,
                                  int32_t y1,
                                  int32_t x2,
                                  int32_t y2,
                                  int32_t x3,
                                  int32_t y3,
                                  int32_t color) {
  DrawLine(x1, y1, x2, y2, color);
  DrawLine(x1, y1, x3, y3, color);
  DrawLine(x2, y2, x3, y3, color);
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
    PYXEL_ERROR("invalid color");
  }

  Rectangle::CopyArea copy_area =
      clip_area_.GetCopyArea(x, y, image->Rectangle(), u, v, Abs(width),
                             Abs(height), width < 0, height < 0);

  if (copy_area.IsEmpty()) {
    return;
  }

  int32_t** src_data = image->Data();
  int32_t** dst_data = screen_data_;

  int32_t sign_x, sign_y;
  int32_t offset_x, offset_y;

  if (width < 0) {
    sign_x = -1;
    offset_x = copy_area.width - 1;
  } else {
    sign_x = 1;
    offset_x = 0;
  }

  if (height < 0) {
    sign_y = -1;
    offset_y = copy_area.height - 1;
  } else {
    sign_y = 1;
    offset_y = 0;
  }

  for (int32_t i = 0; i < copy_area.height; i++) {
    int32_t* src_line = src_data[copy_area.v + sign_y * i + offset_y];
    int32_t* dst_line = dst_data[copy_area.y + i];

    for (int32_t j = 0; j < copy_area.width; j++) {
      int32_t src_color = src_line[copy_area.u + sign_x * j + offset_x];

      if (src_color != color_key) {
        dst_line[copy_area.x + j] = palette_table_[src_color];
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

  int32_t left = clip_area_.Left() / TILEMAP_CHIP_WIDTH;
  int32_t top = clip_area_.Top() / TILEMAP_CHIP_WIDTH;
  int32_t right =
      (clip_area_.Right() + TILEMAP_CHIP_WIDTH - 1) / TILEMAP_CHIP_WIDTH;
  int32_t bottom =
      (clip_area_.Bottom() + TILEMAP_CHIP_HEIGHT - 1) / TILEMAP_CHIP_HEIGHT;
  Rectangle dst_rect = Rectangle(left, top, right - left + 1, bottom - top + 1);

  Rectangle::CopyArea copy_area =
      dst_rect.GetCopyArea(x / TILEMAP_CHIP_WIDTH, y / TILEMAP_CHIP_HEIGHT,
                           tilemap->Rectangle(), u, v, width, height);

  if (copy_area.IsEmpty()) {
    return;
  }

  int32_t** src_data = tilemap->Data();

  copy_area.x = copy_area.x * TILEMAP_CHIP_WIDTH + x % TILEMAP_CHIP_WIDTH;
  copy_area.y = copy_area.y * TILEMAP_CHIP_HEIGHT + y % TILEMAP_CHIP_HEIGHT;

  for (int32_t i = 0; i < copy_area.height; i++) {
    int32_t* src_line = src_data[copy_area.v + i];
    int32_t dst_y = copy_area.y + TILEMAP_CHIP_HEIGHT * i;

    for (int32_t j = 0; j < copy_area.width; j++) {
      int32_t chip = src_line[copy_area.u + j];
      int32_t cu =
          (chip % (IMAGE_BANK_WIDTH / TILEMAP_CHIP_WIDTH)) * TILEMAP_CHIP_WIDTH;
      int32_t cv = (chip / (IMAGE_BANK_HEIGHT / TILEMAP_CHIP_HEIGHT)) *
                   TILEMAP_CHIP_HEIGHT;

      DrawImage(copy_area.x + TILEMAP_CHIP_WIDTH * j, dst_y, image_index, cu,
                cv, TILEMAP_CHIP_WIDTH, TILEMAP_CHIP_HEIGHT, color_key);
    }
  }
}

void Graphics::DrawText(int32_t x, int32_t y, const char* text, int32_t color) {
  int32_t draw_color = GET_DRAW_COLOR(color);
  int32_t cur_color = palette_table_[FONT_COLOR];
  palette_table_[FONT_COLOR] = draw_color;

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
  image_bank_[IMAGE_BANK_FOR_SYSTEM]->SetData(MOUSE_CURSOR_X, MOUSE_CURSOR_Y,
                                              MOUSE_CURSOR_DATA);
}

void Graphics::SetupFont() {
  const int32_t FONT_COUNT = FONT_DATA.size();
  int32_t** dst_data = image_bank_[IMAGE_BANK_FOR_SYSTEM]->Data();

  for (int32_t i = 0; i < FONT_COUNT; i++) {
    int32_t row = i / FONT_ROW_COUNT;
    int32_t col = i % FONT_ROW_COUNT;
    uint32_t font = FONT_DATA[i];

    for (int32_t j = 0; j < FONT_HEIGHT; j++) {
      int32_t* dst_line = dst_data[FONT_Y + FONT_HEIGHT * row + j];

      for (int32_t k = 0; k < FONT_WIDTH; k++) {
        dst_line[FONT_X + FONT_WIDTH * col + k] =
            (font & 0x800000) ? FONT_COLOR : 0;
        font <<= 1;
      }
    }
  }
}

}  // namespace pyxelcore
