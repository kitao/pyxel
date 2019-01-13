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
PYXEL_API int Width_Get();
PYXEL_API int Height_Get();
PYXEL_API int FrameCount_Get();

PYXEL_API void Init(int width, int height, char *caption, int scale,
                    int *palette, int fps, int border_width, int border_color);
PYXEL_API void Run(void (*update)(), void (*draw)());
PYXEL_API void Quit();

//
// Resource
//
PYXEL_API void Save(char *filename);
PYXEL_API void Load(char *filename);

//
// Input
//
PYXEL_API int MouseX_getter();
PYXEL_API int MouseY_getter();

PYXEL_API int Btn(int key);
PYXEL_API int Btnp(int key, int hold, int period);
PYXEL_API int Btnr(int key);
PYXEL_API void Mouse(int visible);

//
// Graphics
//
PYXEL_API int Image_Width_getter(void *self);
PYXEL_API int Image_Height_getter(void *self);
PYXEL_API int *Image_Data_getter(void *self);

PYXEL_API int Image_Get(void *self, int x, int y);
PYXEL_API void Image_Set1(void *self, int x, int y, int data);
PYXEL_API void Image_Set(void *self, int x, int y, int *data, int dataWidth,
                         int dataHeight);
PYXEL_API void Image_Load(void *self, int x, int y, char *filename);
PYXEL_API void Image_Copy(void *self, int x, int y, int img, int u, int v,
                          int w, int h);

PYXEL_API int Tilemap_Width_getter(void *self);
PYXEL_API int Tilemap_Height_getter(void *self);
PYXEL_API int *Tilemap_Data_getter(void *self);

PYXEL_API int Tilemap_Get(int x, int y);
PYXEL_API void Timemap_Set1(int x, int y, int data, int refimg);
PYXEL_API void Timemap_Set(int x, int y, int *data, int dataWidth,
                           int dataHeight, int refimg);
PYXEL_API void Timemap_Copy(int x, int y, int tm, int u, int v, int w, int h);

PYXEL_API void *Image(int img, int system);
PYXEL_API void *Tilemap(int tm);
PYXEL_API void Clip(int x1, int y1, int x2, int y2);

//
// Audio
//
PYXEL_API int *Sound_Note_getter(void *self, int *length);
PYXEL_API void Sound_Note_setter(void *self, int length);
PYXEL_API int *Sound_Tone_getter(void *self, int *length);
PYXEL_API void Sound_Tone_setter(void *self, int length);
PYXEL_API int *Sound_Volume_getter(void *self, int *length);
PYXEL_API void Sound_Volume_setter(void *self, int length);
PYXEL_API int *Sound_Effect_getter(void *self, int *length);
PYXEL_API void Sound_Effect_setter(void *self, int length);
PYXEL_API int Sound_Speed_getter(void *self);
PYXEL_API void Sound_Speed_setter(void *self, int speed);

PYXEL_API void Sound_Set(void *self, char *note, char *tone, char *volume,
                         char *effect, int speed);
PYXEL_API void Sound_SetNote(void *self, char *data);
PYXEL_API void Sound_SetTone(void *self, char *data);
PYXEL_API void Sound_SetVolume(void *self, char *data);
PYXEL_API void Sound_SetEffect(void *self, char *data);

PYXEL_API void *Sound(int snd, int system);
PYXEL_API void *Music(int msc);
PYXEL_API void Play(int ch, int snd, int loop);
PYXEL_API void Playm(int msc, int loop);
PYXEL_API void Stop(int ch);

#ifdef __cplusplus
}
#endif

#endif // PYXELCORE_H_
