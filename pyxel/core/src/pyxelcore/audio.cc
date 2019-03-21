#include <cstddef>

#include "pyxelcore/audio.h"

namespace pyxelcore {

Audio::Audio() {}

Audio::~Audio() {}

void* Audio::Sound(int32_t snd, int32_t system) {
  return NULL;
}

void* Audio::Music(int32_t msc) {
  return NULL;
}

void Audio::Play(int32_t ch, int32_t snd, int32_t loop) {}

void Audio::Playm(int32_t msc, int32_t loop) {}

void Audio::Stop(int32_t ch) {}

}  // namespace pyxelcore
