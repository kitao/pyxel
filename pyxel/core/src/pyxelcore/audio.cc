#include "pyxelcore/app.h"

#include <cstddef>

namespace pyxelcore {

Sound* App::Sound(int32_t snd, bool system) {
  return NULL;
}

Music* App::Music(int32_t msc) {
  return NULL;
}

void App::PlaySound(int32_t ch, int32_t snd, bool loop) {}

void App::PlayMusic(int32_t msc, bool loop) {}

void App::StopPlaying(int32_t ch) {}

}  // namespace pyxelcore
