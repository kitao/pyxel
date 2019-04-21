#include "pyxelcore/image.h"

#include "pyxelcore/constants.h"

#include <SDL2/SDL_image.h>
#include <algorithm>
#include <map>

namespace pyxelcore {

Image::Image(int32_t width, int32_t height, void* data)
    : Rectangle(0, 0, width, height) {
  if (Width() <= 0 || Height() <= 0) {
    // error
  }

  if (data) {
    need_to_delete_ = false;
    data_ = reinterpret_cast<int32_t*>(data);
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

int32_t Image::GetColor(int32_t x, int32_t y) {
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

void Image::SetData(int32_t x,
                    int32_t y,
                    const int32_t* data,
                    int32_t data_width,
                    int32_t data_height) {
  if (!Includes(x, y)) {
    // error
  }

  //
}

void Image::LoadImage(int32_t x,
                      int32_t y,
                      const char* filename,
                      const int32_t* palette_color) {
  SDL_Surface* original_image = IMG_Load(filename);
  SDL_Surface* rgb_image =
      SDL_ConvertSurfaceFormat(original_image, SDL_PIXELFORMAT_RGBA8888, 0);

  // TODO: error handling
  int32_t src_w = rgb_image->w;
  int32_t src_h = rgb_image->h;
  int32_t dest_w = Width();
  int32_t dest_h = Height();

  Rectangle copy_rect =
      Rectangle::FromSize(0, 0, src_w, src_h)
          .Intersect(Rectangle::FromSize(x, y, dest_w, dest_h));

  if (copy_rect.IsEmpty()) {
    SDL_FreeSurface(rgb_image);
    SDL_FreeSurface(original_image);

    return;
  }

  int32_t offset_x = copy_rect.Left() - x;
  int32_t offset_y = copy_rect.Top() - y;

  int32_t src_x = offset_x;
  int32_t src_y = offset_y;
  uint8_t* src_data = reinterpret_cast<uint8_t*>(rgb_image->pixels);
  int32_t src_pitch = rgb_image->pitch;

  int32_t dest_x = x + offset_x;
  int32_t dest_y = y + offset_y;
  int32_t* dest_data = data_;

  int32_t copy_w = copy_rect.Width();
  int32_t copy_h = copy_rect.Height();

  // std::map<int32_t, int32_t> color_table;

  for (int32_t i = 0; i < copy_h; i++) {
    int32_t src_index = src_pitch * (src_y + i) + src_x * 4;
    int32_t dest_index = dest_w * (dest_y + i) + dest_x;

    for (int32_t j = 0; j < copy_w; j++) {
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
        int32_t color_dist = dr * dr * 1 + dg * dg * 1 + db * db * 1;

        if (color_dist < nearest_color_dist) {
          nearest_color = k;
          nearest_color_dist = color_dist;
        }
      }

      dest_data[dest_index + j] = nearest_color;
    }
  }

  SDL_FreeSurface(rgb_image);
  SDL_FreeSurface(original_image);
}

void Image::CopyImage(int32_t x,
                      int32_t y,
                      Image* image,
                      int32_t u,
                      int32_t v,
                      int32_t w,
                      int32_t h,
                      int32_t color_key) {}

}  // namespace pyxelcore
