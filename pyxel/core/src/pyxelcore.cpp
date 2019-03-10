#include <cstdio>

#include "pyxelcore.h"
#include "pyxelcore/audio.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/input.h"
#include "pyxelcore/resource.h"
#include "pyxelcore/system.h"

static pyxelcore::System *s_system = NULL;
static pyxelcore::Resource *s_resource = NULL;
static pyxelcore::Input *s_input = NULL;
static pyxelcore::Graphics *s_graphics = NULL;
static pyxelcore::Audio *s_audio = NULL;

//
// System
//
int width_getter() { return s_system->width_getter(); }
int height_getter() { return s_system->height_getter(); }
int frame_count_getter() { return s_system->frame_count_getter(); }

void init(int width, int height, char *caption, int scale, int *palette,
          int fps, int border_width, int border_color) {
  s_resource = new pyxelcore::Resource();
  s_input = new pyxelcore::Input();
  s_graphics = new pyxelcore::Graphics(width, height);
  s_audio = new pyxelcore::Audio();
  s_system = new pyxelcore::System(s_graphics, width, height, caption, scale,
                                   palette, fps, border_width, border_color);
}

void run(void (*update)(), void (*draw)()) { s_system->run(update, draw); }

void quit() {
  s_system->quit();

  delete s_resource;
  delete s_input;
  delete s_graphics;
  delete s_audio;
  delete s_system;
}

//
// Resource
//

//
// Input
//

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

//
// Image class
//

//
// Tilemap class
//

//
// Sound class
//

//
// Music class
//
