#ifndef PYXELCORE_APP_H_
#define PYXELCORE_APP_H_

#include <cstdint>
#include <string>

#include "pyxelcore/constants.h"

class SDL_Renderer;
class SDL_Window;
class SDL_Texture;

namespace pyxelcore {

class Image;
class Tilemap;
class Sound;
class Music;

class App {
  //
  // System
  //
 public:
  App(int32_t width,
      int32_t height,
      const char* caption = NULL,
      int32_t scale = -1,
      const int32_t* palette_color = NULL,
      int32_t fps = -1,
      int32_t border_width = -1,
      int32_t border_color = -1);
  ~App();

  int32_t Width() { return width_; }
  int32_t Height() { return height_; }
  int32_t FrameCount() { return frame_count_; }

  Image* GetImage(int32_t image_index, bool system = false);
  Tilemap* GetTilemap(int32_t tilemap_index);
  void Run(void (*update)(), void (*draw)());
  void Quit();

 private:
  int32_t width_;
  int32_t height_;
  std::string caption_;
  int32_t scale_;
  int32_t palette_color_[COLOR_COUNT];
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
 public:
  int32_t MouseX() { return mouse_x_; }
  int32_t MouseY() { return mouse_y_; }

  bool IsButtonOn(int32_t key);
  bool IsButtonPressed(int32_t key,
                       int32_t hold_frame = 0,
                       int32_t period_frame = 0);
  bool IsButtonReleased(int32_t key);
  void SetMouseVisibility(int32_t visible);

 private:
  int32_t mouse_x_;
  int32_t mouse_y_;

  //
  // Resource
  //
 public:
  void LoadAsset(const char* filename);
  void SaveAsset(const char* filename);

  //
  // Graphics
  //
 public:
  void ResetClippingArea();
  void SetClippingArea(int32_t x1, int32_t y1, int32_t x2, int32_t y2);

  void ResetPalette();
  void SetPalette(int32_t src_color, int32_t dest_color);

  void Load(int32_t x, int32_t y, const char* filename);

  void Clear(int32_t color);
  void DrawPoint(int32_t x, int32_t y, int32_t color);
  void DrawLine(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t color);
  void DrawRectangle(int32_t x1,
                     int32_t y1,
                     int32_t x2,
                     int32_t y2,
                     int32_t color);
  void DrawRectangleBorder(int32_t x1,
                           int32_t y1,
                           int32_t x2,
                           int32_t y2,
                           int32_t color);
  void DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color);
  void DrawCircleBorder(int32_t x, int32_t y, int32_t radius, int32_t color);
  void DrawImage(int32_t x,
                 int32_t y,
                 Image* image,
                 int32_t u,
                 int32_t v,
                 int32_t width,
                 int32_t height,
                 int32_t color_key = -1);
  void DrawTilemap(int32_t x,
                   int32_t y,
                   Tilemap* tilemap,
                   int32_t u,
                   int32_t v,
                   int32_t width,
                   int32_t height,
                   int32_t colkey = -1);
  void DrawText(int32_t x, int32_t y, const char* text, int32_t color);

 private:
  Image* screen_;
  Image** image_;
  Tilemap** tilemap_;
  int32_t palette_table_[COLOR_COUNT];
  int32_t clip_x1_;
  int32_t clip_y1_;
  int32_t clip_x2_;
  int32_t clip_y2_;

  void SetupFontImage();

  //
  // Audio
  //
 public:
  Sound* GetSound(int32_t sound_index, bool system = false);
  Music* GetMusic(int32_t music_index);
  void PlaySound(int32_t ch, Sound* sound, bool loop = false);
  void PlayMusic(Music* music, bool loop = false);
  void StopPlaying(int32_t ch);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_APP_H_
