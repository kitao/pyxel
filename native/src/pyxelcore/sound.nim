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
