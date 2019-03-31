#include "pyxelcore/audio.h"

#include <cstddef>

namespace pyxelcore {

Audio::Audio() {}

Audio::~Audio() {}

Sound* Audio::Sound(int32_t snd, bool system) {
  return NULL;
}

Music* Audio::Music(int32_t msc) {
  return NULL;
}

void Audio::Play(int32_t ch, int32_t snd, bool loop) {}

void Audio::Playm(int32_t msc, bool loop) {}

void Audio::Stop(int32_t ch) {}

}  // namespace pyxelcore
