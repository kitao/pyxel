#include "pyxelcore/graphics.h"

#include "pyxelcore/constants.h"
#include "pyxelcore/image.h"
#include "pyxelcore/utilities.h"

namespace pyxelcore {

Graphics::Graphics(int32_t width, int32_t height) {
  width_ = width;
  height_ = height;
  screen_ = new Image(width, height);
  screen_data_ = screen_->Data();

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

  for (size_t i = 0; i < size; i++) {
    screen_data_[i] = color;
  }
}

void Graphics::DrawPoint(int32_t x, int32_t y, int32_t color) {
  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  if (x < clip_x1_ || y < clip_y1_ || x > clip_x2_ || y > clip_y2_) {
    return;
  }

  screen_data_[width_ * y + x] = palette_table_[color];
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

  for (int32_t i = top; i <= bottom; i++) {
    int32_t index = width_ * i;

    for (int32_t j = left; j <= right; j++) {
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

  int32_t left = std::max(std::min(x1, x2), clip_x1_);
  int32_t top = std::max(std::min(y1, y2), clip_y1_);
  int32_t right = std::min(std::max(x1, x2), clip_x2_);
  int32_t bottom = std::min(std::max(y1, y2), clip_y2_);

  if (x1 >= clip_x1_ && x1 <= clip_x2_) {
    for (int32_t i = top; i <= bottom; i++) {
      screen_data_[width_ * i + x1] = color;
    }
  }

  if (x2 >= clip_x1_ && x2 <= clip_x2_) {
    for (int32_t i = top; i <= bottom; i++) {
      screen_data_[width_ * i + x2] = color;
    }
  }

  if (y1 >= clip_y1_ && y1 <= clip_y2_) {
    int32_t index = width_ * y1;

    for (int32_t i = left; i <= right; i++) {
      screen_data_[index + i] = color;
    }
  }

  if (y2 >= clip_y1_ && y2 <= clip_y2_) {
    size_t line_head = width_ * y2;

    for (int32_t i = left; i <= right; i++) {
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

  int32_t src_x = u;
  int32_t src_y = v;
  int32_t src_w = src_image->Width();
  int32_t src_h = src_image->Height();
  int32_t dest_x = x;
  int32_t dest_y = y;
  int32_t dest_w = width_;
  int32_t dest_h = height_;
  int32_t copy_w = src_w;
  int32_t copy_h = src_h;

  CopyRegion copy_region =
      GetCopyRegion(src_x, src_y, src_w, src_h, dest_x, dest_y, dest_w, dest_h,
                    copy_w, copy_h, clip_x1_, clip_y1_, clip_x2_, clip_y2_);

  src_x = copy_region.src_x;
  src_y = copy_region.src_y;
  dest_x = copy_region.dest_x;
  dest_y = copy_region.dest_y;
  copy_w = copy_region.copy_w;
  copy_h = copy_region.copy_h;

  if (copy_w <= 0 || copy_h <= 0) {
    return;
  }

  int32_t* src_data = src_image->Data();
  int32_t* dest_data = screen_data_;

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
