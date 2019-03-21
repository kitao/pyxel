#include <cstddef>

#include "pyxelcore/audio.h"

namespace pyxelcore {

Audio::Audio() {}

Audio::~Audio() {}

void* Audio::sound(int32_t snd, int32_t system) {
  return NULL;
}

void* Audio::music(int32_t msc) {
  return NULL;
}

void Audio::play(int32_t ch, int32_t snd, int32_t loop) {}

void Audio::playm(int32_t msc, int32_t loop) {}

void Audio::stop(int32_t ch) {}

}  // namespace pyxelcore
