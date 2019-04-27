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
  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  color = palette_table_[color];

  size_t size = screen_width_ * screen_height_;

  for (size_t i = 0; i < size; i++) {
    screen_data_[i] = color;
  }
}

void Graphics::DrawPoint(int32_t x, int32_t y, int32_t color) {
  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  if (!clip_rect_.Includes(x, y)) {
    return;
  }

  screen_data_[screen_width_ * y + x] = palette_table_[color];
}

void Graphics::DrawLine(int32_t x1,
                        int32_t y1,
                        int32_t x2,
                        int32_t y2,
                        int32_t color) {
  //
}

void Graphics::DrawRectangle(int32_t x1,
                             int32_t y1,
                             int32_t x2,
                             int32_t y2,
                             int32_t color) {
  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  color = palette_table_[color];
  Rectangle draw_rect =
      Rectangle::FromPos(x1, y1, x2, y2).Intersect(clip_rect_);

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
  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  color = palette_table_[color];
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
    size_t line_head = screen_width_ * y2;

    for (int32_t i = rect_left; i <= rect_right; i++) {
      screen_data_[line_head + i] = color;
    }
  }
  //
}

void Graphics::DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color) {
  //
}

void Graphics::DrawCircleBorder(int32_t x,
                                int32_t y,
                                int32_t radius,
                                int32_t color) {
  //
}

void Graphics::DrawImage(int32_t x,
                         int32_t y,
                         int32_t image_index,
                         int32_t u,
                         int32_t v,
                         int32_t width,
                         int32_t height,
                         int32_t color_key) {
  if (image_index < 0 || image_index >= IMAGE_COUNT) {
    // error
  }

  if (color_key != -1 && (color_key < 0 || color_key >= COLOR_COUNT)) {
    // error
  }

  Image* src_image = image_bank_[image_index];
  int32_t src_w = src_image->Width();
  int32_t src_h = src_image->Height();

  Rectangle copy_reigon =
      Rectangle::FromSize(u, v, width, height)
          .Intersect(Rectangle::FromSize(0, 0, src_w, src_h));

  int32_t offset_x = copy_reigon.Left() - u;
  int32_t offset_y = copy_reigon.Top() - v;

  copy_reigon =
      copy_reigon.MoveTo(x + offset_x, y + offset_y).Intersect(clip_rect_);

  if (copy_reigon.IsEmpty()) {
    return;
  }

  offset_x = copy_reigon.Left() - x;
  offset_y = copy_reigon.Top() - y;

  int32_t src_x = u + offset_x;
  int32_t src_y = v + offset_y;
  int32_t* src_data = src_image->Data();

  int32_t dest_x = x + offset_x;
  int32_t dest_y = y + offset_y;
  int32_t dest_w = screen_width_;
  int32_t dest_h = screen_height_;
  int32_t* dest_data = screen_data_;

  int32_t copy_w = copy_reigon.Width();
  int32_t copy_h = copy_reigon.Height();

  if (color_key == -1) {
    for (int32_t i = 0; i < copy_h; i++) {
      int32_t src_index = src_w * (src_y + i) + src_x;
      int32_t dest_index = dest_w * (dest_y + i) + dest_x;

      for (int32_t j = 0; j < copy_w; j++) {
        dest_data[dest_index + j] = palette_table_[src_data[src_index + j]];
      }
    }
  } else {
    for (int32_t i = 0; i < copy_h; i++) {
      int32_t src_index = src_w * (src_y + i) + src_x;
      int32_t dest_index = dest_w * (dest_y + i) + dest_x;

      for (int32_t j = 0; j < copy_w; j++) {
        int32_t src_color = src_data[src_index + j];

        if (src_color != color_key) {
          dest_data[dest_index + j] = palette_table_[src_color];
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
                           int32_t colkey) {
  //
}

void Graphics::DrawText(int32_t x, int32_t y, const char* text, int32_t color) {
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

}  // namespace pyxelcore
