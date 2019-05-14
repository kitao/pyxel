#ifndef PYXELCORE_CHANNEL_H_
#define PYXELCORE_CHANNEL_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Channel {
 public:
  Channel();
  ~Channel();

  void Play(int32_t* sound, int32_t sound_count, bool loop);
  void Stop();
  int32_t Output();

 private:
  void PlaySound();
  void Update();
  void NextSound();
  int32_t NoteToPitch();
  int32_t Lfo(int32_t time);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_CHANNEL_H_
