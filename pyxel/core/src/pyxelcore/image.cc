#include "pyxelcore/image.h"

namespace pyxelcore {

Image::Image(int32_t width, int32_t height) {
  if (width < 1 || height < 1) {
    PYXEL_ERROR("invalid image size");
  }

  width_ = width;
  height_ = height;
  rect_ = pyxelcore::Rectangle(0, 0, width, height);

  data_ = new int32_t*[height];
  data_[0] = new int32_t[width * height]();
  for (int32_t i = 1; i < height; i++) {
    data_[i] = data_[0] + width * i;
  }
}

Image::~Image() {
  delete[] data_[0];
  delete[] data_;
}

int32_t Image::GetValue(int32_t x, int32_t y) const {
  if (!rect_.Includes(x, y)) {
    PYXEL_ERROR("access to outside image");
  }

  return data_[y][x];
}

void Image::SetValue(int32_t x, int32_t y, int32_t value) {
  if (!rect_.Includes(x, y)) {
    return;
  }

  if (value < 0 || value >= COLOR_COUNT) {
    PYXEL_ERROR("invalid value");
  }

  data_[y][x] = value;
}

void Image::SetData(int32_t x, int32_t y, const ImageString& image_string) {
  int32_t width = image_string[0].size();
  int32_t height = image_string.size();

  if (width < 1 || height < 1) {
    PYXEL_ERROR("invalid value size");
  }

  Image image = Image(width, height);
  int32_t** dst_data = image.data_;

  for (int32_t i = 0; i < height; i++) {
    std::string str = image_string[i];
    int32_t* dst_line = dst_data[i];

    for (int32_t j = 0; j < width; j++) {
      int32_t value = std::stoi(str.substr(j, 1), nullptr, 16);

      if (value < 0 || value >= COLOR_COUNT) {
        PYXEL_ERROR("invalid value");
      }

      dst_line[j] = value;
    }
  }

  CopyImage(x, y, &image, 0, 0, width, height);
}

static double ColorDifference(uint8_t r1,
                              uint8_t g1,
                              uint8_t b1,
                              uint8_t r2,
                              uint8_t g2,
                              uint8_t b2) {
  double dx = (r1 - r2) * 0.30;
  double dy = (g1 - g2) * 0.59;
  double dz = (b1 - b2) * 0.11;

  return dx * dx + dy * dy + dz * dz;
}

void Image::LoadImage(int32_t x,
                      int32_t y,
                      const std::string& filename,
                      const PaletteColor& palette_color) {
  SDL_Surface* png_image = IMG_Load(filename.c_str());

  if (!png_image) {
    PYXEL_ERROR("cannot load image '" + filename + "'");
  }

  SDL_Surface* src_image =
      SDL_ConvertSurfaceFormat(png_image, SDL_PIXELFORMAT_RGBA8888, 0);

  int32_t src_width = src_image->pitch;
  uint8_t* src_data = reinterpret_cast<uint8_t*>(src_image->pixels);

  int32_t width = src_image->w;
  int32_t height = src_image->h;

  Image dst_image = Image(width, height);
  int32_t** dst_data = dst_image.data_;

  for (int32_t i = 0; i < height; i++) {
    int32_t src_index = src_width * i;
    int32_t* dst_line = dst_data[i];

    for (int32_t j = 0; j < width; j++) {
      int32_t src_r = src_data[src_index + j * 4 + 3];
      int32_t src_g = src_data[src_index + j * 4 + 2];
      int32_t src_b = src_data[src_index + j * 4 + 1];

      int32_t nearest_color = 0;
      double nearest_color_dist = DBL_MAX;

      for (int32_t k = 0; k < COLOR_COUNT; k++) {
        int32_t color = palette_color[k];
        int32_t pal_r = (color >> 16) & 0xff;
        int32_t pal_g = (color >> 8) & 0xff;
        int32_t pal_b = color & 0xff;

        double color_dist =
            ColorDifference(src_r, src_g, src_b, pal_r, pal_g, pal_b);

        if (color_dist < nearest_color_dist) {
          nearest_color = k;
          nearest_color_dist = color_dist;
        }
      }

      dst_line[j] = nearest_color;
    }
  }

  CopyImage(x, y, &dst_image, 0, 0, width, height);

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
  Rectangle::CopyArea copy_area =
      rect_.GetCopyArea(x, y, image->Rectangle(), u, v, width, height);

  if (copy_area.IsEmpty()) {
    return;
  }

  int32_t** src_data = image->data_;
  int32_t** dst_data = data_;

  for (int32_t i = 0; i < copy_area.height; i++) {
    int32_t* src_line = src_data[copy_area.v + i];
    int32_t* dst_line = dst_data[copy_area.y + i];

    for (int32_t j = 0; j < copy_area.width; j++) {
      dst_line[copy_area.x + j] = src_line[copy_area.u + j];
    }
  }
}

}  // namespace pyxelcore
