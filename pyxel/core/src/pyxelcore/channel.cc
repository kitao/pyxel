#include "pyxelcore/channel.h"

#include "pyxelcore/sound.h"

namespace pyxelcore {

Channel::Channel() {
  is_playing_ = false;
  is_loop_ = false;
  play_index_ = 0;

  time_ = 0;
  one_note_time_ = 0;

  tone_ = 0;
  note_ = 0;
  pitch_ = 0.0f;
  volume_ = 0;
  effect_ = 0;

  effect_time_ = 0;
  effect_pitch_ = 0.0f;
  effect_volume_ = 0;
}

void Channel::PlaySound(const SoundList& sound_list, bool loop) {
  if (sound_list.empty()) {
    return;
  }

  is_playing_ = true;
  is_loop_ = loop;
  sound_list_ = sound_list;
  play_index_ = 0;

  PlaySound();
}

void Channel::StopPlaying() {
  is_playing_ = false;
  pitch_ = 0.0f;
  oscillator_.Stop();
}

void Channel::PlaySound() {
  Sound* sound = sound_list_[play_index_];

  time_ = 0;
  one_note_time_ = sound->Speed() * AUDIO_ONE_SPEED;
  total_note_time_ = one_note_time_ * sound->Note().size();
}

void Channel::Update() {
  if (!is_playing_) {
    return;
  }

  if (total_note_time_ == 0) {
    NextSound();
    return;
  }

  // forward note
  if (time_ % one_note_time_ == 0) {
    Sound* sound = sound_list_[play_index_];
    int32_t pos = time_ / one_note_time_;
    note_ = sound->Note()[pos];
    volume_ = (sound->Volume().empty()
                   ? 7
                   : sound->Volume()[pos % sound->Volume().size()]) *
              AUDIO_ONE_VOLUME;

    if (note_ >= 0 && volume_ > 0) {
      float last_pitch = pitch_;
      tone_ = sound->Tone().empty() ? TONE_TRIANGLE
                                    : sound->Tone()[pos % sound->Tone().size()];
      pitch_ = NoteToPitch(note_);
      effect_ = sound->Effect().empty()
                    ? EFFECT_NONE
                    : sound->Effect()[pos % sound->Effect().size()];

      oscillator_.SetTone(tone_);
      oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch_);
      oscillator_.SetVolume(volume_);

      switch (effect_) {
        case EFFECT_SLIDE:
          effect_time_ = time_;
          effect_pitch_ = last_pitch > 0.0f ? last_pitch : pitch_;
          break;

        case EFFECT_VIBRATO:
          effect_time_ = time_;
          effect_pitch_ = NoteToPitch(note_ + 0.5f) - pitch_;
          break;

        case EFFECT_FADEOUT:
          effect_time_ = time_;
          effect_volume_ = volume_;
          break;
      }
    } else {
      oscillator_.Stop();
    }
  }

  // play note
  if (note_ >= 0 && volume_ > 0) {
    float a;
    int32_t pitch;

    switch (effect_) {
      case EFFECT_SLIDE:
        a = static_cast<float>(time_ - effect_time_) / one_note_time_;
        pitch = pitch_ * a + effect_pitch_ * (1.0f - a);
        oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch);
        break;

      case EFFECT_VIBRATO:
        pitch = pitch_ + Lfo(time_) * effect_pitch_;
        oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch);
        break;

      case EFFECT_FADEOUT:
        oscillator_.SetVolume(
            static_cast<float>(effect_volume_) *
            (1.0f - static_cast<float>(time_ - effect_time_) / one_note_time_));
        break;
    }
  }

  time_++;

  if (time_ == total_note_time_) {
    NextSound();
  }
}

void Channel::NextSound() {
  play_index_ += 1;

  if (play_index_ < sound_list_.size()) {
    PlaySound();
  } else if (is_loop_) {
    play_index_ = 0;
    PlaySound();
  } else {
    StopPlaying();
  }
}

}  // namespace pyxelcore
