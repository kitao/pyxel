#ifndef PYXELCORE_AUDIO_H_
#define PYXELCORE_AUDIO_H_

#include <cstdint>

namespace pyxelcore {

class Audio {
 public:
  Audio();
  ~Audio();

  void* sound(int32_t snd, int32_t system);
  void* music(int32_t msc);
  void play(int32_t ch, int32_t snd, int32_t loop);
  void playm(int32_t msc, int32_t loop);
  void stop(int32_t ch);

 private:
};

}  // namespace pyxelcore

#endif  // PYXELCORE_AUDIO_H_
