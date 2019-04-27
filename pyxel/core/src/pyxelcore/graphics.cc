#include "pyxelcore/graphics.h"

#include "pyxelcore/constants.h"
#include "pyxelcore/image.h"

namespace pyxelcore {

Graphics::Graphics(int32_t width, int32_t height) {
  screen_image_ = new Image(width, height);
  screen_width_ = screen_image_->Width();
  screen_height_ = screen_image_->Height();
  screen_data_ = screen_image_->Data();

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

  if (image_index == IMAGE_COUNT - 1 && !system) {
    // error
  }

  return image_bank_[image_index];
}

Tilemap* Graphics::GetTilemap(int32_t tilemap_index) const {
  //
  return tilemap_bank_[tilemap_index];
}

void Graphics::ResetClippingArea() {
  clip_rect_ = Rectangle::FromSize(0, 0, screen_width_, screen_height_);
}

void Graphics::SetClippingArea(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  clip_rect_ =
      Rectangle::FromPos(x1, y1, x2, y2)
          .Intersect(Rectangle::FromSize(0, 0, screen_width_, screen_height_));
}

void Graphics::ResetPalette() {
  for (int32_t i = 0; i < COLOR_COUNT; i++) {
    palette_table_[i] = i;
  }
}

void Graphics::SetPalette(int32_t src_color, int32_t dest_color) {
  palette_table_[src_color] = dest_color;
}

void Graphics::Clear(int32_t color) {
  color = GetDrawColor(color);

  int32_t size = screen_width_ * screen_height_;

  for (int32_t i = 0; i < size; i++) {
    screen_data_[i] = color;
  }
}

void Graphics::DrawPoint(int32_t x, int32_t y, int32_t color) {
  color = GetDrawColor(color);

  if (!clip_rect_.Includes(x, y)) {
    return;
  }

  screen_data_[screen_width_ * y + x] = color;
}

void Graphics::DrawLine(int32_t x1,
                        int32_t y1,
                        int32_t x2,
                        int32_t y2,
                        int32_t color) {
  color = GetDrawColor(color);

  if (x1 == x2 && y1 == y2) {
    if (clip_rect_.Includes(x1, y1)) {
      screen_data_[screen_width_ * y1 + x1] = color;
    }

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
      int32_t x = start_x + i;
      int32_t y = static_cast<int>(start_y + alpha * i + 0.5f);

      if (clip_rect_.Includes(x, y)) {
        screen_data_[screen_width_ * y + x] = color;
      }
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
      int32_t y = start_y + i;
      int32_t x = static_cast<int>(start_x + alpha * i + 0.5f);

      if (clip_rect_.Includes(x, y)) {
        screen_data_[screen_width_ * y + x] = color;
      }
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

  int32_t rect_left = draw_rect.Left();
  int32_t rect_top = draw_rect.Top();
  int32_t rect_right = draw_rect.Right();
  int32_t rect_bottom = draw_rect.Bottom();

  for (int32_t i = rect_top; i <= rect_bottom; i++) {
    int32_t index = screen_width_ * i;

    for (int32_t j = rect_left; j <= rect_right; j++) {
      screen_data_[index + j] = color;
    }
  }
}

void Graphics::DrawRectangleBorder(int32_t x1,
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

  int32_t rect_left = draw_rect.Left();
  int32_t rect_top = draw_rect.Top();
  int32_t rect_right = draw_rect.Right();
  int32_t rect_bottom = draw_rect.Bottom();

  if (x1 >= clip_rect_.Left() && x1 <= clip_rect_.Right()) {
    for (int32_t i = rect_top; i <= rect_bottom; i++) {
      screen_data_[screen_width_ * i + x1] = color;
    }
  }

  if (x2 >= clip_rect_.Left() && x2 <= clip_rect_.Right()) {
    for (int32_t i = rect_top; i <= rect_bottom; i++) {
      screen_data_[screen_width_ * i + x2] = color;
    }
  }

  if (y1 >= clip_rect_.Top() && y1 <= clip_rect_.Bottom()) {
    int32_t index = screen_width_ * y1;

    for (int32_t i = rect_left; i <= rect_right; i++) {
      screen_data_[index + i] = color;
    }
  }

  if (y2 >= clip_rect_.Top() && y2 <= clip_rect_.Bottom()) {
    int32_t index = screen_width_ * y2;

    for (int32_t i = rect_left; i <= rect_right; i++) {
      screen_data_[index + i] = color;
    }
  }
}

void Graphics::DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color) {
  //
}

void Graphics::DrawCircleBorder(int32_t x,
                                int32_t y,
                                int32_t radius,
                                int32_t color) {
  color = GetDrawColor(color);
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

  //
}

void Graphics::SetupFontImage() {
  const int32_t FONT_COUNT = sizeof(FONT_DATA) / sizeof(FONT_DATA[0]);
  int32_t* data = image_bank_[IMAGE_COUNT - 1]->Data();

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
