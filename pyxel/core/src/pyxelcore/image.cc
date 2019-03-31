#include "pyxelcore/image.h"

#include <algorithm>

namespace pyxelcore {

Image::Image(int32_t width, int32_t height, int32_t color_count) {
  width_ = width;
  height_ = height;
  color_count_ = color_count;
  data_ = new int32_t[width * height];
  palette_ = new int32_t[color_count];

  ResetClippingArea();
  ResetPalette();
  Clear(0);
}

Image::~Image() {
  delete[] data_;
  delete[] palette_;
}

int32_t Image::GetColor(int32_t x, int32_t y) {
  if (x < 0 || y < 0 || x >= width_ || y >= height_) {
    // error
  }

  return data_[width_ * y + x];
}

void Image::ResetClippingArea() {
  clipping_area_.x1 = 0;
  clipping_area_.y1 = 0;
  clipping_area_.x2 = width_ - 1;
  clipping_area_.y2 = height_ - 1;
}

void Image::SetClippingArea(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  clipping_area_.x1 = std::max(std::min(x1, x2), 0);
  clipping_area_.y1 = std::max(std::min(y1, y2), 0);
  clipping_area_.x2 = std::min(std::max(x1, x2), width_ - 1);
  clipping_area_.y2 = std::min(std::max(y1, y2), height_ - 1);
}

void Image::ResetPalette() {
  for (int32_t i = 0; i < color_count_; i++) {
    palette_[i] = i;
  }
}

void Image::SetPalette(int32_t src_color, int32_t dest_color) {
  palette_[src_color] = dest_color;
}

void Image::Load(int32_t x,
                 int32_t y,
                 const char* filename,
                 int32_t width,
                 int32_t height) {
  //
}

void Image::Clear(int32_t color) {
  if (color < 0 || color > color_count_) {
    // error
  }

  color = palette_[color];

  size_t size = width_ * height_;

  for (size_t i = 0; i < size; i++) {
    data_[i] = color;
  }
}

void Image::DrawPoint(int32_t x, int32_t y, int32_t color) {
  if (color < 0 || color > color_count_) {
    // error
  }

  if (x < clipping_area_.x1 || y < clipping_area_.y1 || x > clipping_area_.x2 ||
      y > clipping_area_.y2) {
    return;
  }

  data_[width_ * y + x] = palette_[color];
}

void Image::DrawLine(int32_t x1,
                     int32_t y1,
                     int32_t x2,
                     int32_t y2,
                     int32_t color) {
  //
}

void Image::DrawRectangle(int32_t x1,
                          int32_t y1,
                          int32_t x2,
                          int32_t y2,
                          int32_t color) {
  if (color < 0 || color >= color_count_) {
    // error
  }

  color = palette_[color];

  int32_t left = std::max(std::min(x1, x2), clipping_area_.x1);
  int32_t top = std::max(std::min(y1, y2), clipping_area_.y1);
  int32_t right = std::min(std::max(x1, x2), clipping_area_.x2);
  int32_t bottom = std::min(std::max(y1, y2), clipping_area_.y2);

  for (int32_t i = top; i <= bottom; i++) {
    int32_t index = width_ * i;

    for (int32_t j = left; j <= right; j++) {
      data_[index + j] = color;
    }
  }
}

void Image::DrawRectangleBorder(int32_t x1,
                                int32_t y1,
                                int32_t x2,
                                int32_t y2,
                                int32_t color) {
  if (color < 0 || color >= color_count_) {
    // error
  }

  color = palette_[color];

  int32_t left = std::max(std::min(x1, x2), clipping_area_.x1);
  int32_t top = std::max(std::min(y1, y2), clipping_area_.y1);
  int32_t right = std::min(std::max(x1, x2), clipping_area_.x2);
  int32_t bottom = std::min(std::max(y1, y2), clipping_area_.y2);

  if (x1 >= clipping_area_.x1 && x1 <= clipping_area_.x2) {
    for (int32_t i = top; i <= bottom; i++) {
      data_[width_ * i + x1] = color;
    }
  }

  if (x2 >= clipping_area_.x1 && x2 <= clipping_area_.x2) {
    for (int32_t i = top; i <= bottom; i++) {
      data_[width_ * i + x2] = color;
    }
  }

  if (y1 >= clipping_area_.y1 && y1 <= clipping_area_.y2) {
    int32_t index = width_ * y1;

    for (int32_t i = left; i <= right; i++) {
      data_[index + i] = color;
    }
  }

  if (y2 >= clipping_area_.y1 && y2 <= clipping_area_.y2) {
    size_t line_head = width_ * y2;

    for (int32_t i = left; i <= right; i++) {
      data_[line_head + i] = color;
    }
  }
  //
}

void Image::DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color) {
  //
}

void Image::DrawCircleBorder(int32_t x,
                             int32_t y,
                             int32_t radius,
                             int32_t color) {
  //
}

void Image::DrawImage(int32_t x,
                      int32_t y,
                      Image* image,
                      int32_t u,
                      int32_t v,
                      int32_t width,
                      int32_t height,
                      int32_t color_key) {
  if (color_key != -1 && (color_key < 0 || color_key >= color_count_)) {
    // error
  }

  int32_t left_offset = std::max(std::max(-x + clipping_area_.x1, -u), 0);
  int32_t top_offset = std::max(std::max(-y + clipping_area_.y1, -v), 0);
  int32_t right_offset = std::max(
      std::max(u + width - image->width_, x + width - 1 - clipping_area_.x2),
      0);
  int32_t bottom_offset = std::max(
      std::max(v + height - image->height_, y + height - 1 - clipping_area_.y2),
      0);

  x += left_offset;
  y += top_offset;
  u += left_offset;
  v += top_offset;
  width -= left_offset + right_offset;
  height -= top_offset + bottom_offset;

  if (x >= width_ || y >= height_ || u >= image->width_ ||
      v >= image->height_ || width <= 0 || height <= 0) {
    return;
  }

  int32_t* src_data = image->data_;
  int32_t* dest_data = data_;

  if (color_key == -1) {
    for (int32_t i = 0; i < height; i++) {
      int32_t src_index = i * image->width_ + u;
      int32_t dest_index = i * width_ + x;

      for (int32_t j = 0; j < width; j++) {
        dest_data[src_index + j] = palette_[src_data[src_index + j]];
      }
    }
  } else {
    for (int32_t i = 0; i < height; i++) {
      int32_t src_index = i * image->width_ + u;
      int32_t dest_index = i * width_ + x;

      for (int32_t j = 0; j < width; j++) {
        int32_t src_color = src_data[src_index + j];

        if (src_color != color_key) {
          dest_data[src_index + j] = palette_[src_color];
        }
      }
    }
  }
}

}  // namespace pyxelcore
