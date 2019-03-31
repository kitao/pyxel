#ifndef PYXELCORE_CANVAS_H_
#define PYXELCORE_CANVAS_H_

#include <cstddef>
#include <cstdint>

namespace pyxelcore {

class Tilemap;

class Canvas {
 public:
  Canvas(int32_t width,
         int32_t height,
         int32_t color_count,
         int32_t* data = NULL);
  ~Canvas();

  int32_t Width() { return width_; }
  int32_t Height() { return height_; }
  int32_t* Data() { return data_; }

  int32_t GetColor(int32_t x, int32_t y);
  void SetColor(int32_t x, int32_t y, int32_t color);
  void SetData(int32_t x,
               int32_t y,
               const int32_t* data,
               int32_t data_width,
               int32_t data_height);

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
                 const Canvas* image,
                 int32_t u,
                 int32_t v,
                 int32_t width,
                 int32_t height,
                 int32_t color_key = -1);
  void DrawTilemap(int32_t x,
                   int32_t y,
                   const Tilemap* tilemap,
                   int32_t u,
                   int32_t v,
                   int32_t width,
                   int32_t height,
                   int32_t colkey = -1);
  void DrawText(int32_t x, int32_t y, const char* text, int32_t color);

 private:
  int32_t width_;
  int32_t height_;
  int32_t color_count_;
  int32_t* data_;
  int32_t* palette_;
  bool need_to_delete_data_;

  struct ClippingArea {
    int32_t x1;
    int32_t y1;
    int32_t x2;
    int32_t y2;
  } clipping_area_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_CANVAS_H_
