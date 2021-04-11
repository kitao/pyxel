#[
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
]#
