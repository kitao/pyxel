#include <cstddef>

#include "pyxelcore/audio.h"

namespace pyxelcore {

Audio::Audio() {}

Audio::~Audio() {}

void* Audio::sound(int snd, int system) {
  return NULL;
}

void* Audio::music(int msc) {
  return NULL;
}

void Audio::play(int ch, int snd, int loop) {}

void Audio::playm(int msc, int loop) {}

void Audio::stop(int ch) {}

}  // namespace pyxelcore
