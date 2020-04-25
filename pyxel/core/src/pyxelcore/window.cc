#include "pyxelcore/window.h"

#include "pyxelcore/image.h"

namespace pyxelcore {

Window::Window(const std::string& caption,
               int32_t screen_width,
               int32_t screen_height,
               int32_t screen_scale,
               const PaletteColor& palette_color) {
  screen_width_ = screen_width;
  screen_height_ = screen_height;
  screen_scale_ = screen_scale;
  palette_color_ = palette_color;
  is_fullscreen_ = false;
  mouse_wheel_ = 0;

  if (screen_scale_ <= 0) {
    SDL_DisplayMode display_mode;
    SDL_GetDesktopDisplayMode(0, &display_mode);

    screen_scale_ = Max(
        Min(display_mode.w / screen_width_, display_mode.h / screen_height_) *
            MAX_WINDOW_SIZE_RATIO,
        1.0f);
  }

  int32_t window_width = screen_width_ * screen_scale_;
  int32_t window_height = screen_height_ * screen_scale_;

  window_ = SDL_CreateWindow(caption.c_str(), SDL_WINDOWPOS_CENTERED,
                             SDL_WINDOWPOS_CENTERED, window_width,
                             window_height, SDL_WINDOW_RESIZABLE);

  renderer_ = SDL_CreateRenderer(window_, -1, 0);

  screen_texture_ = SDL_CreateTexture(renderer_, SDL_PIXELFORMAT_RGB888,
                                      SDL_TEXTUREACCESS_STREAMING,
                                      screen_width_, screen_height_);

  SDL_SetWindowMinimumSize(window_, screen_width_, screen_height_);

  SetupWindowIcon();
  UpdateWindowInfo();
}

void Window::SetupWindowIcon() const {
  SDL_Surface* surface = SDL_CreateRGBSurfaceWithFormat(
      0, ICON_WIDTH * ICON_SCALE, ICON_HEIGHT * ICON_SCALE, 32,
      SDL_PIXELFORMAT_RGBA8888);

  Image* image = new Image(ICON_WIDTH, ICON_HEIGHT);
  image->SetData(0, 0, ICON_DATA);

  int32_t** src_data = image->Data();
  uint32_t* dst_data = reinterpret_cast<uint32_t*>(surface->pixels);

  for (int32_t i = 0; i < ICON_HEIGHT; i++) {
    int32_t index = ICON_WIDTH * i;

    for (int32_t j = 0; j < ICON_WIDTH; j++) {
      int32_t color = src_data[i][j];
      uint32_t argb = color == 0 ? 0 : (palette_color_[color] << 8) + 0xff;

      for (int32_t y = 0; y < ICON_SCALE; y++) {
        int32_t index = (ICON_WIDTH * (i * ICON_SCALE + y) + j) * ICON_SCALE;

        for (int32_t x = 0; x < ICON_SCALE; x++) {
          dst_data[index + x] = argb;
        }
      }
    }
  }

  SDL_SetWindowIcon(window_, surface);
  SDL_FreeSurface(surface);

  delete image;
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
    switch (event.type) {
      case SDL_WINDOWEVENT:
        if (event.window.event == SDL_WINDOWEVENT_MOVED ||
            event.window.event == SDL_WINDOWEVENT_RESIZED) {
          UpdateWindowInfo();
        }
        break;

      case SDL_MOUSEWHEEL:
        mouse_wheel_ += event.wheel.y;
        break;

      case SDL_DROPFILE:
        drop_file_ = event.drop.file;
        break;

      case SDL_QUIT:
        window_should_close = true;
        break;
    }
  }

  return window_should_close;
}

void Window::Render(int32_t** screen_data) {
  uint8_t r = (WINDOW_BACKGROUND_COLOR >> 16) & 0xff;
  uint8_t g = (WINDOW_BACKGROUND_COLOR >> 8) & 0xff;
  uint8_t b = WINDOW_BACKGROUND_COLOR & 0xff;

  SDL_SetRenderDrawColor(renderer_, r, g, b, 255);
  SDL_RenderClear(renderer_);

  UpdateScreenTexture(screen_data);

  SDL_Rect dst_rect = {
      screen_x_,
      screen_y_,
      screen_width_ * screen_scale_,
      screen_height_ * screen_scale_,
  };

  SDL_RenderCopy(renderer_, screen_texture_, NULL, &dst_rect);
  SDL_RenderPresent(renderer_);
}

int32_t Window::GetMouseWheel() {
  int32_t mouse_wheel = mouse_wheel_;
  mouse_wheel_ = 0;
  return mouse_wheel;
}

std::string Window::GetDropFile() {
  std::string drop_file = drop_file_;
  drop_file_ = "";
  return drop_file;
}

void Window::SetCaption(const std::string& caption) {
  SDL_SetWindowTitle(window_, caption.c_str());
}

void Window::UpdateScreenTexture(int32_t** screen_data) {
  int32_t* framebuffer;
  int32_t pitch;
  int32_t size = screen_width_ * screen_height_;

  SDL_LockTexture(screen_texture_, NULL, reinterpret_cast<void**>(&framebuffer),
                  &pitch);

  for (int32_t i = 0; i < screen_height_; i++) {
    int32_t index = screen_width_ * i;
    for (int32_t j = 0; j < screen_width_; j++) {
      framebuffer[index + j] = palette_color_[screen_data[i][j]];
    }
  }

  SDL_UnlockTexture(screen_texture_);
}

}  // namespace pyxelcore
