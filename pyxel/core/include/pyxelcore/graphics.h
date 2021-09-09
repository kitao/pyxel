#ifndef PYXELCORE_GRAPHICS_H_
#define PYXELCORE_GRAPHICS_H_

#include "pyxelcore/rectangle.h"

namespace pyxelcore {

class Image;
class Tilemap;

class Graphics {
 public:
  Graphics(int32_t width, int32_t height);
  ~Graphics();

  const Rectangle& ClipArea() const { return clip_area_; }
  const pyxelcore::PaletteTable& PaletteTable() const { return palette_table_; }
  Image* ScreenImage() const { return image_bank_[IMAGE_BANK_FOR_SCREEN]; }

  Image* GetImageBank(int32_t image_index, bool system = false) const;
  Tilemap* GetTilemapBank(int32_t tilemap_index) const;

  void ResetClipArea();
  void SetClipArea(int32_t x, int32_t y, int32_t width, int32_t height);
  void ResetPalette();
  void SetPalette(int32_t src_color, int32_t dst_color);
  void ClearScreen(int32_t color);
  int32_t GetPoint(int32_t x, int32_t y);
  void SetPoint(int32_t x, int32_t y, int32_t color);
  void DrawLine(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t color);
  void DrawRectangle(int32_t x,
                     int32_t y,
                     int32_t width,
                     int32_t height,
                     int32_t color);
  void DrawRectangleBorder(int32_t x,
                           int32_t y,
                           int32_t width,
                           int32_t height,
                           int32_t color);
  void DrawCircle(int32_t x, int32_t y, int32_t radius, int32_t color);
  void DrawCircleBorder(int32_t x, int32_t y, int32_t radius, int32_t color);
  void DrawTriangle(int32_t x1,
                    int32_t y1,
                    int32_t x2,
                    int32_t y2,
                    int32_t x3,
                    int32_t y3,
                    int32_t color);
  void DrawTriangleBorder(int32_t x1,
                          int32_t y1,
                          int32_t x2,
                          int32_t y2,
                          int32_t x3,
                          int32_t y3,
                          int32_t color);
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
                   int32_t color_key = -1);
  void DrawText(int32_t x, int32_t y, const char* text, int32_t color);

 private:
  Image** image_bank_;
  Tilemap** tilemap_bank_;
  int32_t screen_width_;
  int32_t screen_height_;
  int32_t** screen_data_;
  Rectangle clip_area_;
  pyxelcore::PaletteTable palette_table_;

  void SetupMouseCursor();
  void SetupFont();
  int32_t GetDrawColor(int32_t color, const std::string& func_name) const;
  void SetPixel(int32_t x, int32_t y, int32_t color);
};

inline Image* Graphics::GetImageBank(int32_t image_index, bool system) const {
  if (image_index < 0 || image_index >= TOTAL_IMAGE_BANK_COUNT) {
    PYXEL_ERROR("invalid image index");
  }

  if (image_index >= USER_IMAGE_BANK_COUNT && !system) {
    PYXEL_ERROR("access to image bank for system");
  }

  return image_bank_[image_index];
}

inline Tilemap* Graphics::GetTilemapBank(int32_t tilemap_index) const {
  if (tilemap_index < 0 || tilemap_index >= TILEMAP_BANK_COUNT) {
    PYXEL_ERROR("invalid tilemap index");
  }

  return tilemap_bank_[tilemap_index];
}

inline int32_t Graphics::GetDrawColor(int32_t color,
                                      const std::string& func_name) const {
  if (color < 0 || color >= COLOR_COUNT) {
    PyxelError("invalid color", func_name);
  }

  return palette_table_[color];
}

#define GET_DRAW_COLOR(color) GetDrawColor(color, __FUNCTION__)

inline void Graphics::SetPixel(int32_t x, int32_t y, int32_t draw_color) {
  if (clip_area_.Includes(x, y)) {
    screen_data_[y][x] = draw_color;
  }
}

}  // namespace pyxelcore

#endif  // PYXELCORE_GRAPHICS_H_
