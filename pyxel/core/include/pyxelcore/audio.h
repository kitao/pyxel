#ifndef PYXELCORE_AUDIO_H_
#define PYXELCORE_AUDIO_H_

namespace pyxelcore {

class Audio {
public:
  Audio();
  ~Audio();

  void *sound(int snd, int system);
  void *music(int msc);
  void play(int ch, int snd, int loop);
  void playm(int msc, int loop);
  void stop(int ch);

private:
};

} // namespace pyxelcore

#endif // PYXELCORE_AUDIO_H_
