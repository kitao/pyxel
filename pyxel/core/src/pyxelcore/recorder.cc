#include "pyxelcore/recorder.h"

#include "pyxelcore/gif_writer.h"
#include "pyxelcore/image.h"

namespace pyxelcore {

Recorder::Recorder(int32_t width,
                   int32_t height,
                   const PaletteColor& palette_color,
                   int32_t fps) {
  width_ = width;
  height_ = height;
  palette_color_ = palette_color;
  fps_ = fps;
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
      0, width_ * SCREEN_CAPTURE_SCALE, height_ * SCREEN_CAPTURE_SCALE, 32,
      SDL_PIXELFORMAT_RGB888);

  SDL_LockSurface(surface);

  int32_t** src_data = captured_images_[cur_frame_]->Data();
  int32_t* dst_data = reinterpret_cast<int32_t*>(surface->pixels);

  int32_t scaled_width = width_ * SCREEN_CAPTURE_SCALE;
  int32_t scaled_height = height_ * SCREEN_CAPTURE_SCALE;

  for (int32_t i = 0; i < scaled_height; i++) {
    for (int32_t j = 0; j < scaled_width; j++) {
      int32_t index = scaled_width * i + j;
      int32_t color =
          src_data[i / SCREEN_CAPTURE_SCALE][j / SCREEN_CAPTURE_SCALE];

      dst_data[index] = palette_color_[color];
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

  std::string filename = GetBaseName() + ".gif";
  GifWriter* gif_writer =
      new GifWriter(filename, width_, height_, palette_color_);

  for (int32_t i = 0; i < frame_count_; i++) {
    int32_t index = (start_frame_ + i) % SCREEN_CAPTURE_COUNT;

    gif_writer->AddFrame(captured_images_[index],
                         captured_frames_[index] * 100.0f / fps_ + 0.5f);
  }

  gif_writer->EndFrame();
  delete gif_writer;

  // optimize the generated GIF file with Gifsicle
  int32_t res = system(("gifsicle -b -O3 -Okeep-empty " + filename).c_str());

  ResetScreenCapture();
}

void Recorder::Update(const Image* screen_image, int32_t update_frame_count) {
  cur_frame_ = (cur_frame_ + 1) % SCREEN_CAPTURE_COUNT;
  frame_count_++;
  captured_images_[cur_frame_]->CopyImage(0, 0, screen_image, 0, 0, width_,
                                          height_);
  captured_frames_[cur_frame_] = update_frame_count;

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
