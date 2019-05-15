#include "pyxelcore/channel.h"

#include "pyxelcore/sound.h"

namespace pyxelcore {

Channel::Channel() {
  is_playing_ = false;
  is_loop_ = false;
  sound_count_ = 0;
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

void Channel::Play(Sound** sound, int32_t sound_count, bool loop) {
  is_playing_ = true;
  is_loop_ = loop;
  sound_count_ = Min(sound_count, MAX_MUSIC_LENGTH);
  sound_index_ = 0;

  for (int32_t i = 0; i < sound_count_; i++) {
    sound_[i] = sound[i];
  }

  PlaySound();
}

void Channel::Stop() {
  is_playing_ = false;
  pitch_ = 0;
  oscillator_.Stop();
}

int32_t Channel::Output() {
  Update();

  return oscillator_.Output();
}

void Channel::PlaySound() {
  Sound* sound = sound_[sound_index_];

  time_ = 0;
  one_note_time_ = sound->Speed() * AUDIO_ONE_SPEED;
  total_note_time_ = one_note_time_ * sound->NoteLength();
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
    Sound* sound = sound_[sound_index_];
    int32_t pos = static_cast<float>(time_ / one_note_time_);
    note_ = sound->Note()[pos];
    volume_ =
        (sound->Volume() ? sound->Volume()[pos % sound->VolumeLength()]
                             : 7) *
        AUDIO_ONE_VOLUME;

    if (note_ >= 0 && volume_ > 0) {
      int32_t last_pitch = pitch_;
      tone_ = sound->Tone() ? sound->Tone()[pos % sound->ToneLength()] : 0;
      pitch_ = NoteToPitch(note_);
      effect_ = sound->Effect() ? sound->Effect()[pos % sound->EffectLength()] : 0;

      oscillator_.SetTone(tone_);
      oscillator_.SetPeriod(AUDIO_SAMPLE_RATE / pitch_);
      oscillator_.SetVolume(volume_);

      if (effect_ == EFFECT_SLIDE) {
        effect_time_ = time_;
        effect_pitch_ = last_pitch ? last_pitch : pitch_;
      } else if (effect_ == EFFECT_VIBRATO) {
        effect_time_ = time_;
        effect_pitch_ = NoteToPitch(note_ + 0.5f) - pitch_;
      } else {
        oscillator_.Stop();
      }

      // play note
      if (note_ >= 0) {
        /*
                        if self._effect == SOUND_EFFECT_SLIDE:
                    a = (self._time - self._effect_time) / self._one_note_time
                    pitch = self._pitch * a + self._effect_pitch * (1 - a)
                    self._oscillator.set_period(AUDIO_SAMPLE_RATE // pitch)
                elif self._effect == SOUND_EFFECT_VIBRATO:
                    pitch = self._pitch + self._lfo(self._time) *
    self._effect_pitch self._oscillator.set_period(AUDIO_SAMPLE_RATE // pitch)
                elif self._effect == SOUND_EFFECT_FADEOUT:
                    self._oscillator.set_volume(
                        self._effect_volume
                        * (1 - ((self._time - self._effect_time) /
    self._one_note_time))
                    )
                    */
      }
      time_++;

      if (time_ == total_note_time_) {
        NextSound();
      }
    }
  }
}

void Channel::NextSound() {
  sound_index_ += 1;

  if (sound_index_ < sound_count_) {
    PlaySound();
  } else if (is_loop_) {
    sound_index_ = 0;
    PlaySound();
  } else {
    Stop();
  }
}

int32_t Channel::NoteToPitch(int32_t note) {
  return 440.0f * pow(2.0f, (note - 33.0f) / 12.0f);
}

int32_t Channel::Lfo(int32_t time) {
  float x = (time * 8 / AUDIO_SAMPLE_RATE + 0.25f);
  x -= static_cast<int32_t>(x);

  return Abs(x * 4.0f - 2.0f) - 1.0f;
}

}  // namespace pyxelcore