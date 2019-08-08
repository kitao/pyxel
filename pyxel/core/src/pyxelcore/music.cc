#include "pyxelcore/music.h"

namespace pyxelcore {

void Music::Set(const SoundIndexList& channel0,
                const SoundIndexList& channel1,
                const SoundIndexList& channel2,
                const SoundIndexList& channel3) {
  SetChannel0(channel0);
  SetChannel1(channel1);
  SetChannel2(channel2);
  SetChannel3(channel3);
}

void Music::SetChannel0(const SoundIndexList& channel0) {
  for (int32_t sound_index : channel0) {
    if (sound_index < 0 || sound_index >= TOTAL_SOUND_BANK_COUNT) {
      PYXEL_ERROR("invalid sound index");
    }
  }

  channel0_ = channel0;
}

void Music::SetChannel1(const SoundIndexList& channel1) {
  for (int32_t sound_index : channel1) {
    if (sound_index < 0 || sound_index >= TOTAL_SOUND_BANK_COUNT) {
      PYXEL_ERROR("invalid sound index");
    }
  }

  channel1_ = channel1;
}
void Music::SetChannel2(const SoundIndexList& channel2) {
  for (int32_t sound_index : channel2) {
    if (sound_index < 0 || sound_index >= TOTAL_SOUND_BANK_COUNT) {
      PYXEL_ERROR("invalid sound index");
    }
  }

  channel2_ = channel2;
}
void Music::SetChannel3(const SoundIndexList& channel3) {
  for (int32_t sound_index : channel3) {
    if (sound_index < 0 || sound_index >= TOTAL_SOUND_BANK_COUNT) {
      PYXEL_ERROR("invalid sound index");
    }
  }

  channel3_ = channel3;
}

}  // namespace pyxelcore