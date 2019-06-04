#include "pyxelcore/window.h"

namespace pyxelcore {

Window::Window(const std::string& caption,
               int32_t screen_width,
               int32_t screen_height,
               int32_t screen_scale,
               int32_t border_width,
               int32_t border_color) {
  if (border_width < 0) {
    PRINT_ERROR("invalid boader width");
    border_width = 0;
  }

  screen_width_ = screen_width;
  screen_height_ = screen_height;
  screen_scale_ = screen_scale;
  border_color_ = border_color;
  is_fullscreen_ = false;

  if (screen_scale_ <= 0) {
    SDL_DisplayMode display_mode;
    SDL_GetDesktopDisplayMode(0, &display_mode);

    screen_scale_ = Min((display_mode.w - border_width * 2) / screen_width_,
                        (display_mode.h - border_width * 2) / screen_height_);
  }

  int32_t window_width = screen_width_ * screen_scale_ + border_width * 2;
  int32_t window_height = screen_height_ * screen_scale_ + border_width * 2;

  window_ = SDL_CreateWindow(caption.c_str(), SDL_WINDOWPOS_CENTERED,
                             SDL_WINDOWPOS_CENTERED, window_width,
                             window_height, SDL_WINDOW_RESIZABLE);

  renderer_ = SDL_CreateRenderer(window_, -1, 0);

  screen_texture_ = SDL_CreateTexture(renderer_, SDL_PIXELFORMAT_RGB888,
                                      SDL_TEXTUREACCESS_STREAMING,
                                      screen_width_, screen_height_);

  SDL_SetWindowMinimumSize(window_, screen_width_, screen_height_);
  SDL_ShowCursor(false);

  UpdateWindowInfo();
}

void Window::UpdateWindowInfo() {
  SDL_GetWindowPosition(window_, &window_x_, &window_y_);

  int32_t window_width, window_height;
  SDL_GetWindowSize(window_, &window_width, &window_height);

  screen_scale_ =
      Min(window_width / screen_width_, window_height / screen_height_);
  screen_x_ = (window_width - screen_width_ * screen_scale_) / 2;
  screen_y_ = (window_height - screen_height_ * screen_scale_) / 2;
}

void Window::ToggleFullscreen() {
  is_fullscreen_ = !is_fullscreen_;

  SDL_SetWindowFullscreen(window_,
                          is_fullscreen_ ? SDL_WINDOW_FULLSCREEN_DESKTOP : 0);
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
        UpdateWindowInfo();
      }
    }
  }

  return window_should_close;
}

void Window::Render(int32_t** screen_data, const PaletteColor& palette_color) {
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

void Window::UpdateScreenTexture(int32_t** screen_data,
                                 const PaletteColor& palette_color) {
  int32_t* framebuffer;
  int32_t pitch;
  int32_t size = screen_width_ * screen_height_;

  SDL_LockTexture(screen_texture_, NULL, reinterpret_cast<void**>(&framebuffer),
                  &pitch);

  for (int32_t i = 0; i < screen_height_; i++) {
    int32_t index = screen_width_ * i;
    for (int32_t j = 0; j < screen_width_; j++) {
      framebuffer[index + j] = palette_color[screen_data[i][j]];
    }
  }

  SDL_UnlockTexture(screen_texture_);
}

}  // namespace pyxelcore
