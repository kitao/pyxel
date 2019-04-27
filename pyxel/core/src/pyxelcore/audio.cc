#include "pyxelcore/audio.h"

#include <cstddef>

namespace pyxelcore {

Audio::Audio() {}

Audio::~Audio() {}

Sound* Audio::GetSound(int32_t sound_index, bool system) const {
  return NULL;
}

Music* Audio::GetMusic(int32_t music_index) const {
  return NULL;
}

void Audio::PlaySound(int32_t channel, int32_t sound_index, bool loop) {
  //
}

void Audio::PlayMusic(int32_t music_index, bool loop) {
  //
}

void Audio::StopPlaying(int32_t channel) {
  //
}

}  // namespace pyxelcore
