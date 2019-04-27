#ifndef PYXELCORE_AUDIO_H_
#define PYXELCORE_AUDIO_H_

#include <cstdint>

namespace pyxelcore {

class Sound;
class Music;

class Audio {
 public:
  Audio();
  ~Audio();

  Sound* GetSound(int32_t sound_index, bool system = false) const;
  Music* GetMusic(int32_t music_index) const;
  void PlaySound(int32_t channel, int32_t sound_index, bool loop = false);
  void PlayMusic(int32_t music_index, bool loop = false);
  void StopPlaying(int32_t channel);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_AUDIO_H_
