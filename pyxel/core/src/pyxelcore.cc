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
int32_t _get_constant_number(const char* name) {
  return pyxelcore::GetConstantNumber(name);
}

void _get_constant_string(char* str, int32_t str_length, const char* name) {
  strncpy(str, pyxelcore::GetConstantString(name).c_str(), str_length);
}

//
// System
//
inline pyxelcore::System* GetSystem() {
  if (!s_system) {
    PYXEL_ERROR("uninitialized function call");
  }

  return s_system;
}

int32_t width_getter() {
  return GetSystem()->Width();
}

int32_t height_getter() {
  return GetSystem()->Height();
}

int32_t frame_count_getter() {
  return GetSystem()->FrameCount();
}

void init(int32_t width,
          int32_t height,
          const char* caption,
          int32_t scale,
          const int32_t* palette,
          int32_t fps,
          int32_t border_width,
          int32_t border_color,
          int32_t quit_key) {
  std::array<int32_t, pyxelcore::COLOR_COUNT> palette_color;
  for (int32_t i = 0; i < pyxelcore::COLOR_COUNT; i++) {
    palette_color[i] = palette[i];
  }

  s_system = new pyxelcore::System(width, height, std::string(caption), scale,
                                   palette_color, fps, border_width,
                                   border_color, quit_key);
  s_resource = s_system->Resource();
  s_input = s_system->Input();
  s_graphics = s_system->Graphics();
  s_audio = s_system->Audio();
}

void run(void (*update)(), void (*draw)()) {
  GetSystem()->Run(update, draw);
}

void quit() {
  GetSystem()->Quit();
}

void flip() {
  GetSystem()->FlipScreen();
}

void show() {
  GetSystem()->ShowScreen();
}

void _drop_file_getter(char* str, int32_t str_length) {
  strncpy(str, GetSystem()->DropFile().c_str(), str_length);
}

void _caption(const char* caption) {
  GetSystem()->SetCaption(caption);
}

//
// Resource
//
inline pyxelcore::Resource* GetResource() {
  if (!s_resource) {
    PYXEL_ERROR("uninitialized function call");
  }

  return s_resource;
}

void save(const char* filename) {
  GetResource()->SaveAsset(filename);
}

void load(const char* filename,
          int32_t image,
          int32_t tilemap,
          int32_t sound,
          int32_t music) {
  GetResource()->LoadAsset(filename, image, tilemap, sound, music);
}

//
// Input
//
inline pyxelcore::Input* GetInput() {
  if (!s_input) {
    PYXEL_ERROR("uninitialized function call");
  }

  return s_input;
}

int32_t mouse_x_getter() {
  return GetInput()->MouseX();
}

int32_t mouse_y_getter() {
  return GetInput()->MouseY();
}

int32_t btn(int32_t key) {
  return GetInput()->IsButtonOn(key);
}

int32_t btnp(int32_t key, int32_t hold, int32_t period) {
  return GetInput()->IsButtonPressed(key, hold, period);
}

int32_t btnr(int32_t key) {
  return GetInput()->IsButtonReleased(key);
}

void mouse(int32_t visible) {
  return GetInput()->SetMouseVisible(visible);
}

//
// Graphics
//
inline pyxelcore::Graphics* GetGraphics() {
  if (!s_graphics) {
    PYXEL_ERROR("uninitialized function call");
  }

  return s_graphics;
}

void* image(int32_t img, int32_t system) {
  return GetGraphics()->GetImageBank(img, system);
}

void* tilemap(int32_t tm) {
  return GetGraphics()->GetTilemapBank(tm);
}

void clip0() {
  GetGraphics()->ResetClipArea();
}

void clip(int32_t x, int32_t y, int32_t w, int32_t h) {
  GetGraphics()->SetClipArea(x, y, w, h);
}

void pal0() {
  GetGraphics()->ResetPalette();
}

void pal(int32_t col1, int32_t col2) {
  GetGraphics()->SetPalette(col1, col2);
}

void cls(int32_t col) {
  GetGraphics()->ClearScreen(col);
}

void pix(int32_t x, int32_t y, int32_t col) {
  GetGraphics()->DrawPoint(x, y, col);
}

void line(int32_t x1, int32_t y1, int32_t x2, int32_t y2, int32_t col) {
  GetGraphics()->DrawLine(x1, y1, x2, y2, col);
}

void rect(int32_t x, int32_t y, int32_t w, int32_t h, int32_t col) {
  GetGraphics()->DrawRectangle(x, y, w, h, col);
}

void rectb(int32_t x, int32_t y, int32_t w, int32_t h, int32_t col) {
  GetGraphics()->DrawRectangleBorder(x, y, w, h, col);
}

void circ(int32_t x, int32_t y, int32_t r, int32_t col) {
  GetGraphics()->DrawCircle(x, y, r, col);
}

void circb(int32_t x, int32_t y, int32_t r, int32_t col) {
  GetGraphics()->DrawCircleBorder(x, y, r, col);
}

void tri(int32_t x1,
         int32_t y1,
         int32_t x2,
         int32_t y2,
         int32_t x3,
         int32_t y3,
         int32_t col) {
  GetGraphics()->DrawTriangle(x1, y1, x2, y2, x3, y3, col);
}

void trib(int32_t x1,
          int32_t y1,
          int32_t x2,
          int32_t y2,
          int32_t x3,
          int32_t y3,
          int32_t col) {
  GetGraphics()->DrawTriangleBorder(x1, y1, x2, y2, x3, y3, col);
}

void blt(int32_t x,
         int32_t y,
         int32_t img,
         int32_t u,
         int32_t v,
         int32_t w,
         int32_t h,
         int32_t colkey) {
  GetGraphics()->DrawImage(x, y, img, u, v, w, h, colkey);
}

void bltm(int32_t x,
          int32_t y,
          int32_t tm,
          int32_t u,
          int32_t v,
          int32_t w,
          int32_t h,
          int32_t colkey) {
  GetGraphics()->DrawTilemap(x, y, tm, u, v, w, h, colkey);
}

void text(int32_t x, int32_t y, const char* s, int32_t col) {
  GetGraphics()->DrawText(x, y, s, col);
}

//
// Audio
//
inline pyxelcore::Audio* GetAudio() {
  if (!s_audio) {
    PYXEL_ERROR("uninitialized function call");
  }

  return s_audio;
}

void* sound(int32_t snd, int32_t system) {
  return GetAudio()->GetSoundBank(snd, system);
}

void* music(int32_t msc) {
  return GetAudio()->GetMusicBank(msc);
}

int32_t play_pos(int32_t ch) {
  return GetAudio()->GetPlayPos(ch);
}

void play1(int32_t ch, int32_t snd, int32_t loop) {
  GetAudio()->PlaySound(ch, snd, loop);
}

void play(int32_t ch, int32_t* snd, int32_t snd_length, int32_t loop) {
  pyxelcore::SoundIndexList sound_index_list;
  for (int32_t i = 0; i < snd_length; i++) {
    sound_index_list.push_back(snd[i]);
  }

  GetAudio()->PlaySound(ch, sound_index_list, loop);
}

void playm(int32_t msc, int32_t loop) {
  GetAudio()->PlayMusic(msc, loop);
}

void stop(int32_t ch) {
  GetAudio()->StopPlaying(ch);
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

int32_t** image_data_getter(void* self) {
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
  pyxelcore::ImageString image_string;
  for (int32_t i = 0; i < data_length; i++) {
    image_string.push_back(data[i]);
  }

  IMAGE->SetData(x, y, image_string);
}

void image_load(void* self, int32_t x, int32_t y, const char* filename) {
  IMAGE->LoadImage(x, y, filename, GetSystem()->PaletteColor());
}

void image_copy(void* self,
                int32_t x,
                int32_t y,
                int32_t img,
                int32_t u,
                int32_t v,
                int32_t w,
                int32_t h) {
  IMAGE->CopyImage(x, y, GetGraphics()->GetImageBank(img, true), u, v, w, h);
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

int32_t** tilemap_data_getter(void* self) {
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
  TILEMAP->SetValue(x, y, data);
}

void tilemap_set(void* self,
                 int32_t x,
                 int32_t y,
                 const char** data,
                 int32_t data_length) {
  pyxelcore::TilemapString tilemap_string;
  for (int32_t i = 0; i < data_length; i++) {
    tilemap_string.push_back(data[i]);
  }

  TILEMAP->SetData(x, y, tilemap_string);
}

void tilemap_copy(void* self,
                  int32_t x,
                  int32_t y,
                  int32_t tm,
                  int32_t u,
                  int32_t v,
                  int32_t w,
                  int32_t h) {
  return TILEMAP->CopyTilemap(x, y, GetGraphics()->GetTilemapBank(tm), u, v, w,
                              h);
}

//
// Sound class
//
int32_t* sound_note_getter(void* self) {
  return SOUND->Note().data();
}

int32_t sound_note_length_getter(void* self) {
  return SOUND->Note().size();
}

void sound_note_length_setter(void* self, int32_t length) {
  SOUND->Note().resize(length);
}

int32_t* sound_tone_getter(void* self) {
  return SOUND->Tone().data();
}

int32_t sound_tone_length_getter(void* self) {
  return SOUND->Tone().size();
}

void sound_tone_length_setter(void* self, int32_t length) {
  SOUND->Tone().resize(length);
}

int32_t* sound_volume_getter(void* self) {
  return SOUND->Volume().data();
}

int32_t sound_volume_length_getter(void* self) {
  return SOUND->Volume().size();
}

void sound_volume_length_setter(void* self, int32_t length) {
  SOUND->Volume().resize(length);
}

int32_t* sound_effect_getter(void* self) {
  return SOUND->Effect().data();
}

int32_t sound_effect_length_getter(void* self) {
  return SOUND->Effect().size();
}

void sound_effect_length_setter(void* self, int32_t length) {
  SOUND->Effect().resize(length);
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
  return MUSIC->Channel0().data();
}

int32_t music_ch0_length_getter(void* self) {
  return MUSIC->Channel0().size();
}

void music_ch0_length_setter(void* self, int32_t length) {
  MUSIC->Channel0().resize(length);
}

int32_t* music_ch1_getter(void* self) {
  return MUSIC->Channel1().data();
}

int32_t music_ch1_length_getter(void* self) {
  return MUSIC->Channel1().size();
}

void music_ch1_length_setter(void* self, int32_t length) {
  MUSIC->Channel1().resize(length);
}

int32_t* music_ch2_getter(void* self) {
  return MUSIC->Channel2().data();
}

int32_t music_ch2_length_getter(void* self) {
  return MUSIC->Channel2().size();
}

void music_ch2_length_setter(void* self, int32_t length) {
  MUSIC->Channel2().resize(length);
}

int32_t* music_ch3_getter(void* self) {
  return MUSIC->Channel3().data();
}

int32_t music_ch3_length_getter(void* self) {
  return MUSIC->Channel3().size();
}

void music_ch3_length_setter(void* self, int32_t length) {
  MUSIC->Channel3().resize(length);
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
  pyxelcore::SoundIndexList sound_index_list0;
  for (int32_t i = 0; i < ch0_length; i++) {
    sound_index_list0.push_back(ch0[i]);
  }

  pyxelcore::SoundIndexList sound_index_list1;
  for (int32_t i = 0; i < ch1_length; i++) {
    sound_index_list1.push_back(ch1[i]);
  }

  pyxelcore::SoundIndexList sound_index_list2;
  for (int32_t i = 0; i < ch2_length; i++) {
    sound_index_list2.push_back(ch2[i]);
  }

  pyxelcore::SoundIndexList sound_index_list3;
  for (int32_t i = 0; i < ch3_length; i++) {
    sound_index_list3.push_back(ch3[i]);
  }

  MUSIC->Set(sound_index_list0, sound_index_list1, sound_index_list2,
             sound_index_list3);
}
