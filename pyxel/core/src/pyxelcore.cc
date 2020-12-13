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
int _get_constant_number(const char* name) {
  return pyxelcore::GetConstantNumber(name);
}

void _get_constant_string(char* str, int str_length, const char* name) {
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

int width_getter() {
  return GetSystem()->Width();
}

int height_getter() {
  return GetSystem()->Height();
}

int frame_count_getter() {
  return GetSystem()->FrameCount();
}

void init(int width,
          int height,
          const char* caption,
          int scale,
          const int* palette,
          int fps,
          int quit_key,
          int fullscreen) {
  std::array<int, pyxelcore::COLOR_COUNT> palette_color;
  for (int i = 0; i < pyxelcore::COLOR_COUNT; i++) {
    palette_color[i] = palette[i];
  }

  s_system = new pyxelcore::System(width, height, std::string(caption), scale,
                                   palette_color, fps, quit_key, fullscreen);
  s_resource = s_system->Resource();
  s_input = s_system->Input();
  s_graphics = s_system->Graphics();
  s_audio = s_system->Audio();
}

void run(void (*update)(), void (*draw)()) {
  GetSystem()->Run(update, draw);
}

int quit() {
  return GetSystem()->Quit();
}

int flip() {
  return GetSystem()->FlipScreen();
}

void show() {
  GetSystem()->ShowScreen();
}

void _drop_file_getter(char* str, int str_length) {
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

void load(const char* filename, int image, int tilemap, int sound, int music) {
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

int mouse_x_getter() {
  return GetInput()->MouseX();
}

int mouse_y_getter() {
  return GetInput()->MouseY();
}

int mouse_wheel_getter() {
  return GetInput()->MouseWheel();
}

int btn(int key) {
  return GetInput()->IsButtonOn(key);
}

int btns(int *keys, int len) {
  return GetInput()->GetButtonsOn(keys, len);
}

int btnp(int key, int hold, int period) {
  return GetInput()->IsButtonPressed(key, hold, period);
}

int btnsp(int *keys, int len, int hold, int period) {
  return GetInput()->GetButtonsPressed(keys, hold, period);
}

int btnr(int key) {
  return GetInput()->IsButtonReleased(key);
}

void mouse(int visible) {
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

void* image(int img, int system) {
  return GetGraphics()->GetImageBank(img, system);
}

void* tilemap(int tm) {
  return GetGraphics()->GetTilemapBank(tm);
}

void clip0() {
  GetGraphics()->ResetClipArea();
}

void clip(int x, int y, int w, int h) {
  GetGraphics()->SetClipArea(x, y, w, h);
}

void pal0() {
  GetGraphics()->ResetPalette();
}

void pal(int col1, int col2) {
  GetGraphics()->SetPalette(col1, col2);
}

void cls(int col) {
  GetGraphics()->ClearScreen(col);
}

int pget(int x, int y) {
  return GetGraphics()->GetPoint(x, y);
}

void pset(int x, int y, int col) {
  GetGraphics()->SetPoint(x, y, col);
}

void line(int x1, int y1, int x2, int y2, int col) {
  GetGraphics()->DrawLine(x1, y1, x2, y2, col);
}

void rect(int x, int y, int w, int h, int col) {
  GetGraphics()->DrawRectangle(x, y, w, h, col);
}

void rectb(int x, int y, int w, int h, int col) {
  GetGraphics()->DrawRectangleBorder(x, y, w, h, col);
}

void circ(int x, int y, int r, int col) {
  GetGraphics()->DrawCircle(x, y, r, col);
}

void circb(int x, int y, int r, int col) {
  GetGraphics()->DrawCircleBorder(x, y, r, col);
}

void tri(int x1, int y1, int x2, int y2, int x3, int y3, int col) {
  GetGraphics()->DrawTriangle(x1, y1, x2, y2, x3, y3, col);
}

void trib(int x1, int y1, int x2, int y2, int x3, int y3, int col) {
  GetGraphics()->DrawTriangleBorder(x1, y1, x2, y2, x3, y3, col);
}

void blt(int x, int y, int img, int u, int v, int w, int h, int colkey) {
  GetGraphics()->DrawImage(x, y, img, u, v, w, h, colkey);
}

void bltm(int x, int y, int tm, int u, int v, int w, int h, int colkey) {
  GetGraphics()->DrawTilemap(x, y, tm, u, v, w, h, colkey);
}

void text(int x, int y, const char* s, int col) {
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

void* sound(int snd, int system) {
  return GetAudio()->GetSoundBank(snd, system);
}

void* music(int msc) {
  return GetAudio()->GetMusicBank(msc);
}

int play_pos(int ch) {
  return GetAudio()->GetPlayPos(ch);
}

void play1(int ch, int snd, int loop) {
  GetAudio()->PlaySound(ch, snd, loop);
}

void play(int ch, int* snd, int snd_length, int loop) {
  pyxelcore::SoundIndexList sound_index_list;
  for (int i = 0; i < snd_length; i++) {
    sound_index_list.push_back(snd[i]);
  }

  GetAudio()->PlaySound(ch, sound_index_list, loop);
}

void playm(int msc, int loop) {
  GetAudio()->PlayMusic(msc, loop);
}

void stop(int ch) {
  GetAudio()->StopPlaying(ch);
}

//
// Image class
//
int image_width_getter(void* self) {
  return IMAGE->Width();
}

int image_height_getter(void* self) {
  return IMAGE->Height();
}

int** image_data_getter(void* self) {
  return IMAGE->Data();
}

int image_get(void* self, int x, int y) {
  return IMAGE->GetValue(x, y);
}

void image_set1(void* self, int x, int y, int data) {
  IMAGE->SetValue(x, y, data);
}

void image_set(void* self, int x, int y, const char** data, int data_length) {
  pyxelcore::ImageString image_string;
  for (int i = 0; i < data_length; i++) {
    image_string.push_back(data[i]);
  }

  IMAGE->SetData(x, y, image_string);
}

void image_load(void* self, int x, int y, const char* filename) {
  IMAGE->LoadImage(x, y, filename, GetSystem()->PaletteColor());
}

void image_copy(void* self, int x, int y, int img, int u, int v, int w, int h) {
  IMAGE->CopyImage(x, y, GetGraphics()->GetImageBank(img, true), u, v, w, h);
}

//
// Tilemap class
//
int tilemap_width_getter(void* self) {
  return TILEMAP->Width();
}

int tilemap_height_getter(void* self) {
  return TILEMAP->Height();
}

int** tilemap_data_getter(void* self) {
  return TILEMAP->Data();
}

int tilemap_refimg_getter(void* self) {
  return TILEMAP->ImageIndex();
}

void tilemap_refimg_setter(void* self, int refimg) {
  TILEMAP->ImageIndex(refimg);
}

int tilemap_get(void* self, int x, int y) {
  return TILEMAP->GetValue(x, y);
}

void tilemap_set1(void* self, int x, int y, int data) {
  TILEMAP->SetValue(x, y, data);
}

void tilemap_set(void* self, int x, int y, const char** data, int data_length) {
  pyxelcore::TilemapString tilemap_string;
  for (int i = 0; i < data_length; i++) {
    tilemap_string.push_back(data[i]);
  }

  TILEMAP->SetData(x, y, tilemap_string);
}

void tilemap_copy(void* self,
                  int x,
                  int y,
                  int tm,
                  int u,
                  int v,
                  int w,
                  int h) {
  return TILEMAP->CopyTilemap(x, y, GetGraphics()->GetTilemapBank(tm), u, v, w,
                              h);
}

//
// Sound class
//
int* sound_note_getter(void* self) {
  return SOUND->Note().data();
}

int sound_note_length_getter(void* self) {
  return SOUND->Note().size();
}

void sound_note_length_setter(void* self, int length) {
  SOUND->Note().resize(length);
}

int* sound_tone_getter(void* self) {
  return SOUND->Tone().data();
}

int sound_tone_length_getter(void* self) {
  return SOUND->Tone().size();
}

void sound_tone_length_setter(void* self, int length) {
  SOUND->Tone().resize(length);
}

int* sound_volume_getter(void* self) {
  return SOUND->Volume().data();
}

int sound_volume_length_getter(void* self) {
  return SOUND->Volume().size();
}

void sound_volume_length_setter(void* self, int length) {
  SOUND->Volume().resize(length);
}

int* sound_effect_getter(void* self) {
  return SOUND->Effect().data();
}

int sound_effect_length_getter(void* self) {
  return SOUND->Effect().size();
}

void sound_effect_length_setter(void* self, int length) {
  SOUND->Effect().resize(length);
}

int sound_speed_getter(void* self) {
  return SOUND->Speed();
}

void sound_speed_setter(void* self, int speed) {
  SOUND->Speed(speed);
}

void sound_set(void* self,
               const char* note,
               const char* tone,
               const char* volume,
               const char* effect,
               int speed) {
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
int* music_ch0_getter(void* self) {
  return MUSIC->Channel0().data();
}

int music_ch0_length_getter(void* self) {
  return MUSIC->Channel0().size();
}

void music_ch0_length_setter(void* self, int length) {
  MUSIC->Channel0().resize(length);
}

int* music_ch1_getter(void* self) {
  return MUSIC->Channel1().data();
}

int music_ch1_length_getter(void* self) {
  return MUSIC->Channel1().size();
}

void music_ch1_length_setter(void* self, int length) {
  MUSIC->Channel1().resize(length);
}

int* music_ch2_getter(void* self) {
  return MUSIC->Channel2().data();
}

int music_ch2_length_getter(void* self) {
  return MUSIC->Channel2().size();
}

void music_ch2_length_setter(void* self, int length) {
  MUSIC->Channel2().resize(length);
}

int* music_ch3_getter(void* self) {
  return MUSIC->Channel3().data();
}

int music_ch3_length_getter(void* self) {
  return MUSIC->Channel3().size();
}

void music_ch3_length_setter(void* self, int length) {
  MUSIC->Channel3().resize(length);
}

void music_set(void* self,
               const int* ch0,
               int ch0_length,
               const int* ch1,
               int ch1_length,
               const int* ch2,
               int ch2_length,
               const int* ch3,
               int ch3_length) {
  pyxelcore::SoundIndexList sound_index_list0;
  for (int i = 0; i < ch0_length; i++) {
    sound_index_list0.push_back(ch0[i]);
  }

  pyxelcore::SoundIndexList sound_index_list1;
  for (int i = 0; i < ch1_length; i++) {
    sound_index_list1.push_back(ch1[i]);
  }

  pyxelcore::SoundIndexList sound_index_list2;
  for (int i = 0; i < ch2_length; i++) {
    sound_index_list2.push_back(ch2[i]);
  }

  pyxelcore::SoundIndexList sound_index_list3;
  for (int i = 0; i < ch3_length; i++) {
    sound_index_list3.push_back(ch3[i]);
  }

  MUSIC->Set(sound_index_list0, sound_index_list1, sound_index_list2,
             sound_index_list3);
}
