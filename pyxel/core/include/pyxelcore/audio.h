#ifndef PYXELCORE_AUDIO_H_
#define PYXELCORE_AUDIO_H_

namespace pyxelcore {

class Audio {
public:
  Audio();
  ~Audio();

  void *Sound(int snd, int system);
  void *Music(int msc);
  void Play(int ch, int snd, int loop);
  void Playm(int msc, int loop);
  void Stop(int ch);

private:
};

} // namespace pyxelcore

#endif // PYXELCORE_AUDIO_H_
