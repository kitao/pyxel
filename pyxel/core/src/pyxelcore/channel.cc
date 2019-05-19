#include "pyxelcore/channel.h"

#include "pyxelcore/sound.h"

namespace pyxelcore {

Channel::Channel() {
  is_playing_ = false;
  is_loop_ = false;
  sound_length_ = 0;
  sound_index_ = 0;

  time_ = 0;
  one_note_time_ = 0;
  sound_index_ = 0;

  tone_ = 0;
  note_ = 0;
  pitch_ = 0;
  volume_ = 0;
  effect_ = 0;

  effect_time_ = 0;
  effect_pitch_ = 0;
  effect_volume_ = 0;
}

void Channel::PlaySound(Sound** sound, int32_t sound_length, bool loop) {
  is_playing_ = true;
  is_loop_ = loop;
  sound_length_ = Min(sound_length, MAX_MUSIC_LENGTH);
  sound_index_ = 0;

  for (int32_t i = 0; i < sound_length_; i++) {
    sound_[i] = sound[i];
  }

  PlaySound();
}

void Channel::StopPlaying() {
  is_playing_ = false;
  pitch_ = 0;
  oscillator_.Stop();
}

void Channel::PlaySound() {
  Sound* sound = sound_[sound_index_];

  time_ = 0;
  one_note_time_ = sound->Speed() * AUDIO_ONE_SPEED;
  total_note_time_ = one_note_time_ * sound->NoteLength();
}

void Channel::NextNote() {
  if (!is_playing_) {
    return;
  }

  if (total_note_time_ == 0) {
    NextSound();
    return;
  }

  // forward note
  if (time_ % one_note_time_ == 0) {
    Sound* sound = sound_[sound_index_];
    int32_t pos = static_cast<float>(time_ / one_note_time_);
    note_ = sound->Note()[pos];
    volume_ = (sound->VolumeLength() > 0
                   ? sound->Volume()[pos % sound->VolumeLength()]
                   : 7) *
              AUDIO_ONE_VOLUME;

    if (note_ >= 0 && volume_ > 0) {
      int32_t last_pitch = pitch_;
      tone_ = sound->ToneLength() > 0 ? sound->Tone()[pos % sound->ToneLength()]
                                      : TONE_TRIANGLE;
      pitch_ = NoteToPitch(note_);
      effect_ = sound->EffectLength() > 0
                    ? sound->Effect()[pos % sound->EffectLength()]
                    : EFFECT_NONE;

      oscillator_.SetTone(tone_);
      oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch_);
      oscillator_.SetVolume(volume_);

      switch (effect_) {
        case EFFECT_SLIDE:
          effect_time_ = time_;
          effect_pitch_ = last_pitch > 0 ? last_pitch : pitch_;
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
  if (note_ >= 0) {
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
            (1.0f -
             ((static_cast<float>(time_) - effect_time_) / one_note_time_)));
        break;
    }
  }

  time_++;

  if (time_ == total_note_time_) {
    NextSound();
  }
}

void Channel::NextSound() {
  sound_index_ += 1;

  if (sound_index_ < sound_length_) {
    PlaySound();
  } else if (is_loop_) {
    sound_index_ = 0;
    PlaySound();
  } else {
    StopPlaying();
  }
}

}  // namespace pyxelcore