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

//
// System
//
PYXEL_API int Width();
PYXEL_API int Height();
PYXEL_API int FrameCount();

PYXEL_API void Init(int width, int height, const char *caption, int scale,
                    const int *palette, int fps, int border_width,
                    int border_color);
PYXEL_API void Run(void (*update)(), void (*draw)());
PYXEL_API void Quit();

//
// Resource
//
PYXEL_API void Save(const char *filename);
PYXEL_API void Load(const char *filename);

//
// Input
//
PYXEL_API int MouseX();
PYXEL_API int MouseY();

PYXEL_API bool Btn(int key);
PYXEL_API bool Btnp(int key, int hold, int period);
PYXEL_API bool Btnr(int key);
PYXEL_API void Mouse(bool visible);

//
// Graphics
//
PYXEL_API int Image(int img, bool system);
PYXEL_API int Tilemap(int tm);
PYXEL_API void Clip(int x1, int y1, int x2, int y2);

//
// Audio
//
PYXEL_API int Sound(int snd, bool system);
PYXEL_API int Music(int msc);
PYXEL_API void Play(int ch, int snd, bool loop);
PYXEL_API void Playm(int msc, bool loop);
PYXEL_API void Stop(int ch);

#ifdef __cplusplus
}
#endif

#endif // PYXELCORE_H_
