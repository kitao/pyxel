#ifndef PYXELCORE_APP_H_
#define PYXELCORE_APP_H_

#include <cstdint>
#include <string>

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

class Image;
class Tilemap;

class App {
 public:
  App(int32_t width,
      int32_t height,
      const char* caption = NULL,
      int32_t scale = -1,
      const int32_t* palette = NULL,
      int32_t fps = -1,
      int32_t border_width = -1,
      int32_t border_color = -1);
  ~App();

  Image* Screen() { return screen_; }

  //
  // Constants
  //
  int32_t GetConstantNumber(const char* name);
  const char* GetConstantString(const char* name);

  //
  // System
  //
  int32_t Width() { return width_; }
  int32_t Height() { return height_; }
  int32_t FrameCount() { return frame_count_; }

  Image* GetImage(int32_t img, bool system = false);
  Tilemap* GetTilemap(int32_t tm);

  void Run(void (*update)(), void (*draw)());
  void Quit();

  //
  // Input
  //
  int32_t MouseX() { return mouse_x_; }
  int32_t MouseY() { return mouse_y_; }

  bool IsButtonOn(int32_t key);
  bool IsButtonPressed(int32_t key, int32_t hold = 0, int32_t period = 0);
  bool IsButtonReleased(int32_t key);
  void SetMouseVisibility(int32_t visible);

  //
  // Image class
  //
  void LoadImage(Image* image, int32_t x, int32_t y, const char* filename);

  //
  // Resource
  //
  void LoadAsset(const char* filename);
  void SaveAsset(const char* filename);

 private:
  //
  // System
  //
  int32_t width_;
  int32_t height_;
  std::string caption_;
  int32_t scale_;
  int32_t palette_[16];
  int32_t fps_;
  int32_t border_width_;
  int32_t border_color_;
  int32_t frame_count_;

  SDL_Renderer* renderer_;
  SDL_Window* window_;
  SDL_Texture* screen_texture_;

  void UpdateScreenTexture();

  //
  // Input
  //
  int32_t mouse_x_;
  int32_t mouse_y_;

  //
  // Graphics
  //
  Image* screen_;
  Image** image_;
  Tilemap** tilemap_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_APP_H_
