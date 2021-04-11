#[
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

void music_set(void* self, const int* ch0, int ch0_length, const int* ch1, int ch1_length, const int* ch2, int ch2_length, const int* ch3, int ch3_length);
]#
