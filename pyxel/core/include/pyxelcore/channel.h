#ifndef PYXELCORE_CHANNEL_H_
#define PYXELCORE_CHANNEL_H_

#include "pyxelcore/oscillator.h"

namespace pyxelcore {

class Sound;

class Channel {
 public:
  Channel();

  void Play(Sound** sound, int32_t sound_count, bool loop);
  void Stop();
  int32_t Output();

 private:
  Oscillator oscillator_;

  bool is_playing_;
  bool is_loop_;
  Sound* sound_[MAX_MUSIC_LENGTH];
  int32_t sound_count_;
  int32_t sound_index_;

  int32_t time_;
  int32_t one_note_time_;
  int32_t total_note_time_;

  int32_t tone_;
  int32_t note_;
  int32_t pitch_;
  int32_t volume_;
  int32_t effect_;

  int32_t effect_time_;
  int32_t effect_pitch_;
  int32_t effect_volume_;

  void PlaySound();
  void Update();
  void NextSound();
  int32_t NoteToPitch(int32_t note);
  int32_t Lfo(int32_t time);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_CHANNEL_H_
