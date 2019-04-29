#include "pyxelcore/image.h"

#include "pyxelcore/constants.h"

#include <SDL2/SDL_image.h>
#include <algorithm>
#include <map>

namespace pyxelcore {

Image::Image(int32_t width, int32_t height, int32_t* data)
    : Rectangle(0, 0, width, height) {
  if (Width() <= 0 || Height() <= 0) {
    // error
  }

  if (data) {
    need_to_delete_ = false;
    data_ = data;
  } else {
    need_to_delete_ = true;
    data_ = new int32_t[Width() * Height()];
  }
}

Image::~Image() {
  if (need_to_delete_) {
    delete[] data_;
  }
}

int32_t Image::GetColor(int32_t x, int32_t y) const {
  if (!Includes(x, y)) {
    // error
  }

  return data_[Width() * y + x];
}

void Image::SetColor(int32_t x, int32_t y, int32_t color) {
  if (!Includes(x, y)) {
    // error
  }

  if (color < 0 || color >= COLOR_COUNT) {
    // error
  }

  data_[Width() * y + x] = color;
}

void Image::SetColor(int32_t x,
                     int32_t y,
                     const char** color_str,
                     int32_t color_str_count) {
  int32_t width = strlen(color_str[0]);
  int32_t height = color_str_count;
  Image* image = new Image(width, height);
  int32_t* data = image->Data();

  for (int32_t i = 0; i < height; i++) {
    int32_t index = width * i;
    const char* str = color_str[i];

    for (int32_t j = 0; j < width; j++) {
      int32_t value = str[j];

      if (value >= '0' && value <= '9') {
        value -= '0';
      } else if (value >= 'A' && value <= 'F') {
        value -= 'A';
      } else if (value >= 'a' && value <= 'f') {
        value -= 'a';
      } else {
        // error
      }

      data[index + j] = value;
    }
  }

  DrawImage(x, y, image, Rectangle::FromSize(0, 0, width, height), *this);

  delete image;
}

void Image::LoadImage(int32_t x,
                      int32_t y,
                      const char* filename,
                      const int32_t* palette_color) {
  SDL_Surface* png_image = IMG_Load(filename);

  // TODO: error handling

  SDL_Surface* src_image =
      SDL_ConvertSurfaceFormat(png_image, SDL_PIXELFORMAT_RGBA8888, 0);

  int32_t width = src_image->w;
  int32_t height = src_image->h;

  uint8_t* src_data = reinterpret_cast<uint8_t*>(src_image->pixels);
  int32_t src_pitch = src_image->pitch;

  Image image = Image(width, height);
  int32_t* dest_data = image.Data();

  for (int32_t i = 0; i < height; i++) {
    int32_t src_index = src_pitch * i;
    int32_t dest_index = width * i;

    for (int32_t j = 0; j < width; j++) {
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

  DrawImage(x, y, &image, Rectangle::FromSize(0, 0, width, height), *this);

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
  DrawImage(x, y, image, Rectangle::FromSize(u, v, width, height), *this);
}

void Image::DrawImage(int32_t x,
                      int32_t y,
                      const Image* image,
                      const Rectangle& copy_rect,
                      const Rectangle& clip_rect,
                      const int32_t* palette_table,
                      int32_t color_key) {
  if (color_key != -1 && (color_key < 0 || color_key >= COLOR_COUNT)) {
    // error
  }

  Rectangle src_rect = static_cast<Rectangle>(*image).Intersect(copy_rect);

  x += std::max(src_rect.Left() - copy_rect.Left(), 0);
  y += std::max(src_rect.Top() - copy_rect.Top(), 0);

  Rectangle dest_rect =
      static_cast<Rectangle>(*this).Intersect(clip_rect).Intersect(
          src_rect.MoveTo(x, y));

  if (dest_rect.IsEmpty()) {
    return;
  }

  src_rect =
      dest_rect.MoveTo(copy_rect.Left() + std::max(dest_rect.Left() - x, 0),
                       copy_rect.Top() + std::max(dest_rect.Top() - y, 0));

  int32_t src_x = src_rect.Left();
  int32_t src_y = src_rect.Top();
  int32_t src_w = image->Width();
  int32_t src_h = image->Height();
  int32_t* src_data = image->Data();

  int32_t dest_x = dest_rect.Left();
  int32_t dest_y = dest_rect.Top();
  int32_t dest_w = Width();
  int32_t dest_h = Height();
  int32_t* dest_data = Data();

  int32_t copy_w = dest_rect.Width();
  int32_t copy_h = dest_rect.Height();

  for (int32_t i = 0; i < copy_h; i++) {
    int32_t src_index = src_w * (src_y + i) + src_x;
    int32_t dest_index = dest_w * (dest_y + i) + dest_x;

    for (int32_t j = 0; j < copy_w; j++) {
      int32_t src_color = src_data[src_index + j];

      // TODO: performance improvement
      if (src_color != color_key) {
        dest_data[dest_index + j] =
            palette_table ? palette_table[src_color] : src_color;
      }
    }
  }
}

}  // namespace pyxelcore
