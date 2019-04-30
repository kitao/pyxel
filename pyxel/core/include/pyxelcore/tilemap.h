#ifndef PYXELCORE_TILEMAP_H_
#define PYXELCORE_TILEMAP_H_

#include "pyxelcore/rectangle.h"

namespace pyxelcore {

class Tilemap : public Rectangle {
 public:
  Tilemap(int32_t width, int32_t height);
  ~Tilemap();

  int32_t* Data() const { return data_; }

  int32_t GetValue(int32_t x, int32_t y) const;

  /*
 PYXEL_API int32_t tilemap_get(void* self, int32_t x, int32_t y);
 PYXEL_API void timemap_set1(void* self,
                             int32_t x,
                             int32_t y,
                             int32_t data,
                             int32_t refimg);
 PYXEL_API void timemap_set(void* self,
                            int32_t x,
                            int32_t y,
                            const int32_t* data,
                            int32_t data_width,
                            int32_t data_height,
                            int32_t refimg);
 PYXEL_API void timemap_copy(void* self,
                             int32_t x,
                             int32_t y,
                             int32_t tm,
                             int32_t u,
                             int32_t v,
                             int32_t w,
                             int32_t h);
                             */
 private:
  int32_t* data_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_TILEMAP_H_
