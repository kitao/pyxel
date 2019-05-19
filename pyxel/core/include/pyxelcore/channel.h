#ifndef PYXELCORE_CHANNEL_H_
#define PYXELCORE_CHANNEL_H_

#include "pyxelcore/oscillator.h"

namespace pyxelcore {

class Sound;

class Channel {
 public:
  Channel();

  void PlaySound(Sound** sound, int32_t sound_count, bool loop);
  void StopPlaying();
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
  void NextNote();
  void NextSound();
  float NoteToPitch(float note);
  float Lfo(int32_t time);
};

inline int32_t Channel::Output() {
  NextNote();

  return oscillator_.Output();
}

inline float Channel::NoteToPitch(float note) {
  return 440.0f * pow(2.0f, (note - 33.0f) / 12.0f);
}

inline float Channel::Lfo(int32_t time) {
  float x = (time * 8 / AUDIO_SAMPLE_RATE + 0.25f);
  x -= static_cast<int32_t>(x);

  return Abs(x * 4.0f - 2.0f) - 1.0f;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_CHANNEL_H_
