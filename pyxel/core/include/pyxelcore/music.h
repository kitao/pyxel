#ifndef PYXELCORE_MUSIC_H_
#define PYXELCORE_MUSIC_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Music {
 public:
  Music();

  int32_t* Sound(int32_t channel);
  int32_t SoundLength(int32_t channel) const;
  void SoundLength(int32_t channel, int32_t length);

  void Set(const int32_t* channel0,
           int32_t channel0_length,
           const int32_t* channel1,
           int32_t channel1_length,
           const int32_t* channel2,
           int32_t channel2_length,
           const int32_t* channel3,
           int32_t channel3_length);
  void SetSound(int32_t channel, const int32_t* sound, int32_t sound_length);

 private:
  int32_t sound_[MUSIC_CHANNEL_COUNT][MAX_MUSIC_LENGTH];
  int32_t sound_length_[MUSIC_CHANNEL_COUNT];
};

inline int32_t* Music::Sound(int32_t channel) {
  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PRINT_ERROR("invalid music channel");
  }

  return sound_[channel];
}

inline int32_t Music::SoundLength(int32_t channel) const {
  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PRINT_ERROR("invalid music channel");
  }

  return sound_length_[channel];
}

inline void Music::SoundLength(int32_t channel, int32_t length) {
  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PRINT_ERROR("invalid music channel");
  }

  if (length < 0 || length > MAX_MUSIC_LENGTH) {
    PRINT_ERROR("invalid music length");
  }

  sound_length_[channel] = length;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_MUSIC_H_
