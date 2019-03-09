#include <cstdio>

#include "pyxelcore.h"
#include "pyxelcore/app.h"
#include "pyxelcore/graphics.h"

static pyxelcore::App *s_app = NULL;
static pyxelcore::Graphics *s_graphics = NULL;

//
// System
//
int width_getter() { return s_app->width_getter(); }
int height_getter() { return s_app->height_getter(); }
int frame_count_getter() { return s_app->frame_count_getter(); }

void init(int width, int height, char *caption, int scale, int *palette,
          int fps, int border_width, int border_color) {
  s_app = new pyxelcore::App(width, height, caption, scale, palette, fps,
                             border_width, border_color);

  s_graphics = s_app->Graphics();
}

void run(void (*update)(), void (*draw)()) { s_app->run(update, draw); }

void quit() { s_app->quit(); }

//
// Resource
//
void save(char *filename) {}
void load(char *filename) {}

//
// Input
//
int mouse_x_getter() { return 0; }
int mouse_y_getter() { return 0; }

int btn(int key) { return 0; }
int btnp(int key, int hold, int period) { return 0; }
int btnr(int key) { return 0; }
void mouse(int visible) {}

//
// Graphics
//
void *image(int img, int system) { return NULL; }
void *tilemap(int tm) { return NULL; }
void clip0() { s_graphics->clip(); }
void clip(int x1, int y1, int x2, int y2) { s_graphics->clip(x1, y1, x2, y2); }
void pal0() { s_graphics->pal(); }
void pal(int col1, int col2) { s_graphics->pal(col1, col2); }
void cls(int col) { s_graphics->cls(col); }
void pix(int x, int y, int col) { s_graphics->pix(x, y, col); }
void line(int x1, int y1, int x2, int y2, int col) {
  s_graphics->line(x1, y1, x2, y2, col);
}
void rect(int x1, int y1, int x2, int y2, int col) {
  s_graphics->rect(x1, y1, x2, y2, col);
}
void rectb(int x1, int y1, int x2, int y2, int col) {
  s_graphics->rectb(x1, y1, x2, y2, col);
}
void circ(int x, int y, int r, int col) { s_graphics->circ(x, y, r, col); }
void circb(int x, int y, int r, int col) { s_graphics->circb(x, y, r, col); }
void blt(int x, int y, int img, int u, int v, int w, int h, int colkey) {
  s_graphics->blt(x, y, img, u, v, w, h, colkey);
}
void bltm(int x, int y, int tm, int u, int v, int w, int h, int colkey) {
  s_graphics->bltm(x, y, tm, u, v, w, h, colkey);
}
void text(int x, int y, int s, int col) { s_graphics->text(x, y, s, col); }

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
int Image_width_getter(void *self) { return 0; }
int Image_height_getter(void *self) { return 0; }
int *Image_data_getter(void *self) { return NULL; }

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
int Tilemap_width_getter(void *self) { return 0; }
int Tilemap_height_getter(void *self) { return 0; }
int *Tilemap_data_getter(void *self) { return NULL; }

int Tilemap_get(int x, int y) { return 0; }
void Timemap_set1(int x, int y, int data, int refimg) {}
void Timemap_set(int x, int y, int *data, int data_width, int data_height,
                 int refimg) {}
void Timemap_copy(int x, int y, int tm, int u, int v, int w, int h) {}

//
// Sound class
//
int *Sound_note_getter(void *self, int *length) { return NULL; }
void Sound_note_setter(void *self, int length) {}
int *Sound_tone_getter(void *self, int *length) { return NULL; }
void Sound_tone_setter(void *self, int length) {}
int *Sound_volume_getter(void *self, int *length) { return NULL; }
void Sound_volume_setter(void *self, int length) {}
int *Sound_effect_getter(void *self, int *length) { return NULL; }
void Sound_effect_setter(void *self, int length) {}
int Sound_speed_getter(void *self) { return 0; }
void Sound_speed_setter(void *self, int speed) {}

void Sound_set(void *self, char *note, char *tone, char *volume, char *effect,
               int speed) {}
void Sound_set_note(void *self, char *data) {}
void Sound_set_tone(void *self, char *data) {}
void Sound_set_volume(void *self, char *data) {}
void Sound_set_effect(void *self, char *data) {}

//
// Music class
//
int *Music_ch0_getter(void *self, int *length) { return NULL; }
void Music_ch0_setter(void *self, int length) {}
int *Music_ch1_getter(void *self, int *length) { return NULL; }
void Music_ch1_setter(void *self, int length) {}
int *Music_ch2_getter(void *self, int *length) { return NULL; }
void Music_ch2_setter(void *self, int length) {}
int *Music_ch3_getter(void *self, int *length) { return NULL; }
void Music_ch3_setter(void *self, int length) {}

void Music_set(void *self, int *ch0, int ch0_length, int *ch1, int ch1_length,
               int *ch2, int ch2_length, int *ch3, int ch3_length) {}
void Music_set_ch0(void *self, int *data, int data_length) {}
void Music_set_ch1(void *self, int *data, int data_length) {}
void Music_set_ch2(void *self, int *data, int data_length) {}
void Music_set_ch3(void *self, int *data, int data_length) {}
