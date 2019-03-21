#include <cstdio>

#include "pyxelcore.h"
#include "pyxelcore/audio.h"
#include "pyxelcore/constants.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"
#include "pyxelcore/input.h"
#include "pyxelcore/resource.h"
#include "pyxelcore/system.h"

static pyxelcore::Constants* s_constants = NULL;
static pyxelcore::System* s_system = NULL;
static pyxelcore::Resource* s_resource = NULL;
static pyxelcore::Input* s_input = NULL;
static pyxelcore::Graphics* s_graphics = NULL;
static pyxelcore::Audio* s_audio = NULL;

//
// Constants
//
int get_constant_number(char* name) {
  return s_constants->get_constant_number(name);
}

const char* get_constant_string(char* name) {
  return s_constants->get_constant_string(name);
}

//
// System
//
int width_getter() {
  return s_system->width_getter();
}

int height_getter() {
  return s_system->height_getter();
}

int frame_count_getter() {
  return s_system->frame_count_getter();
}

void init(int width,
          int height,
          char* caption,
          int scale,
          int* palette,
          int fps,
          int border_width,
          int border_color) {
  s_constants = new pyxelcore::Constants();
  s_resource = new pyxelcore::Resource();
  s_input = new pyxelcore::Input();
  s_graphics = new pyxelcore::Graphics(width, height);
  s_audio = new pyxelcore::Audio();
  s_system = new pyxelcore::System(s_graphics, width, height, caption, scale,
                                   palette, fps, border_width, border_color);
}

void run(void (*update)(), void (*draw)()) {
  printf("start of run\n");
  s_system->run(update, draw);
}

void quit() {
  s_system->quit();

  delete s_system;
  delete s_audio;
  delete s_graphics;
  delete s_input;
  delete s_resource;
  delete s_constants;
}

//
// Resource
//
void save(char* filename) {
  s_resource->save(filename);
}

void load(char* filename) {
  s_resource->load(filename);
}

//
// Input
//
int mouse_x_getter() {
  return s_input->mouse_x_getter();
}
int mouse_y_getter() {
  return s_input->mouse_y_getter();
}

int btn(int key) {
  return s_input->btn(key);
}

int btnp(int key, int hold, int period) {
  return s_input->btnp(key, hold, period);
}

int btnr(int key) {
  return s_input->btnr(key);
}

void mouse(int visible) {
  return s_input->mouse(visible);
}

//
// Graphics
//
void* image(int img, int system) {
  return s_graphics->image(img, system);
}

void* tilemap(int tm) {
  return s_graphics->tilemap(tm);
}

void clip0() {
  s_graphics->clip();
}

void clip(int x1, int y1, int x2, int y2) {
  s_graphics->clip(x1, y1, x2, y2);
}

void pal0() {
  s_graphics->pal();
}

void pal(int col1, int col2) {
  s_graphics->pal(col1, col2);
}

void cls(int col) {
  s_graphics->cls(col);
}

void pix(int x, int y, int col) {
  s_graphics->pix(x, y, col);
}

void line(int x1, int y1, int x2, int y2, int col) {
  s_graphics->line(x1, y1, x2, y2, col);
}

void rect(int x1, int y1, int x2, int y2, int col) {
  s_graphics->rect(x1, y1, x2, y2, col);
}

void rectb(int x1, int y1, int x2, int y2, int col) {
  s_graphics->rectb(x1, y1, x2, y2, col);
}

void circ(int x, int y, int r, int col) {
  s_graphics->circ(x, y, r, col);
}

void circb(int x, int y, int r, int col) {
  s_graphics->circb(x, y, r, col);
}

void blt(int x, int y, int img, int u, int v, int w, int h, int colkey) {
  s_graphics->blt(x, y, img, u, v, w, h, colkey);
}

void bltm(int x, int y, int tm, int u, int v, int w, int h, int colkey) {
  s_graphics->bltm(x, y, tm, u, v, w, h, colkey);
}

void text(int x, int y, int s, int col) {
  s_graphics->text(x, y, s, col);
}

//
// Audio
//
void* sound(int snd, int system) {
  return s_audio->sound(snd, system);
}

void* music(int msc) {
  return s_audio->music(msc);
}

void play(int ch, int snd, int loop) {
  s_audio->play(ch, snd, loop);
}

void playm(int msc, int loop) {
  s_audio->playm(msc, loop);
}

void stop(int ch) {
  s_audio->stop(ch);
}

//
// Image class
//
int Image_width_getter(void* self) {
  return reinterpret_cast<pyxelcore::Image*>(self)->width();
}

int Image_height_getter(void* self) {
  return reinterpret_cast<pyxelcore::Image*>(self)->height();
}

int* Image_data_getter(void* self) {
  return reinterpret_cast<pyxelcore::Image*>(self)->data();
}

int Image_get(void* self, int x, int y) {
  return reinterpret_cast<pyxelcore::Image*>(self)->get(x, y);
}

void Image_set1(void* self, int x, int y, int data) {}

void Image_set(void* self,
               int x,
               int y,
               int* data,
               int data_width,
               int data_height) {}

void Image_load(void* self, int x, int y, char* filename) {}

void Image_copy(void* self, int x, int y, int img, int u, int v, int w, int h) {
}

//
// Tilemap class
//

//
// Sound class
//

//
// Music class
//
