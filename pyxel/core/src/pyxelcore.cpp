#include <cstdio>

#include "pyxelcore.h"
#include "pyxelcore/app.h"

static pyxelcore::App *s_app = NULL;

//
// System
//
int get_width() { return s_app->Width(); }
int get_height() { return s_app->Height(); }
int get_frame_count() { return s_app->FrameCount(); }

void init(int width, int height, char *caption, int scale, int *palette,
          int fps, int border_width, int border_color) {
  s_app = new pyxelcore::App(width, height, caption, scale, palette, fps,
                             border_width, border_color);
}

void run(void (*update)(), void (*draw)()) { s_app->Run(update, draw); }

void quit() { s_app->Quit(); }

//
// Resource
//
void save(char *filename) {}
void load(char *filename) {}

//
// Input
//
int get_mouse_x() { return 0; }
int get_mouse_y() { return 0; }

int btn(int key) { return 0; }
int btnp(int key, int hold, int period) { return 0; }
int btnr(int key) { return 0; }
void mouse(int visible) {}

//
// Graphics
//
void *image(int img, int system) { return NULL; }
void *tilemap(int tm) { return NULL; }
void clip(int x1, int y1, int x2, int y2) {}

//
// Audio
//
void *sound(int snd, int system) { return NULL; }
void *music(int msc) { return NULL; }
void play(int ch, int snd, int loop) {}
void playm(int msc, int loop) {}
void stop(int ch) {}

//
// Image class
//
int get_Image_width(void *self) { return 0; }
int get_Image_height(void *self) { return 0; }
int *get_Image_data(void *self) { return NULL; }

int Image_get(void *self, int x, int y) { return 0; }
void Image_set1(void *self, int x, int y, int data) {}
void Image_set(void *self, int x, int y, int *data, int data_width,
               int data_height) {}
void Image_load(void *self, int x, int y, char *filename) {}
void Image_copy(void *self, int x, int y, int img, int u, int v, int w, int h) {
}

//
// Tilemap class
//
int get_Tilemap_width(void *self) { return 0; }
int get_Tilemap_height(void *self) { return 0; }
int *get_Tilemap_data(void *self) { return NULL; }

int Tilemap_get(int x, int y) { return 0; }
void Timemap_set1(int x, int y, int data, int refimg) {}
void Timemap_set(int x, int y, int *data, int data_width, int data_height,
                 int refimg) {}
void Timemap_copy(int x, int y, int tm, int u, int v, int w, int h) {}

//
// Sound class
//
int *get_Sound_note(void *self, int *length) { return NULL; }
void set_Sound_note(void *self, int length) {}
int *get_Sound_tone(void *self, int *length) { return NULL; }
void set_Sound_tone(void *self, int length) {}
int *get_Sound_volume(void *self, int *length) { return NULL; }
void set_Sound_volume(void *self, int length) {}
int *get_Sound_effect(void *self, int *length) { return NULL; }
void set_Sound_effect(void *self, int length) {}
int get_Sound_speed(void *self) { return 0; }
void set_Sound_speed(void *self, int speed) {}

void Sound_set(void *self, char *note, char *tone, char *volume, char *effect,
               int speed) {}
void Sound_set_note(void *self, char *data) {}
void Sound_set_tone(void *self, char *data) {}
void Sound_set_volume(void *self, char *data) {}
void Sound_set_effect(void *self, char *data) {}

//
// Music class
//
int *get_Music_ch0(void *self, int *length) { return NULL; }
void set_Music_ch0(void *self, int length) {}
int *get_Music_ch1(void *self, int *length) { return NULL; }
void set_Music_ch1(void *self, int length) {}
int *get_Music_ch2(void *self, int *length) { return NULL; }
void set_Music_ch2(void *self, int length) {}
int *get_Music_ch3(void *self, int *length) { return NULL; }
void set_Music_ch3(void *self, int length) {}

void Music_set(void *self, int *ch0, int ch0_length, int *ch1, int ch1_length,
               int *ch2, int ch2_length, int *ch3, int ch3_length) {}
void Music_set_ch0(void *self, int *data, int data_length) {}
void Music_set_ch1(void *self, int *data, int data_length) {}
void Music_set_ch2(void *self, int *data, int data_length) {}
void Music_set_ch3(void *self, int *data, int data_length) {}
