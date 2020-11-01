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
  int32_t GetPlayPos(int32_t channel) const;
  void PlaySound(int32_t channel, int32_t sound_index, bool loop = false);
  void PlaySound(int32_t channel,
                 const SoundIndexList& sound_index_list,
                 bool loop = false);
  void PlayMusic(int32_t music_index, bool loop = false);
  void StopPlaying(int32_t channel = -1);

 private:
  SDL_mutex* audio_mutex_;
  Sound** sound_bank_;
  Music** music_bank_;
  Channel channel_[MUSIC_CHANNEL_COUNT];

  void LockAudio();
  void UnlockAudio();

  static void callback(void* userdata, uint8_t* stream, int len);
};

inline Sound* Audio::GetSoundBank(int32_t sound_index, bool system) const {
  if (sound_index < 0 || sound_index >= TOTAL_SOUND_BANK_COUNT) {
    PYXEL_ERROR("invalid sound index");
  }

  if (sound_index >= USER_SOUND_BANK_COUNT && !system) {
    PYXEL_ERROR("access to sound bank for system");
  }

  return sound_bank_[sound_index];
}

inline Music* Audio::GetMusicBank(int32_t music_index) const {
  if (music_index < 0 || music_index >= MUSIC_BANK_COUNT) {
    PYXEL_ERROR("invalid music index");
  }

  return music_bank_[music_index];
}

inline int32_t Audio::GetPlayPos(int32_t channel) const {
  if (channel < 0 || channel >= MUSIC_CHANNEL_COUNT) {
    PYXEL_ERROR("invalid channel");
  }

  return channel_[channel].PlayPos();
}

inline void Audio::LockAudio() {
  SDL_LockMutex(audio_mutex_);
}

inline void Audio::UnlockAudio() {
  SDL_UnlockMutex(audio_mutex_);
}

}  // namespace pyxelcore

#endif  // PYXELCORE_AUDIO_H_
