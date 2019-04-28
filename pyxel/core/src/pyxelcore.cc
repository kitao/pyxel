#include "pyxelcore.h"

#include "pyxelcore/audio.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"
#include "pyxelcore/input.h"
#include "pyxelcore/resource.h"
#include "pyxelcore/system.h"
#include "pyxelcore/utilities.h"

static pyxelcore::System* s_system = NULL;
static pyxelcore::Resource* s_resource = NULL;
static pyxelcore::Input* s_input = NULL;
static pyxelcore::Graphics* s_graphics = NULL;
static pyxelcore::Audio* s_audio = NULL;

//
// System
//
int32_t width_getter() {
  return s_system->Width();
}

int32_t height_getter() {
  return s_system->Height();
}

int32_t frame_count_getter() {
  return s_system->FrameCount();
}

void init(int32_t width,
          int32_t height,
          const char* caption,
          int32_t scale,
          const int32_t* palette,
          int32_t fps,
          int32_t border_width,
          int32_t border_color) {
  s_system = new pyxelcore::System(width, height, caption, scale, palette, fps,
                                   border_width, border_color);
  s_resource = s_system->Resource();
  s_input = s_system->Input();
  s_graphics = s_system->Graphics();
  s_audio = s_system->Audio();
}

void run(void (*update)(), void (*draw)()) {
  s_system->Run(update, draw);
}

void quit() {
  delete s_system;
}

//
// Resource
//
void save(const char* filename) {
  s_resource->SaveAsset(filename);
}

void load(const char* filename) {
  s_resource->LoadAsset(filename);
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
  return s_input->IsButtonOn(key);
}

int32_t btnp(int32_t key, int32_t hold, int32_t period) {
  return s_input->IsButtonPressed(key, hold, period);
}

int32_t btnr(int32_t key) {
  return s_input->IsButtonReleased(key);
}

void mouse(int32_t visible) {
  return s_input->SetMouseVisibility(visible);
}

//
// Graphics
//
void* image(int32_t img, int32_t system) {
  return s_graphics->GetImage(img, system);
}

void* tilemap(int32_t tm) {
  return s_graphics->GetTilemap(tm);
}

void clip0() {
  s_graphics->ResetClippingArea();
}

void clip(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  s_graphics->SetClippingArea(x1, y1, x2, y2);
}

void pal0() {
  s_graphics->ResetPalette();
}

void pal(int32_t col1, int32_t col2) {
  s_graphics->SetPalette(col1, col2);
}

void cls(int32_t col) {
  s_graphics->Clear(col);
}

void pix(int32_t x, int32_t y, int32_t col) {
  s_graphics->DrawPoint(x, y, col);
}

void line(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col) {
  s_graphics->DrawLine(x1, y1, x2, y2, col);
}

void rect(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col) {
  s_graphics->DrawRectangle(x1, y1, x2, y2, col);
}

void rectb(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col) {
  s_graphics->DrawRectangleBorder(x1, y1, x2, y2, col);
}

void circ(int32_t x, int32_t y, int32_t r, int32_t col) {
  s_graphics->DrawCircle(x, y, r, col);
}

void circb(int32_t x, int32_t y, int32_t r, int32_t col) {
  s_graphics->DrawCircleBorder(x, y, r, col);
}

void blt(int32_t x,
         int32_t y,
         int32_t img,
         int32_t u,
         int32_t v,
         int32_t w,
         int32_t h,
         int32_t colkey) {
  s_graphics->DrawImage(x, y, s_graphics->GetImage(img),
                        pyxelcore::Rectangle::FromSize(u, v, w, h), colkey);
}

void bltm(int32_t x,
          int32_t y,
          int32_t tm,
          int32_t u,
          int32_t v,
          int32_t w,
          int32_t h,
          int32_t colkey) {
  s_graphics->DrawTilemap(x, y, s_graphics->GetTilemap(tm),
                          pyxelcore::Rectangle::FromSize(u, v, w, h), colkey);
}

void text(int32_t x, int32_t y, const char* s, int32_t col) {
  s_graphics->DrawText(x, y, s, col);
}

//
// Audio
//
void* sound(int32_t snd, int32_t system) {
  return s_audio->GetSound(snd, system);
}

void* music(int32_t msc) {
  return s_audio->GetMusic(msc);
}

void play(int32_t ch, int32_t snd, int32_t loop) {
  s_audio->PlaySound(snd, loop);
}

void playm(int32_t msc, int32_t loop) {
  s_audio->PlayMusic(msc, loop);
}

void stop(int32_t ch) {
  s_audio->StopPlaying(ch);
}

//
// Image class
//
int32_t image_width_getter(void* self) {
  return reinterpret_cast<pyxelcore::Image*>(self)->Width();
}

int32_t image_height_getter(void* self) {
  return reinterpret_cast<pyxelcore::Image*>(self)->Height();
}

int32_t* image_data_getter(void* self) {
  return reinterpret_cast<pyxelcore::Image*>(self)->Data();
}

int32_t image_get(void* self, int32_t x, int32_t y) {
  return reinterpret_cast<pyxelcore::Image*>(self)->GetColor(x, y);
}

void image_set1(void* self, int32_t x, int32_t y, int32_t color) {
  reinterpret_cast<pyxelcore::Image*>(self)->SetColor(x, y, color);
}

void image_set(void* self,
               int32_t x,
               int32_t y,
               const int32_t* data,
               int32_t data_width,
               int32_t data_height) {
  reinterpret_cast<pyxelcore::Image*>(self)->SetData(x, y, data, data_width,
                                                     data_height);
}

void image_load(void* self, int32_t x, int32_t y, const char* filename) {
  reinterpret_cast<pyxelcore::Image*>(self)->LoadImage(
      x, y, filename, s_system->PaletteColor());
}

void image_copy(void* self,
                int32_t x,
                int32_t y,
                int32_t img,
                int32_t u,
                int32_t v,
                int32_t w,
                int32_t h) {
  pyxelcore::Image* image = s_graphics->GetImage(img);
  reinterpret_cast<pyxelcore::Image*>(self)->CopyImage(
      x, y, image, pyxelcore::Rectangle::FromSize(u, v, w, h), *image);
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

//
// Utilities
//
int32_t get_constant_number(const char* name) {
  return pyxelcore::Utilities::GetConstantNumber(name);
}

const char* get_constant_string(const char* name) {
  return pyxelcore::Utilities::GetConstantString(name);
}

void raise_error(const char* msg) {
  pyxelcore::Utilities::RaiseError(msg);
}
