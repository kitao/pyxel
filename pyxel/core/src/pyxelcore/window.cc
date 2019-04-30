#include "pyxelcore/window.h"

#include "pyxelcore/constants.h"
#include "pyxelcore/utilities.h"

#include <SDL2/SDL.h>
#include <SDL2/SDL_image.h>

namespace pyxelcore {

Window::Window(const char* caption,
               int32_t screen_width,
               int32_t screen_height,
               int32_t screen_scale,
               int32_t border_width,
               int32_t border_color) {
  if (screen_width < MIN_SCREEN_SIZE || screen_width > MAX_SCREEN_SIZE ||
      screen_height < MIN_SCREEN_SIZE || screen_height > MAX_SCREEN_SIZE) {
    PRINT_ERROR("invalide screen size");
    screen_width = Clamp(screen_width, MIN_SCREEN_SIZE, MAX_SCREEN_SIZE);
    screen_height = Clamp(screen_height, MIN_SCREEN_SIZE, MAX_SCREEN_SIZE);
  }

  if (border_width < 0) {
    PRINT_ERROR("invalide boader width");
    border_width = 0;
  }

  screen_width_ = screen_width;
  screen_height_ = screen_height;
  screen_scale_ = screen_scale;
  border_width_ = border_width;
  border_color_ = border_color;

  SDL_Init(SDL_INIT_VIDEO);  // TODO: error handling
  IMG_Init(IMG_INIT_PNG);    // TODO: erro handling

  if (screen_scale_ <= 0) {
    SDL_DisplayMode display_mode;
    SDL_GetDesktopDisplayMode(0, &display_mode);

    screen_scale_ = Min((display_mode.w - border_width_ * 2) / screen_width_,
                        (display_mode.h - border_width_ * 2) / screen_height_);
  }

  window_ =
      SDL_CreateWindow(caption, SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED,
                       screen_width_ * screen_scale_,
                       screen_height_ * screen_scale_, SDL_WINDOW_RESIZABLE);

  renderer_ = SDL_CreateRenderer(window_, -1, 0);

  screen_texture_ = SDL_CreateTexture(renderer_, SDL_PIXELFORMAT_RGB888,
                                      SDL_TEXTUREACCESS_STREAMING,
                                      screen_width_, screen_height_);

  SDL_SetWindowMinimumSize(window_, screen_width_, screen_height_);

  UpdateInfo();
}

void Window::UpdateInfo() {
  SDL_GetWindowPosition(window_, &window_x_, &window_y_);

  int32_t window_width, window_height;
  SDL_GetWindowSize(window_, &window_width, &window_height);

  screen_scale_ = Min((window_width - border_width_ * 2) / screen_width_,
                      (window_height - border_width_ * 2) / screen_height_);

  screen_x_ = (window_width - screen_width_ * screen_scale_) / 2;
  screen_y_ = (window_height - screen_height_ * screen_scale_) / 2;
}

bool Window::ProcessEvents() {
  SDL_Event event;
  bool window_should_close = false;

  while (SDL_PollEvent(&event)) {
    if (event.type == SDL_QUIT) {
      window_should_close = true;
    } else if (event.type == SDL_WINDOWEVENT) {
      if (event.window.event == SDL_WINDOWEVENT_MOVED ||
          event.window.event == SDL_WINDOWEVENT_RESIZED) {
        UpdateInfo();
      }
    }
  }

  return window_should_close;
}

void Window::Render(const int32_t* screen_data, const int32_t* palette_color) {
  uint8_t r = (border_color_ >> 16) & 0xff;
  uint8_t g = (border_color_ >> 8) & 0xff;
  uint8_t b = border_color_ & 0xff;

  SDL_SetRenderDrawColor(renderer_, r, g, b, 255);
  SDL_RenderClear(renderer_);

  UpdateScreenTexture(screen_data, palette_color);

  SDL_Rect dest_rect = {
      screen_x_,
      screen_y_,
      screen_width_ * screen_scale_,
      screen_height_ * screen_scale_,
  };

  SDL_RenderCopy(renderer_, screen_texture_, NULL, &dest_rect);

  SDL_RenderPresent(renderer_);
}

void Window::UpdateScreenTexture(const int32_t* screen_data,
                                 const int32_t* palette_color) {
  int32_t* pixels;
  int32_t pitch;
  int32_t size = screen_width_ * screen_height_;

  SDL_LockTexture(screen_texture_, NULL, (void**)&pixels, &pitch);

  for (int32_t i = 0; i < size; i++) {
    pixels[i] = palette_color[screen_data[i]];
  }

  SDL_UnlockTexture(screen_texture_);
}

}  // namespace pyxelcore
