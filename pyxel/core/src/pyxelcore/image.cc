#include "pyxelcore/image.h"

namespace pyxelcore {

Image::Image(int32_t width, int32_t height) {
  if (width < 1 || height < 1) {
    PRINT_ERROR("invalid image size");
    width = Max(width, 1);
    height = Max(height, 1);
  }

  width_ = width;
  height_ = height;
  rect_ = Rectangle::FromSize(0, 0, width, height);
  data_ = new int32_t[width * height];
}

Image::~Image() {
  delete[] data_;
}

int32_t Image::GetValue(int32_t x, int32_t y) const {
  if (!rect_.Includes(x, y)) {
    PRINT_ERROR("access to outside image");
    return 0;
  }

  return data_[width_ * y + x];
}

void Image::SetValue(int32_t x, int32_t y, int32_t value) {
  if (!rect_.Includes(x, y)) {
    return;
  }

  if (value < 0 || value >= COLOR_COUNT) {
    PRINT_ERROR("invalid value");
  }

  data_[width_ * y + x] = value;
}

void Image::SetValue(int32_t x,
                     int32_t y,
                     const char** value,
                     int32_t value_count) {
  int32_t width = strlen(value[0]);
  int32_t height = value_count;

  if (width < 1 || height < 1) {
    PRINT_ERROR("invalid value size");
    return;
  }

  Image image = Image(width, height);
  int32_t* data = image.data_;

  for (int32_t i = 0; i < height; i++) {
    int32_t index = width * i;
    std::string str = value[i];

    for (int32_t j = 0; j < width; j++) {
      int32_t value = std::stoi(str.substr(j, 1), nullptr, 16);

      if (value < 0 || value >= COLOR_COUNT) {
        PRINT_ERROR("invalid value");
        value = 0;
      }

      data[index + j] = value;
    }
  }

  CopyImage(x, y, &image, 0, 0, width, height);
}

bool Image::LoadImage(int32_t x,
                      int32_t y,
                      const char* filename,
                      const int32_t* palette_color) {
  SDL_Surface* png_image = IMG_Load(filename);

  if (!png_image) {
    char buf[256];
    snprintf(buf, sizeof(buf), "cannot load image '%s'", filename);
    PRINT_ERROR(buf);

    return false;
  }

  SDL_Surface* src_image =
      SDL_ConvertSurfaceFormat(png_image, SDL_PIXELFORMAT_RGBA8888, 0);

  int32_t src_width = src_image->pitch;
  uint8_t* src_data = reinterpret_cast<uint8_t*>(src_image->pixels);

  int32_t width = src_image->w;
  int32_t height = src_image->h;

  Image dst_image = Image(width, height);
  int32_t* dst_data = dst_image.data_;

  for (int32_t i = 0; i < height; i++) {
    int32_t src_index = src_width * i;
    int32_t dst_index = width * i;

    for (int32_t j = 0; j < width; j++) {
      int32_t src_r = src_data[src_index + j * 4 + 3];
      int32_t src_g = src_data[src_index + j * 4 + 2];
      int32_t src_b = src_data[src_index + j * 4 + 1];

      int32_t nearest_color = 0;
      int32_t nearest_color_dist = INT32_MAX;

      for (int32_t k = 0; k < COLOR_COUNT; k++) {
        int32_t color = palette_color[k];
        int32_t pal_r = (color >> 16) & 0xff;
        int32_t pal_g = (color >> 8) & 0xff;
        int32_t pal_b = color & 0xff;
        int32_t color_dist =
            Abs(src_r - pal_r) + Abs(src_g - pal_g) + Abs(src_b - pal_b);

        if (color_dist < nearest_color_dist) {
          nearest_color = k;
          nearest_color_dist = color_dist;
        }
      }

      dst_data[dst_index + j] = nearest_color;
    }
  }

  CopyImage(x, y, &dst_image, 0, 0, width, height);

  SDL_FreeSurface(png_image);
  SDL_FreeSurface(src_image);

  return true;
}

void Image::CopyImage(int32_t x,
                      int32_t y,
                      const Image* image,
                      int32_t u,
                      int32_t v,
                      int32_t width,
                      int32_t height) {
  Rectangle::CopyArea copy_area =
      rect_.GetCopyArea(x, y, image->Rectangle(), u, v, width, height);

  if (copy_area.IsEmpty()) {
    return;
  }

  int32_t src_width = image->width_;
  int32_t* src_data = image->data_;

  int32_t dst_width = width_;
  int32_t* dst_data = data_;

  for (int32_t i = 0; i < copy_area.height; i++) {
    int32_t src_index = src_width * (copy_area.v + i) + copy_area.u;
    int32_t dst_index = dst_width * (copy_area.y + i) + copy_area.x;

    for (int32_t j = 0; j < copy_area.width; j++) {
      dst_data[dst_index + j] = src_data[src_index + j];
    }
  }
}

}  // namespace pyxelcore
