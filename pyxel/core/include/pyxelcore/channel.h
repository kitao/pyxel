#ifndef PYXELCORE_CHANNEL_H_
#define PYXELCORE_CHANNEL_H_

#include "pyxelcore/oscillator.h"

namespace pyxelcore {

class Channel {
 public:
  Channel();

  int32_t PlayPos() const;

  void PlaySound(const SoundList& sound_list, bool loop);
  void StopPlaying();
  int16_t Output();

 private:
  Oscillator oscillator_;

  bool is_playing_;
  bool is_loop_;
  SoundList sound_list_;
  int32_t play_index_;

  int32_t time_;
  int32_t one_note_time_;
  int32_t total_note_time_;

  int32_t tone_;
  int32_t note_;
  float pitch_;
  int32_t volume_;
  int32_t effect_;

  int32_t effect_time_;
  float effect_pitch_;
  int32_t effect_volume_;

  void PlaySound();
  void Update();
  void NextSound();
  float NoteToPitch(float note);
  float Lfo(int32_t time);
};

inline int32_t Channel::PlayPos() const {
  return is_playing_ ? play_index_ * 100 + time_ / one_note_time_ : -1;
}

inline int16_t Channel::Output() {
  Update();

  return oscillator_.Output();
}

inline float Channel::NoteToPitch(float note) {
  return 440.0f * pow(2.0f, (note - 33.0f) / 12.0f);
}

inline float Channel::Lfo(int32_t time) {
  float x = (time * 8.0f / AUDIO_SAMPLE_RATE + 0.25f);
  x -= static_cast<int32_t>(x);

  return Abs(x * 4.0f - 2.0f) - 1.0f;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_CHANNEL_H_
