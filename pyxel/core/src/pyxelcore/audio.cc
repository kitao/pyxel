#include "pyxelcore/app.h"

#include <cstddef>

namespace pyxelcore {

Sound* App::GetSound(int32_t sound_index, bool system) {
  return NULL;
}

Music* App::GetMusic(int32_t music_index) {
  return NULL;
}

void App::PlaySound(int32_t ch, Sound* sound, bool loop) {
  //
}

void App::PlayMusic(Music* music, bool loop) {
  //
}

void App::StopPlaying(int32_t ch) {
  //
}

}  // namespace pyxelcore
