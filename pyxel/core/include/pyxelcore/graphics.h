#ifndef PYXELCORE_GRAPHICS_H_
#define PYXELCORE_GRAPHICS_H_

#include <cstdint>

namespace pyxelcore {

class Image;
class Tilemap;

class Graphics {
 public:
  Graphics(int32_t width, int32_t height);
  ~Graphics();

  Image* Screen() { return screen_; }

  Image* GetImage(int32_t img, bool system = false);
  Tilemap* GetTilemap(int32_t tm);
  void DrawTilemap(int32_t x,
                   int32_t y,
                   int32_t tm,
                   int32_t u,
                   int32_t v,
                   int32_t width,
                   int32_t height,
                   int32_t colkey = -1);
  void DrawText(int32_t x, int32_t y, const char* text, int32_t color);

 private:
  Image* screen_;
  Image** image_bank_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_GRAPHICS_H_
