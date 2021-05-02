import
  nimpy,
  ../pyxelcore

proc greet(name: string): string {.exportpy.} =
  return "Hello, " & name & "!"

proc test_sdl {.exportpy.} =
  discard
  # discard pyxel.frameCount


#[

#
# constants
#
int _get_constant_number(const char* name);
void _get_constant_string(char* str,
  int str_length,
  const char* name);

#
# system
#
int width_getter();
int height_getter();
int frame_count_getter();

void init(int width,
  int height,
  const char* caption,
  int scale,
  const int* palette,
  int fps,
  int quit_key,
  int fullscreen);
void run(void (*update)(), void (*draw)());
int quit();
int flip();
void show();

void _drop_file_getter(char* str, int str_length);
void _caption(const char* caption);

#
# resource
#
void save(const char* filename);
void load(const char* filename,
  int image,
  int tilemap,
  int sound,
  int music);

#
# input
#
int mouse_x_getter();
int mouse_y_getter();
int mouse_wheel_getter();

int btn(int key);
int btnp(int key, int hold, int period);
int btnr(int key);
void mouse(int visible);

#
# Graphics
#
void* image(int img, int system);
void* tilemap(int tm);
void clip0();
void clip(int x, int y, int w, int h);
void pal0();
void pal(int col1, int col2);
void cls(int col);
int pget(int x, int y);
void pset(int x, int y, int col);
void line(int x1, int y1, int x2, int y2, int col);
void rect(int x, int y, int w, int h, int col);
void rectb(int x, int y, int w, int h, int col);
void circ(int x, int y, int r, int col);
void circb(int x, int y, int r, int col);
void tri(int x1, int y1, int x2, int y2, int x3, int y3, int col);
void trib(int x1, int y1, int x2, int y2, int x3, int y3, int col);
void blt(int x, int y, int img, int u, int v, int w, int h, int colkey);
void bltm(int x, int y, int tm, int u, int v, int w, int h, int colkey);
void text(int x, int y, const char* s, int col);

#
# Audio
#
void* sound(int snd, int system);
void* music(int msc);
int play_pos(int ch);
void play1(int ch, int snd, int loop);
void play(int ch, int* snd, int snd_length, int loop);
void playm(int msc, int loop);
void stop(int ch);

#
# Image class
#
int image_width_getter(void* self);
int image_height_getter(void* self);
int** image_data_getter(void* self);

int image_get(void* self, int x, int y);
void image_set1(void* self, int x, int y, int data);
void image_set(void* self,
  int x,
  int y,
  const char** data,
  int data_length);
void image_load(void* self, int x, int y, const char* filename);
void image_copy(void* self, int x, int y, int img, int u, int v, int w, int h);

#
# Tilemap class
#
int tilemap_width_getter(void* self);
int tilemap_height_getter(void* self);
int** tilemap_data_getter(void* self);
int tilemap_refimg_getter(void* self);
void tilemap_refimg_setter(void* self, int refimg);

int tilemap_get(void* self, int x, int y);
void tilemap_set1(void* self, int x, int y, int data);
void tilemap_set(void* self,
  int x,
  int y,
  const char** data,
  int data_length);
void tilemap_copy(void* self, int x, int y, int tm, int u, int v, int w, int h);

#
# Sound class
#
int* sound_note_getter(void* self);
int sound_note_length_getter(void* self);
void sound_note_length_setter(void* self, int length);
int* sound_tone_getter(void* self);
int sound_tone_length_getter(void* self);
void sound_tone_length_setter(void* self, int length);
int* sound_volume_getter(void* self);
int sound_volume_length_getter(void* self);
void sound_volume_length_setter(void* self, int length);
int* sound_effect_getter(void* self);
int sound_effect_length_getter(void* self);
void sound_effect_length_setter(void* self, int length);
int sound_speed_getter(void* self);
void sound_speed_setter(void* self, int speed);

void sound_set(void* self,
  const char* note,
  const char* tone,
  const char* volume,
  const char* effect,
  int speed);
void sound_set_note(void* self, const char* note);
void sound_set_tone(void* self, const char* tone);
void sound_set_volume(void* self, const char* volume);
void sound_set_effect(void* self, const char* effect);

#
# Music class
#
int* music_ch0_getter(void* self);
int music_ch0_length_getter(void* self);
void music_ch0_length_setter(void* self, int length);
int* music_ch1_getter(void* self);
int music_ch1_length_getter(void* self);
void music_ch1_length_setter(void* self, int length);
int* music_ch2_getter(void* self);
int music_ch2_length_getter(void* self);
void music_ch2_length_setter(void* self, int length);
int* music_ch3_getter(void* self);
int music_ch3_length_getter(void* self);
void music_ch3_length_setter(void* self, int length);

void music_set(void* self,
  const int* ch0,
  int ch0_length,
  const int* ch1,
  int ch1_length,
  const int* ch2,
  int ch2_length,
  const int* ch3,
  int ch3_length);

]#
