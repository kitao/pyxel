#ifndef PYXELCORE_GRAPHICS_H_
#define PYXELCORE_GRAPHICS_H_

#include <cstdint>

#include "pyxelcore/constants.h"

namespace pyxelcore {

class Image;
class Tilemap;

class Graphics {
 public:
  Graphics(int32_t width, int32_t height);
  ~Graphics();

  Image* Screen() { return screen_; }

  Image* GetImage(int32_t image_index, bool system = false);
  Tilemap* GetTilemap(int32_t tilemap_index);

  void ResetClippingArea();
  void SetClippingArea(int32_t x1, int32_t y1, int32_t x2, int32_t y2);

  void ResetPalette();
  void SetPalette(int32_t src_color, int32_t dest_color);

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
                 int32_t image_index,
                 int32_t u,
                 int32_t v,
                 int32_t width,
                 int32_t height,
                 int32_t color_key = -1);
  void DrawTilemap(int32_t x,
                   int32_t y,
                   int32_t tilemap_index,
                   int32_t u,
                   int32_t v,
                   int32_t width,
                   int32_t height,
                   int32_t colkey = -1);
  void DrawText(int32_t x, int32_t y, const char* text, int32_t color);

 private:
  Image* screen_;
  int32_t width_;
  int32_t height_;
  Image** image_;
  Tilemap** tilemap_;
  int32_t palette_table_[COLOR_COUNT];
  int32_t clip_x1_;
  int32_t clip_y1_;
  int32_t clip_x2_;
  int32_t clip_y2_;

  void SetupFontImage();
};

}  // namespace pyxelcore

#endif  // PYXELCORE_GRAPHICS_H_
