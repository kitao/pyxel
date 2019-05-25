#include "pyxelcore.h"

#include "pyxelcore/audio.h"
#include "pyxelcore/graphics.h"
#include "pyxelcore/image.h"
#include "pyxelcore/input.h"
#include "pyxelcore/music.h"
#include "pyxelcore/resource.h"
#include "pyxelcore/sound.h"
#include "pyxelcore/system.h"
#include "pyxelcore/tilemap.h"

#define IMAGE reinterpret_cast<pyxelcore::Image*>(self)
#define TILEMAP reinterpret_cast<pyxelcore::Tilemap*>(self)
#define SOUND reinterpret_cast<pyxelcore::Sound*>(self)
#define MUSIC reinterpret_cast<pyxelcore::Music*>(self)

static pyxelcore::System* s_system = NULL;
static pyxelcore::Resource* s_resource = NULL;
static pyxelcore::Input* s_input = NULL;
static pyxelcore::Graphics* s_graphics = NULL;
static pyxelcore::Audio* s_audio = NULL;

//
// Constants
//
int32_t get_constant_number(const char* name) {
  return pyxelcore::GetConstantNumber(name);
}

const char* get_constant_string(const char* name) {
  return pyxelcore::GetConstantString(name);
}

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
  s_system->Quit();
  delete s_system;

  s_system = NULL;
  s_resource = NULL;
  s_input = NULL;
  s_graphics = NULL;
  s_audio = NULL;
}

//
// Resource
//
int32_t save(const char* filename) {
  return s_resource->SaveAsset(filename);
}

int32_t load(const char* filename) {
  return s_resource->LoadAsset(filename);
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
  return s_input->SetMouseVisible(visible);
}

//
// Graphics
//
void* image(int32_t img, int32_t system) {
  return s_graphics->GetImageBank(img, system);
}

void* tilemap(int32_t tm) {
  return s_graphics->GetTilemapBank(tm);
}

void clip0() {
  s_graphics->ResetClipArea();
}

void clip(int32_t x1, int32_t y1, int32_t x2, int32_t y2) {
  s_graphics->SetClipArea(x1, y1, x2, y2);
}

void pal0() {
  s_graphics->ResetPalette();
}

void pal(int32_t col1, int32_t col2) {
  s_graphics->SetPalette(col1, col2);
}

void cls(int32_t col) {
  s_graphics->ClearScreen(col);
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
  s_graphics->DrawImage(x, y, img, u, v, w, h, colkey);
}

void bltm(int32_t x,
          int32_t y,
          int32_t tm,
          int32_t u,
          int32_t v,
          int32_t w,
          int32_t h,
          int32_t colkey) {
  s_graphics->DrawTilemap(x, y, tm, u, v, w, h, colkey);
}

void text(int32_t x, int32_t y, const char* s, int32_t col) {
  s_graphics->DrawText(x, y, s, col);
}

//
// Audio
//
int32_t playing() {
  return s_audio->IsPlaying();
}

int32_t play_pos(int32_t ch) {
  return s_audio->GetPlayPos(ch);
}

void* sound(int32_t snd, int32_t system) {
  return s_audio->GetSoundBank(snd, system);
}

void* music(int32_t msc) {
  return s_audio->GetMusicBank(msc);
}

void play1(int32_t ch, int32_t snd, int32_t loop) {
  s_audio->PlaySound(ch, snd, loop);
}

void play(int32_t ch, int32_t* snd, int32_t snd_length, int32_t loop) {
  s_audio->PlaySound(ch, snd, snd_length, loop);
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
  return IMAGE->Width();
}

int32_t image_height_getter(void* self) {
  return IMAGE->Height();
}

int32_t* image_data_getter(void* self) {
  return IMAGE->Data();
}

int32_t image_get(void* self, int32_t x, int32_t y) {
  return IMAGE->GetValue(x, y);
}

void image_set1(void* self, int32_t x, int32_t y, int32_t data) {
  IMAGE->SetValue(x, y, data);
}

void image_set(void* self,
               int32_t x,
               int32_t y,
               const char** data,
               int32_t data_length) {
  IMAGE->SetData(x, y, data, data_length);
}

int32_t image_load(void* self, int32_t x, int32_t y, const char* filename) {
  return IMAGE->LoadImage(x, y, filename, s_system->PaletteColor());
}

void image_copy(void* self,
                int32_t x,
                int32_t y,
                int32_t img,
                int32_t u,
                int32_t v,
                int32_t w,
                int32_t h) {
  IMAGE->CopyImage(x, y, s_graphics->GetImageBank(img, true), u, v, w, h);
}

//
// Tilemap class
//
int32_t tilemap_width_getter(void* self) {
  return TILEMAP->Width();
}

int32_t tilemap_height_getter(void* self) {
  return TILEMAP->Height();
}

int32_t* tilemap_data_getter(void* self) {
  return TILEMAP->Data();
}

int32_t tilemap_refimg_getter(void* self) {
  return TILEMAP->ImageIndex();
}

void tilemap_refimg_setter(void* self, int32_t refimg) {
  TILEMAP->ImageIndex(refimg);
}

int32_t tilemap_get(void* self, int32_t x, int32_t y) {
  return TILEMAP->GetValue(x, y);
}

void tilemap_set1(void* self, int32_t x, int32_t y, int32_t data) {
  return TILEMAP->SetValue(x, y, data);
}

void tilemap_set(void* self,
                 int32_t x,
                 int32_t y,
                 const char** data,
                 int32_t data_length) {
  return TILEMAP->SetData(x, y, data, data_length);
}

void tilemap_copy(void* self,
                  int32_t x,
                  int32_t y,
                  int32_t tm,
                  int32_t u,
                  int32_t v,
                  int32_t w,
                  int32_t h) {
  return TILEMAP->CopyTilemap(x, y, s_graphics->GetTilemapBank(tm), u, v, w, h);
}

//
// Sound class
//
int32_t* sound_note_getter(void* self) {
  return SOUND->Note();
}

int32_t sound_note_length_getter(void* self) {
  return SOUND->NoteLength();
}

void sound_note_length_setter(void* self, int32_t length) {
  SOUND->NoteLength(length);
}

int32_t* sound_tone_getter(void* self) {
  return SOUND->Tone();
}

int32_t sound_tone_length_getter(void* self) {
  return SOUND->ToneLength();
}

void sound_tone_length_setter(void* self, int32_t length) {
  SOUND->ToneLength(length);
}

int32_t* sound_volume_getter(void* self) {
  return SOUND->Volume();
}

int32_t sound_volume_length_getter(void* self) {
  return SOUND->VolumeLength();
}

void sound_volume_length_setter(void* self, int32_t length) {
  SOUND->VolumeLength(length);
}

int32_t* sound_effect_getter(void* self) {
  return SOUND->Effect();
}

int32_t sound_effect_length_getter(void* self) {
  return SOUND->EffectLength();
}

void sound_effect_length_setter(void* self, int32_t length) {
  SOUND->EffectLength(length);
}

int32_t sound_speed_getter(void* self) {
  return SOUND->Speed();
}

void sound_speed_setter(void* self, int32_t speed) {
  SOUND->Speed(speed);
}

void sound_set(void* self,
               const char* note,
               const char* tone,
               const char* volume,
               const char* effect,
               int32_t speed) {
  SOUND->Set(note, tone, volume, effect, speed);
}

void sound_set_note(void* self, const char* note) {
  SOUND->SetNote(note);
}

void sound_set_tone(void* self, const char* tone) {
  SOUND->SetTone(tone);
}

void sound_set_volume(void* self, const char* volume) {
  SOUND->SetVolume(volume);
}

void sound_set_effect(void* self, const char* effect) {
  SOUND->SetEffect(effect);
}

//
// Music class
//
int32_t* music_ch0_getter(void* self) {
  return MUSIC->Sound(0);
}

int32_t music_ch0_length_getter(void* self) {
  return MUSIC->SoundLength(0);
}

void music_ch0_length_setter(void* self, int32_t length) {
  MUSIC->SoundLength(0, length);
}

int32_t* music_ch1_getter(void* self) {
  return MUSIC->Sound(1);
}

int32_t music_ch1_length_getter(void* self) {
  return MUSIC->SoundLength(1);
}

void music_ch1_length_setter(void* self, int32_t length) {
  MUSIC->SoundLength(1, length);
}

int32_t* music_ch2_getter(void* self) {
  return MUSIC->Sound(2);
}

int32_t music_ch2_length_getter(void* self) {
  return MUSIC->SoundLength(2);
}

void music_ch2_length_setter(void* self, int32_t length) {
  MUSIC->SoundLength(2, length);
}

int32_t* music_ch3_getter(void* self) {
  return MUSIC->Sound(3);
}

int32_t music_ch3_length_getter(void* self) {
  return MUSIC->SoundLength(3);
}

void music_ch3_length_setter(void* self, int32_t length) {
  MUSIC->SoundLength(3, length);
}

void music_set(void* self,
               const int32_t* ch0,
               int32_t ch0_length,
               const int32_t* ch1,
               int32_t ch1_length,
               const int32_t* ch2,
               int32_t ch2_length,
               const int32_t* ch3,
               int32_t ch3_length) {
  MUSIC->Set(ch0, ch0_length, ch1, ch1_length, ch2, ch2_length, ch3,
             ch3_length);
}
