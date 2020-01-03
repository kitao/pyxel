#include "pyxelcore/recorder.h"

#include "pyxelcore/image.h"

#include "gif-h/gif.h"

namespace pyxelcore {

Recorder::Recorder(int32_t width,
                   int32_t height,
                   const PaletteColor& palette_color,
                   int32_t fps) {
  width_ = width;
  height_ = height;
  scaled_width_ = width * SCREEN_CAPTURE_SCALE;
  scaled_height_ = height * SCREEN_CAPTURE_SCALE;
  palette_color_ = palette_color;
  delay_time_ = static_cast<int32_t>((100.0f / fps) + 0.5f);
  cur_frame_ = -1;
  start_frame_ = 0;
  frame_count_ = 0;

  for (int32_t i = 0; i < SCREEN_CAPTURE_COUNT; i++) {
    captured_images_[i] = new Image(width, height);
  }
}

Recorder::~Recorder() {
  for (int32_t i = 0; i < SCREEN_CAPTURE_COUNT; i++) {
    delete captured_images_[i];
  }
}

void Recorder::SaveScreenshot() {
  if (frame_count_ < 1) {
    return;
  }

  SDL_Surface* surface = SDL_CreateRGBSurfaceWithFormat(
      0, scaled_width_, scaled_height_, 32, SDL_PIXELFORMAT_RGB888);

  SDL_LockSurface(surface);

  int32_t** src_data = captured_images_[cur_frame_]->Data();
  int32_t* dst_data = reinterpret_cast<int32_t*>(surface->pixels);

  for (int32_t i = 0; i < height_; i++) {
    for (int32_t j = 0; j < width_; j++) {
      int32_t color = palette_color_[src_data[i][j]];

      for (int32_t y = 0; y < SCREEN_CAPTURE_SCALE; y++) {
        int32_t index = scaled_width_ * (i * SCREEN_CAPTURE_SCALE + y) +
                        j * SCREEN_CAPTURE_SCALE;

        for (int32_t x = 0; x < SCREEN_CAPTURE_SCALE; x++) {
          dst_data[index + x] = color;
        }
      }
    }
  }

  SDL_UnlockSurface(surface);
  IMG_SavePNG(surface, (GetBaseName() + ".png").c_str());
  SDL_FreeSurface(surface);
}

void Recorder::ResetScreenCapture() {
  start_frame_ = (cur_frame_ + 1) % SCREEN_CAPTURE_COUNT;
  frame_count_ = 0;
}

void Recorder::SaveScreenCapture() {
  if (frame_count_ < 1) {
    return;
  }

  uint32_t* dst_data = new uint32_t[scaled_width_ * scaled_height_];
  GifWriter gif;
  GifBegin(&gif, (GetBaseName() + ".gif").c_str(),
           width_ * SCREEN_CAPTURE_SCALE, height_ * SCREEN_CAPTURE_SCALE,
           delay_time_);

  for (int32_t frame = 0; frame < frame_count_; frame++) {
    int32_t** src_data =
        captured_images_[(start_frame_ + frame) % SCREEN_CAPTURE_COUNT]->Data();

    for (int32_t i = 0; i < height_; i++) {
      for (int32_t j = 0; j < width_; j++) {
        int32_t color = palette_color_[src_data[i][j]];

        for (int32_t y = 0; y < SCREEN_CAPTURE_SCALE; y++) {
          int32_t index = scaled_width_ * (i * SCREEN_CAPTURE_SCALE + y) +
                          j * SCREEN_CAPTURE_SCALE;

          for (int32_t x = 0; x < SCREEN_CAPTURE_SCALE; x++) {
            dst_data[index + x] =
                ((color & 0xff) << 16) + (color & 0xff00) + (color >> 16);
          }
        }
      }
    }

    GifWriteFrame(&gif, reinterpret_cast<uint8_t*>(dst_data), scaled_width_,
                  scaled_height_, delay_time_);
  }

  GifEnd(&gif);
  delete[] dst_data;

  ResetScreenCapture();
}

void Recorder::Update(const Image* screen_image) {
  cur_frame_ = (cur_frame_ + 1) % SCREEN_CAPTURE_COUNT;
  frame_count_++;
  captured_images_[cur_frame_]->CopyImage(0, 0, screen_image, 0, 0, width_,
                                          height_);

  if (frame_count_ > SCREEN_CAPTURE_COUNT) {
    start_frame_ = (start_frame_ + 1) % SCREEN_CAPTURE_COUNT;
    frame_count_ = SCREEN_CAPTURE_COUNT;
  }
}

std::string Recorder::GetBaseName() const {
#ifdef WIN32
  std::string desktop_path = getenv("USERPROFILE");
  desktop_path += "\\Desktop\\";
#else
  std::string desktop_path = getenv("HOME");
  desktop_path += "/Desktop/";
#endif

  char basename[30];
  time_t t = std::time(nullptr);
  std::strftime(basename, sizeof(basename), "pyxel-%y%m%d-%H%M%S",
                std::localtime(&t));

  return desktop_path + basename;
}

}  // namespace pyxelcore
