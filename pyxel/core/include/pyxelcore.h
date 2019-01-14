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

//
// System
//
PYXEL_API int Width_Getter();
PYXEL_API int Height_Getter();
PYXEL_API int FrameCount_Getter();

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
PYXEL_API int MouseX_Getter();
PYXEL_API int MouseY_Getter();

PYXEL_API int Btn(int key);
PYXEL_API int Btnp(int key, int hold, int period);
PYXEL_API int Btnr(int key);
PYXEL_API void Mouse(int visible);

//
// Graphics
//
PYXEL_API void *Image(int img, int system);
PYXEL_API void *Tilemap(int tm);
PYXEL_API void Clip(int x1, int y1, int x2, int y2);

//
// Audio
//
PYXEL_API void *Sound(int snd, int system);
PYXEL_API void *Music(int msc);
PYXEL_API void Play(int ch, int snd, int loop);
PYXEL_API void Playm(int msc, int loop);
PYXEL_API void Stop(int ch);

//
// Image class
//
PYXEL_API int Image_Width_Getter(void *self);
PYXEL_API int Image_Height_Getter(void *self);
PYXEL_API int *Image_Data_Getter(void *self);

PYXEL_API int Image_Get(void *self, int x, int y);
PYXEL_API void Image_Set1(void *self, int x, int y, int data);
PYXEL_API void Image_Set(void *self, int x, int y, int *data, int data_width,
                         int data_height);
PYXEL_API void Image_Load(void *self, int x, int y, char *filename);
PYXEL_API void Image_Copy(void *self, int x, int y, int img, int u, int v,
                          int w, int h);

//
// Tilemap class
//
PYXEL_API int Tilemap_Width_Getter(void *self);
PYXEL_API int Tilemap_Height_Getter(void *self);
PYXEL_API int *Tilemap_Data_Getter(void *self);

PYXEL_API int Tilemap_Get(int x, int y);
PYXEL_API void Timemap_Set1(int x, int y, int data, int refimg);
PYXEL_API void Timemap_Set(int x, int y, int *data, int data_width,
                           int data_height, int refimg);
PYXEL_API void Timemap_Copy(int x, int y, int tm, int u, int v, int w, int h);

//
// Sound class
//
PYXEL_API int *Sound_Note_Getter(void *self, int *length);
PYXEL_API void Sound_Note_Setter(void *self, int length);
PYXEL_API int *Sound_Tone_Getter(void *self, int *length);
PYXEL_API void Sound_Tone_Setter(void *self, int length);
PYXEL_API int *Sound_Volume_Getter(void *self, int *length);
PYXEL_API void Sound_Volume_Setter(void *self, int length);
PYXEL_API int *Sound_Effect_Getter(void *self, int *length);
PYXEL_API void Sound_Effect_Setter(void *self, int length);
PYXEL_API int Sound_Speed_Getter(void *self);
PYXEL_API void Sound_Speed_Setter(void *self, int speed);

PYXEL_API void Sound_Set(void *self, char *note, char *tone, char *volume,
                         char *effect, int speed);
PYXEL_API void Sound_SetNote(void *self, char *data);
PYXEL_API void Sound_SetTone(void *self, char *data);
PYXEL_API void Sound_SetVolume(void *self, char *data);
PYXEL_API void Sound_SetEffect(void *self, char *data);

//
// Music class
//
PYXEL_API int *Music_Ch0_Getter(void *self, int *length);
PYXEL_API void Music_Ch0_Setter(void *self, int length);
PYXEL_API int *Music_Ch1_Getter(void *self, int *length);
PYXEL_API void Music_Ch1_Setter(void *self, int length);
PYXEL_API int *Music_Ch2_Getter(void *self, int *length);
PYXEL_API void Music_Ch2_Setter(void *self, int length);
PYXEL_API int *Music_Ch3_Getter(void *self, int *length);
PYXEL_API void Music_Ch3_Setter(void *self, int length);

PYXEL_API void Music_Set(void *self, int *ch0, int ch0_length, int *ch1,
                         int ch1_length, int *ch2, int ch2_length, int *ch3,
                         int ch3_length);
PYXEL_API void Music_SetCh0(void *self, int *data, int data_length);
PYXEL_API void Music_SetCh1(void *self, int *data, int data_length);
PYXEL_API void Music_SetCh2(void *self, int *data, int data_length);
PYXEL_API void Music_SetCh3(void *self, int *data, int data_length);

#ifdef __cplusplus
}
#endif

#endif // PYXELCORE_H_
