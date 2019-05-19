#include "pyxelcore/music.h"

namespace pyxelcore {

Music::Music() {
  for (int32_t i = 0; i < MUSIC_CHANNEL_COUNT; i++) {
    sound_length_[i] = 0;
  }
}

void Music::Set(const int32_t* channel0,
                int32_t channel0_length,
                const int32_t* channel1,
                int32_t channel1_length,
                const int32_t* channel2,
                int32_t channel2_length,
                const int32_t* channel3,
                int32_t channel3_length) {
  SetSound(0, channel0, channel0_length);
  SetSound(1, channel1, channel1_length);
  SetSound(2, channel2, channel2_length);
  SetSound(3, channel3, channel3_length);
}

void Music::SetSound(int32_t channel,
                     const int32_t* sound,
                     int32_t sound_length) {
  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PRINT_ERROR("invalid music channel");
  }

  if (sound_length < 0 || sound_length > MAX_MUSIC_LENGTH) {
    PRINT_ERROR("invalid music length");
  }

  for (int32_t i = 0; i < sound_length; i++) {
    sound_[channel][i] = sound[i];
  }
}

}  // namespace pyxelcore