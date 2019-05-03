#include "pyxelcore/image.h"

#include <string>

namespace pyxelcore {

Image::Image(int32_t width, int32_t height) {
  if (width < 1 || height < 1) {
    PRINT_ERROR("invalide image size");
    width = Max(width, 1);
    height = Max(height, 1);
  }

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

  return data_[Width() * y + x];
}

void Image::SetValue(int32_t x, int32_t y, int32_t value) {
  if (!rect_.Includes(x, y)) {
    return;
  }

  if (value < 0 || value >= COLOR_COUNT) {
    PRINT_ERROR("invalid value");
  }

  data_[Width() * y + x] = value;
}

void Image::SetData(int32_t x,
                    int32_t y,
                    const char** data,
                    int32_t data_count) {
  int32_t width = strlen(data[0]);
  int32_t height = data_count;
  Image* image = new Image(width, height);
  int32_t* dst_data = image->data_;

  for (int32_t i = 0; i < height; i++) {
    int32_t index = width * i;
    std::string str = data[i];

    for (int32_t j = 0; j < width; j++) {
      int32_t value = std::stoi(str.substr(j, 1), nullptr, 16);

      if (value < 0 || value >= COLOR_COUNT) {
        PRINT_ERROR("invalid value");
        value = 0;
      }

      dst_data[index + j] = value;
    }
  }

  CopyImage(x, y, image, 0, 0, width, height);

  delete image;
}

void Image::LoadImage(int32_t x,
                      int32_t y,
                      const char* filename,
                      const int32_t* palette_color) {
  SDL_Surface* png_image = IMG_Load(filename);

  if (!png_image) {
    char buf[256];
    snprintf(buf, sizeof(buf), "cannot load image '%s'", filename);
    PRINT_ERROR(buf);
    return;
  }

  SDL_Surface* src_image =
      SDL_ConvertSurfaceFormat(png_image, SDL_PIXELFORMAT_RGBA8888, 0);

  uint8_t* src_data = reinterpret_cast<uint8_t*>(src_image->pixels);
  int32_t src_width = src_image->pitch;

  int32_t dst_width = src_image->w;
  int32_t dst_height = src_image->h;

  Image image = Image(dst_width, dst_height);
  int32_t* dest_data = image.data_;

  for (int32_t i = 0; i < dst_height; i++) {
    int32_t src_index = src_width * i;
    int32_t dest_index = dst_width * i;

    for (int32_t j = 0; j < dst_width; j++) {
      int32_t src_r = src_data[src_index + j * 4 + 3];
      int32_t src_g = src_data[src_index + j * 4 + 2];
      int32_t src_b = src_data[src_index + j * 4 + 1];

      int32_t nearest_color = 0;
      int32_t nearest_color_dist = INT32_MAX;

      for (int32_t k = 0; k < COLOR_COUNT; k++) {
        int32_t color = palette_color[k];
        int32_t dr = src_r - ((color >> 16) & 0xff);
        int32_t dg = src_g - ((color >> 8) & 0xff);
        int32_t db = src_b - (color & 0xff);
        int32_t color_dist = dr * dr + dg * dg + db * db;

        if (color_dist < nearest_color_dist) {
          nearest_color = k;
          nearest_color_dist = color_dist;
        }
      }

      dest_data[dest_index + j] = nearest_color;
    }
  }

  CopyImage(x, y, &image, 0, 0, dst_width, dst_height);

  SDL_FreeSurface(png_image);
  SDL_FreeSurface(src_image);
}

void Image::CopyImage(int32_t x,
                      int32_t y,
                      const Image* image,
                      int32_t u,
                      int32_t v,
                      int32_t width,
                      int32_t height) {
  Rectangle::CopyArea copy_area = rect_.GetCopyArea(
      x, y, image->rect_, Rectangle::FromSize(u, v, width, height));

  int32_t copy_width = copy_area.copy_width;
  int32_t copy_height = copy_area.copy_height;

  if (copy_width <= 0 || copy_height <= 0) {
    return;
  }

  int32_t src_x = copy_area.src_x;
  int32_t src_y = copy_area.src_y;
  int32_t src_width = image->Width();
  int32_t* src_data = image->data_;

  int32_t dst_x = copy_area.dst_x;
  int32_t dst_y = copy_area.dst_y;
  int32_t dst_width = Width();
  int32_t dst_height = Height();
  int32_t* dst_data = data_;

  for (int32_t i = 0; i < copy_height; i++) {
    int32_t src_index = src_width * (src_y + i) + src_x;
    int32_t dst_index = dst_width * (dst_y + i) + dst_x;

    for (int32_t j = 0; j < copy_width; j++) {
      dst_data[dst_index + j] = src_data[src_index + j];
    }
  }
}

}  // namespace pyxelcore
