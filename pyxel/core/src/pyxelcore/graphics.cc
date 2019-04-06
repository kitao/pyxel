#include "pyxelcore/graphics.h"

#include "pyxelcore/constants.h"
#include "pyxelcore/image.h"

namespace pyxelcore {

Graphics::Graphics(int32_t width, int32_t height) {
  width_ = width;
  height_ = height;
  screen_ = new Image(width, height);

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

  delete screen_;
}

Image* Graphics::GetImage(int32_t image_index, bool system) {
  if (image_index < 0 || image_index >= IMAGE_COUNT) {
    // error
  }

  if (image_index == IMAGE_COUNT - 1 && !system) {
    // error
  }

  return image_bank_[image_index];
}

Tilemap* Graphics::GetTilemap(int32_t tilemap_index) {
  //
  return tilemap_bank_[tilemap_index];
}

void Graphics::ResetClippingArea() {
  clip_x1_ = 0;
  clip_y1_ = 0;
  clip_x2_ = width_ - 1;
  clip_y2_ = height_ - 1;
}

void Graphics::SetClippingArea(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  clip_x1_ = std::max(std::min(x1, x2), 0);
  clip_y1_ = std::max(std::min(y1, y2), 0);
  clip_x2_ = std::min(std::max(x1, x2), width_ - 1);
  clip_y2_ = std::min(std::max(y1, y2), height_ - 1);
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

  size_t size = width_ * height_;
  int32_t* data = screen_->Data();

  for (size_t i = 0; i < size; i++) {
    data[i] = color;
  }
}

void Graphics::DrawPoint(int32_t x, int32_t y, int32_t color) {
  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  if (x < clip_x1_ || y < clip_y1_ || x > clip_x2_ || y > clip_y2_) {
    return;
  }

  screen_->Data()[width_ * y + x] = palette_table_[color];
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

  int32_t left = std::max(std::min(x1, x2), clip_x1_);
  int32_t top = std::max(std::min(y1, y2), clip_y1_);
  int32_t right = std::min(std::max(x1, x2), clip_x2_);
  int32_t bottom = std::min(std::max(y1, y2), clip_y2_);

  int32_t* data = screen_->Data();

  for (int32_t i = top; i <= bottom; i++) {
    int32_t index = width_ * i;

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
  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  color = palette_table_[color];

  int32_t left = std::max(std::min(x1, x2), clip_x1_);
  int32_t top = std::max(std::min(y1, y2), clip_y1_);
  int32_t right = std::min(std::max(x1, x2), clip_x2_);
  int32_t bottom = std::min(std::max(y1, y2), clip_y2_);

  int32_t* data = screen_->Data();

  if (x1 >= clip_x1_ && x1 <= clip_x2_) {
    for (int32_t i = top; i <= bottom; i++) {
      data[width_ * i + x1] = color;
    }
  }

  if (x2 >= clip_x1_ && x2 <= clip_x2_) {
    for (int32_t i = top; i <= bottom; i++) {
      data[width_ * i + x2] = color;
    }
  }

  if (y1 >= clip_y1_ && y1 <= clip_y2_) {
    int32_t index = width_ * y1;

    for (int32_t i = left; i <= right; i++) {
      data[index + i] = color;
    }
  }

  if (y2 >= clip_y1_ && y2 <= clip_y2_) {
    size_t line_head = width_ * y2;

    for (int32_t i = left; i <= right; i++) {
      data[line_head + i] = color;
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
  int32_t src_width = src_image->Width();
  int32_t src_height = src_image->Height();

  int32_t left_offset = std::max(std::max(-x + clip_x1_, -u), 0);
  int32_t top_offset = std::max(std::max(-y + clip_y1_, -v), 0);
  int32_t right_offset =
      std::max(std::max(u + width - src_width, x + width - 1 - clip_x2_), 0);
  int32_t bottom_offset =
      std::max(std::max(v + height - src_height, y + height - 1 - clip_y2_), 0);

  x += left_offset;
  y += top_offset;
  u += left_offset;
  v += top_offset;
  width -= left_offset + right_offset;
  height -= top_offset + bottom_offset;

  if (x >= width_ || y >= height_ || u >= width_ || v >= height_ ||
      width <= 0 || height <= 0) {
    return;
  }

  int32_t* src_data = src_image->Data();
  int32_t* dest_data = screen_->Data();

  if (color_key == -1) {
    for (int32_t i = 0; i < height; i++) {
      int32_t src_index = src_width * (v + i) + u;
      int32_t dest_index = width_ * (y + i) + x;

      for (int32_t j = 0; j < width; j++) {
        dest_data[dest_index + j] = palette_table_[src_data[src_index + j]];
      }
    }
  } else {
    for (int32_t i = 0; i < height; i++) {
      int32_t src_index = src_width * (v + i) + u;
      int32_t dest_index = width_ * (y + i) + x;

      for (int32_t j = 0; j < width; j++) {
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
