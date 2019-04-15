#ifndef PYXELCORE_SYSTEM_H_
#define PYXELCORE_SYSTEM_H_

#include "pyxelcore/constants.h"
#include "pyxelcore/image.h"

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

class Resource;
class Input;
class Graphics;
class Audio;

class System {
 public:
  struct WindowInfo {
    int32_t window_x;
    int32_t window_y;
    int32_t window_width;
    int32_t window_height;
    int32_t screen_x;
    int32_t screen_y;
    int32_t screen_scale;
    SDL_Window* window;
    SDL_Renderer* renderer;
    SDL_Texture* screen_texture;
  };

  System(int32_t width,
         int32_t height,
         const char* caption = NULL,
         int32_t scale = -1,
         const int32_t* palette_color = NULL,
         int32_t fps = -1,
         int32_t border_width = -1,
         int32_t border_color = -1);
  ~System();

  Resource* Resource() { return resource_; }
  Input* Input() { return input_; }
  Graphics* Graphics() { return graphics_; }
  Audio* Audio() { return audio_; }
  struct WindowInfo* WindowInfo() {
    return &window_info_;
  }
  const int32_t* PaletteColor() { return palette_color_; }

  int32_t Width() { return width_; }
  int32_t Height() { return height_; }
  int32_t FrameCount() { return frame_count_; }

  void Run(void (*update)(), void (*draw)());

 private:
  pyxelcore::Input* input_;
  pyxelcore::Resource* resource_;
  pyxelcore::Graphics* graphics_;
  pyxelcore::Audio* audio_;

  int32_t width_;
  int32_t height_;
  std::string caption_;
  int32_t fps_;
  int32_t border_width_;
  int32_t border_color_;
  int32_t frame_count_;
  struct WindowInfo window_info_;
  int32_t palette_color_[COLOR_COUNT];

  void SetupWindow();
  void RenderWindow();
  void UpdateScreenTexture();
  void UpdateWindowInfo();
};

}  // namespace pyxelcore

#endif  // PYXELCORE_SYSTEM_H_
