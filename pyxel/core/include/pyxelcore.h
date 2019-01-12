#ifndef PYXELCORE_H_
#define PYXELCORE_H_

#ifdef __cplusplus
extern "C" {
#endif

#ifdef WIN32
#ifdef PYXEL_DLL
#define PYXEL_API __declspec(dllexport)
#else
#define PYXEL_API __declspec(dllimport)
#endif
#else
#define PYXEL_API
#endif

PYXEL_API int test(int width, int height);
PYXEL_API void init(int width, int height, const char *caption, int scale,
                    const int *palette, int fps, int border_width,
                    int border_color);

#ifdef __cplusplus
}
#endif

#endif // PYXELCORE_H_
