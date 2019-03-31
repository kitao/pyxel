#ifndef PYXELCORE_AUDIO_H_
#define PYXELCORE_AUDIO_H_

#include <cstdint>

namespace pyxelcore {

class Sound;
class Music;

class Audio {
 public:
  Audio();
  ~Audio();

  Sound* Sound(int32_t snd, bool system = false);
  Music* Music(int32_t msc);
  void Play(int32_t ch, int32_t snd, bool loop = false);
  void Playm(int32_t msc, bool loop = false);
  void Stop(int32_t ch);

 private:
};

}  // namespace pyxelcore

#endif  // PYXELCORE_AUDIO_H_
