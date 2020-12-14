#ifndef PYXELCORE_H_
#define PYXELCORE_H_

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _WIN32
#ifdef PYXEL_DLL
#define PYXEL_API __declspec(dllexport)
#else
#define PYXEL_API __declspec(dllimport)
#endif
#else
#define PYXEL_API
#endif

//
// Constants
//
PYXEL_API int _get_constant_number(const char* name);
PYXEL_API void _get_constant_string(char* str,
                                    int str_length,
                                    const char* name);

//
// System
//
PYXEL_API int width_getter();
PYXEL_API int height_getter();
PYXEL_API int frame_count_getter();

PYXEL_API void init(int width,
                    int height,
                    const char* caption,
                    int scale,
                    const int* palette,
                    int fps,
                    int quit_key,
                    int fullscreen);
PYXEL_API void run(void (*update)(), void (*draw)());
PYXEL_API int quit();
PYXEL_API int flip();
PYXEL_API void show();

PYXEL_API void _drop_file_getter(char* str, int str_length);
PYXEL_API void _caption(const char* caption);

//
// Resource
//
PYXEL_API void save(const char* filename);
PYXEL_API void load(const char* filename,
                    int image,
                    int tilemap,
                    int sound,
                    int music);

//
// Input
//
PYXEL_API int mouse_x_getter();
PYXEL_API int mouse_y_getter();
PYXEL_API int mouse_wheel_getter();

PYXEL_API int btn(int key);
PYXEL_API int btns(int *keys, int len);
PYXEL_API int btnp(int key, int hold, int period);
PYXEL_API int btnsp(int *keys, int len, int hold, int period);
PYXEL_API int btnr(int key);
PYXEL_API void mouse(int visible);

//
// Graphics
//
PYXEL_API void* image(int img, int system);
PYXEL_API void* tilemap(int tm);
PYXEL_API void clip0();
PYXEL_API void clip(int x, int y, int w, int h);
PYXEL_API void pal0();
PYXEL_API void pal(int col1, int col2);
PYXEL_API void cls(int col);
PYXEL_API int pget(int x, int y);
PYXEL_API void pset(int x, int y, int col);
PYXEL_API void line(int x1, int y1, int x2, int y2, int col);
PYXEL_API void rect(int x, int y, int w, int h, int col);
PYXEL_API void rectb(int x, int y, int w, int h, int col);
PYXEL_API void circ(int x, int y, int r, int col);
PYXEL_API void circb(int x, int y, int r, int col);
PYXEL_API void tri(int x1, int y1, int x2, int y2, int x3, int y3, int col);
PYXEL_API void trib(int x1, int y1, int x2, int y2, int x3, int y3, int col);
PYXEL_API void
blt(int x, int y, int img, int u, int v, int w, int h, int colkey);
PYXEL_API void
bltm(int x, int y, int tm, int u, int v, int w, int h, int colkey);
PYXEL_API void text(int x, int y, const char* s, int col);

//
// Audio
//
PYXEL_API void* sound(int snd, int system);
PYXEL_API void* music(int msc);
PYXEL_API int play_pos(int ch);
PYXEL_API void play1(int ch, int snd, int loop);
PYXEL_API void play(int ch, int* snd, int snd_length, int loop);
PYXEL_API void playm(int msc, int loop);
PYXEL_API void stop(int ch);

//
// Image class
//
PYXEL_API int image_width_getter(void* self);
PYXEL_API int image_height_getter(void* self);
PYXEL_API int** image_data_getter(void* self);

PYXEL_API int image_get(void* self, int x, int y);
PYXEL_API void image_set1(void* self, int x, int y, int data);
PYXEL_API void image_set(void* self,
                         int x,
                         int y,
                         const char** data,
                         int data_length);
PYXEL_API void image_load(void* self, int x, int y, const char* filename);
PYXEL_API void
image_copy(void* self, int x, int y, int img, int u, int v, int w, int h);

//
// Tilemap class
//
PYXEL_API int tilemap_width_getter(void* self);
PYXEL_API int tilemap_height_getter(void* self);
PYXEL_API int** tilemap_data_getter(void* self);
PYXEL_API int tilemap_refimg_getter(void* self);
PYXEL_API void tilemap_refimg_setter(void* self, int refimg);

PYXEL_API int tilemap_get(void* self, int x, int y);
PYXEL_API void tilemap_set1(void* self, int x, int y, int data);
PYXEL_API void tilemap_set(void* self,
                           int x,
                           int y,
                           const char** data,
                           int data_length);
PYXEL_API void
tilemap_copy(void* self, int x, int y, int tm, int u, int v, int w, int h);

//
// Sound class
//
PYXEL_API int* sound_note_getter(void* self);
PYXEL_API int sound_note_length_getter(void* self);
PYXEL_API void sound_note_length_setter(void* self, int length);
PYXEL_API int* sound_tone_getter(void* self);
PYXEL_API int sound_tone_length_getter(void* self);
PYXEL_API void sound_tone_length_setter(void* self, int length);
PYXEL_API int* sound_volume_getter(void* self);
PYXEL_API int sound_volume_length_getter(void* self);
PYXEL_API void sound_volume_length_setter(void* self, int length);
PYXEL_API int* sound_effect_getter(void* self);
PYXEL_API int sound_effect_length_getter(void* self);
PYXEL_API void sound_effect_length_setter(void* self, int length);
PYXEL_API int sound_speed_getter(void* self);
PYXEL_API void sound_speed_setter(void* self, int speed);

PYXEL_API void sound_set(void* self,
                         const char* note,
                         const char* tone,
                         const char* volume,
                         const char* effect,
                         int speed);
PYXEL_API void sound_set_note(void* self, const char* note);
PYXEL_API void sound_set_tone(void* self, const char* tone);
PYXEL_API void sound_set_volume(void* self, const char* volume);
PYXEL_API void sound_set_effect(void* self, const char* effect);

//
// Music class
//
PYXEL_API int* music_ch0_getter(void* self);
PYXEL_API int music_ch0_length_getter(void* self);
PYXEL_API void music_ch0_length_setter(void* self, int length);
PYXEL_API int* music_ch1_getter(void* self);
PYXEL_API int music_ch1_length_getter(void* self);
PYXEL_API void music_ch1_length_setter(void* self, int length);
PYXEL_API int* music_ch2_getter(void* self);
PYXEL_API int music_ch2_length_getter(void* self);
PYXEL_API void music_ch2_length_setter(void* self, int length);
PYXEL_API int* music_ch3_getter(void* self);
PYXEL_API int music_ch3_length_getter(void* self);
PYXEL_API void music_ch3_length_setter(void* self, int length);

PYXEL_API void music_set(void* self,
                         const int* ch0,
                         int ch0_length,
                         const int* ch1,
                         int ch1_length,
                         const int* ch2,
                         int ch2_length,
                         const int* ch3,
                         int ch3_length);

#ifdef __cplusplus
}
#endif

#endif  // PYXELCORE_H_
