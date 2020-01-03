#ifndef PYXELCORE_MUSIC_H_
#define PYXELCORE_MUSIC_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Music {
 public:
  SoundIndexList& Channel0() { return channel0_; }
  SoundIndexList& Channel1() { return channel1_; }
  SoundIndexList& Channel2() { return channel2_; }
  SoundIndexList& Channel3() { return channel3_; }

  void Set(const SoundIndexList& channel0,
           const SoundIndexList& channel1,
           const SoundIndexList& channel2,
           const SoundIndexList& channle3);
  void SetChannel0(const SoundIndexList& channel0);
  void SetChannel1(const SoundIndexList& channel1);
  void SetChannel2(const SoundIndexList& channel2);
  void SetChannel3(const SoundIndexList& channel3);

 private:
  SoundIndexList channel0_;
  SoundIndexList channel1_;
  SoundIndexList channel2_;
  SoundIndexList channel3_;
};

}  // namespace pyxelcore

#endif  // PYXELCORE_MUSIC_H_
