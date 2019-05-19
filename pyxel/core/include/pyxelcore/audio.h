#ifndef PYXELCORE_AUDIO_H_
#define PYXELCORE_AUDIO_H_

#include "pyxelcore/channel.h"

namespace pyxelcore {

class Sound;
class Music;

class Audio {
 public:
  Audio();
  ~Audio();

  Sound* GetSoundBank(int32_t sound_index, bool system = false) const;
  Music* GetMusicBank(int32_t music_index) const;
  void PlaySound(int32_t channel, int32_t sound_index, bool loop = false);
  void PlaySound(int32_t channel,
                 int32_t* sound_index,
                 int32_t sound_count,
                 bool loop = false);
  void PlayMusic(int32_t music_index, bool loop = false);
  void StopPlaying(int32_t channel = -1);

 private:
  Sound** sound_bank_;
  Music** music_bank_;
  Channel channel_list_[MUSIC_CHANNEL_COUNT];

  static void callback(void* userdata, uint8_t* stream, int len);
};

inline Sound* Audio::GetSoundBank(int32_t sound_index, bool system) const {
  if (sound_index < 0 || sound_index >= SOUND_BANK_COUNT) {
    PRINT_ERROR("invalid sound index");
    sound_index = 0;
  }

  if (sound_index == SOUND_BANK_FOR_SYSTEM && !system) {
    PRINT_ERROR("access to sound bank for system");
  }

  return sound_bank_[sound_index];
}

inline Music* Audio::GetMusicBank(int32_t music_index) const {
  if (music_index < 0 || music_index >= MUSIC_BANK_COUNT) {
    PRINT_ERROR("invalid music index");
    music_index = 0;
  }

  return music_bank_[music_index];
}

}  // namespace pyxelcore

#endif  // PYXELCORE_AUDIO_H_
