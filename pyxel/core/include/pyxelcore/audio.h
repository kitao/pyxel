#ifndef PYXELCORE_AUDIO_H_
#define PYXELCORE_AUDIO_H_

#include <cstdint>

namespace pyxelcore {

class Audio {
 public:
  Audio();
  ~Audio();

  void* Sound(int32_t snd, int32_t system);
  void* Music(int32_t msc);
  void Play(int32_t ch, int32_t snd, int32_t loop);
  void Playm(int32_t msc, int32_t loop);
  void Stop(int32_t ch);

 private:
};

}  // namespace pyxelcore

#endif  // PYXELCORE_AUDIO_H_
