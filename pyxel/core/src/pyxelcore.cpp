#include "pyxelcore.h"
#include "pyxelcore/app.h"
#include <cstdio>

static pyxelcore::App *app = NULL;

//
// System
//
int Width_Getter() { return app ? app->Width_Getter() : 0; }
int Height_Getter() { return app ? app->Height_Getter() : 0; }
int FrameCount_Getter() { return app ? app->FrameCount_Getter() : 0; }

void Init(int width, int height, char *caption, int scale, int *palette,
          int fps, int border_width, int border_color) {
  app = new pyxelcore::App(width, height, caption, scale, palette, fps,
                           border_width, border_color);
}

void Run(void (*update)(), void (*draw)()) {
  if (app) {
    app->Run(update, draw);
  }
}

void Quit() {}

//
// Resource
//
void Save(char *filename) {}
void Load(char *filename) {}

//
// Input
//
int MouseX_Getter() { return 0; }
int MouseY_Getter() { return 0; }

int Btn(int key) { return 0; }
int Btnp(int key, int hold, int period) { return 0; }
int Btnr(int key) { return 0; }
void Mouse(int visible) {}

//
// Graphics
//
void *Image(int img, int system) { return NULL; }
void *Tilemap(int tm) { return NULL; }
void Clip(int x1, int y1, int x2, int y2) {}

//
// Audio
//
void *Sound(int snd, int system) { return NULL; }
void *Music(int msc) { return NULL; }
void Play(int ch, int snd, int loop) {}
void Playm(int msc, int loop) {}
void Stop(int ch) {}

//
// Image class
//
int Image_Width_Getter(void *self) { return 0; }
int Image_Height_Getter(void *self) { return 0; }
int *Image_Data_Getter(void *self) { return NULL; }

int Image_Get(void *self, int x, int y) { return 0; }
void Image_Set1(void *self, int x, int y, int data) {}
void Image_Set(void *self, int x, int y, int *data, int data_width,
               int data_height) {}
void Image_Load(void *self, int x, int y, char *filename) {}
void Image_Copy(void *self, int x, int y, int img, int u, int v, int w, int h) {
}

//
// Tilemap class
//
int Tilemap_Width_Getter(void *self) { return 0; }
int Tilemap_Height_Getter(void *self) { return 0; }
int *Tilemap_Data_Getter(void *self) { return NULL; }

int Tilemap_Get(int x, int y) { return 0; }
void Timemap_Set1(int x, int y, int data, int refimg) {}
void Timemap_Set(int x, int y, int *data, int data_width, int data_height,
                 int refimg) {}
void Timemap_Copy(int x, int y, int tm, int u, int v, int w, int h) {}

//
// Sound class
//
int *Sound_Note_Getter(void *self, int *length) { return NULL; }
void Sound_Note_Setter(void *self, int length) {}
int *Sound_Tone_Getter(void *self, int *length) { return NULL; }
void Sound_Tone_Setter(void *self, int length) {}
int *Sound_Volume_Getter(void *self, int *length) { return NULL; }
void Sound_Volume_Setter(void *self, int length) {}
int *Sound_Effect_Getter(void *self, int *length) { return NULL; }
void Sound_Effect_Setter(void *self, int length) {}
int Sound_Speed_Getter(void *self) { return 0; }
void Sound_Speed_Setter(void *self, int speed) {}

void Sound_Set(void *self, char *note, char *tone, char *volume, char *effect,
               int speed) {}
void Sound_SetNote(void *self, char *data) {}
void Sound_SetTone(void *self, char *data) {}
void Sound_SetVolume(void *self, char *data) {}
void Sound_SetEffect(void *self, char *data) {}

//
// Music class
//
int *Music_Ch0_Getter(void *self, int *length) { return NULL; }
void Music_Ch0_Setter(void *self, int length) {}
int *Music_Ch1_Getter(void *self, int *length) { return NULL; }
void Music_Ch1_Setter(void *self, int length) {}
int *Music_Ch2_Getter(void *self, int *length) { return NULL; }
void Music_Ch2_Setter(void *self, int length) {}
int *Music_Ch3_Getter(void *self, int *length) { return NULL; }
void Music_Ch3_Setter(void *self, int length) {}

void Music_Set(void *self, int *ch0, int ch0_length, int *ch1, int ch1_length,
               int *ch2, int ch2_length, int *ch3, int ch3_length) {}
void Music_SetCh0(void *self, int *data, int data_length) {}
void Music_SetCh1(void *self, int *data, int data_length) {}
void Music_SetCh2(void *self, int *data, int data_length) {}
void Music_SetCh3(void *self, int *data, int data_length) {}
