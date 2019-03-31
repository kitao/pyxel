#include <cstdio>

#include "pyxelcore.h"
#include "pyxelcore/app.h"
#include "pyxelcore/audio.h"
#include "pyxelcore/canvas.h"
#include "pyxelcore/input.h"

static pyxelcore::App* s_app = NULL;
static pyxelcore::Canvas* s_screen = NULL;
static pyxelcore::Input* s_input = NULL;
static pyxelcore::Audio* s_audio = NULL;

//
// System
//
int32_t width_getter() {
  return s_app->Width();
}

int32_t height_getter() {
  return s_app->Height();
}

int32_t frame_count_getter() {
  return s_app->FrameCount();
}

void init(int32_t width,
          int32_t height,
          const char* caption,
          int32_t scale,
          const int32_t* palette,
          int32_t fps,
          int32_t border_width,
          int32_t border_color) {
  s_input = new pyxelcore::Input();
  s_audio = new pyxelcore::Audio();
  s_app = new pyxelcore::App(width, height, caption, scale, palette, fps,
                             border_width, border_color);
  s_screen = s_app->Screen();
}

void run(void (*update)(), void (*draw)()) {
  s_app->Run(update, draw);
}

void quit() {
  s_app->Quit();

  delete s_app;
  delete s_audio;
  delete s_input;
}

//
// Resource
//
void save(const char* filename) {
  s_app->SaveAsset(filename);
}

void load(const char* filename) {
  s_app->LoadAsset(filename);
}

//
// Input
//
int32_t mouse_x_getter() {
  return s_input->MouseX();
}

int32_t mouse_y_getter() {
  return s_input->MouseY();
}

int32_t btn(int32_t key) {
  return s_input->Btn(key);
}

int32_t btnp(int32_t key, int32_t hold, int32_t period) {
  return s_input->Btnp(key, hold, period);
}

int32_t btnr(int32_t key) {
  return s_input->Btnr(key);
}

void mouse(int32_t visible) {
  return s_input->Mouse(visible);
}

//
// Graphics
//
void* image(int32_t img, int32_t system) {
  return s_app->GetImage(img, system);
}

void* tilemap(int32_t tm) {
  return s_app->GetTilemap(tm);
}

void clip0() {
  s_screen->ResetClippingArea();
}

void clip(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  s_screen->SetClippingArea(x1, y1, x2, y2);
}

void pal0() {
  s_screen->ResetPalette();
}

void pal(int32_t col1, int32_t col2) {
  s_screen->SetPalette(col1, col2);
}

void cls(int32_t col) {
  s_screen->Clear(col);
}

void pix(int32_t x, int32_t y, int32_t col) {
  s_screen->DrawPoint(x, y, col);
}

void line(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col) {
  s_screen->DrawLine(x1, y1, x2, y2, col);
}

void rect(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col) {
  s_screen->DrawRectangle(x1, y1, x2, y2, col);
}

void rectb(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col) {
  s_screen->DrawRectangleBorder(x1, y1, x2, y2, col);
}

void circ(int32_t x, int32_t y, int32_t r, int32_t col) {
  s_screen->DrawCircle(x, y, r, col);
}

void circb(int32_t x, int32_t y, int32_t r, int32_t col) {
  s_screen->DrawCircleBorder(x, y, r, col);
}

void blt(int32_t x,
         int32_t y,
         int32_t img,
         int32_t u,
         int32_t v,
         int32_t w,
         int32_t h,
         int32_t colkey) {
  s_screen->DrawImage(x, y, s_app->GetImage(img), u, v, w, h, colkey);
}

void bltm(int32_t x,
          int32_t y,
          int32_t tm,
          int32_t u,
          int32_t v,
          int32_t w,
          int32_t h,
          int32_t colkey) {
  s_screen->DrawTilemap(x, y, s_app->GetTilemap(tm), u, v, w, h, colkey);
}

void text(int32_t x, int32_t y, const char* s, int32_t col) {
  s_screen->DrawText(x, y, s, col);
}

//
// Audio
//
void* sound(int32_t snd, int32_t system) {
  return s_audio->Sound(snd, system);
}

void* music(int32_t msc) {
  return s_audio->Music(msc);
}

void play(int32_t ch, int32_t snd, int32_t loop) {
  s_audio->Play(ch, snd, loop);
}

void playm(int32_t msc, int32_t loop) {
  s_audio->Playm(msc, loop);
}

void stop(int32_t ch) {
  s_audio->Stop(ch);
}

//
// Image class
//
int32_t image_width_getter(void* self) {
  return reinterpret_cast<pyxelcore::Canvas*>(self)->Width();
}

int32_t image_height_getter(void* self) {
  return reinterpret_cast<pyxelcore::Canvas*>(self)->Height();
}

int32_t* image_data_getter(void* self) {
  return reinterpret_cast<pyxelcore::Canvas*>(self)->Data();
}

int32_t image_get(void* self, int32_t x, int32_t y) {
  return reinterpret_cast<pyxelcore::Canvas*>(self)->GetColor(x, y);
}

void image_set1(void* self, int32_t x, int32_t y, int32_t color) {
  reinterpret_cast<pyxelcore::Canvas*>(self)->SetColor(x, y, color);
}

void image_set(void* self,
               int32_t x,
               int32_t y,
               const int32_t* data,
               int32_t data_width,
               int32_t data_height) {
  reinterpret_cast<pyxelcore::Canvas*>(self)->SetData(x, y, data, data_width,
                                                      data_height);
}

void image_load(void* self, int32_t x, int32_t y, const char* filename) {}

void image_copy(void* self,
                int32_t x,
                int32_t y,
                int32_t img,
                int32_t u,
                int32_t v,
                int32_t w,
                int32_t h) {}

//
// Tilemap class
//

//
// Sound class
//

//
// Music class
//
