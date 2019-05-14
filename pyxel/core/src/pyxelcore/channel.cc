#include "pyxelcore/channel.h"

namespace pyxelcore {

Channel::Channel() {
  /*
          self._oscillator = Oscillator()

          self._is_playing = False
          self._is_loop = False
          self._sound_list = None
          self._sound_index = 0

          self._time = 0
          self._one_note_time = 0
          self._total_note_time = 0

          self._tone = None
          self._note = 0
          self._pitch = 0
          self._volume = 0
          self._effect = 0

          self._effect_time = 0
          self._effect_pitch = 0
          self._effect_volume = 0
  */
}

Channel::~Channel() {}

void Channel::Play(int32_t* sound, int32_t sound_count, bool loop) {
  //
  /*
            self._is_playing = True
          self._is_loop = loop
          self._sound_list = sound_list
          self._sound_index = 0

          self._play_sound()
          */
}

void Channel::Stop() {
  //
  /*
            self._is_playing = False
          self._pitch = 0
          self._oscillator.stop()
          */
}

int32_t Channel::Output() {
  /*
            self._update()
          return self._oscillator.output()
          */
  return 0;
}

void Channel::PlaySound() {
  //
  /*
            sound = self._sound_list[self._sound_index]

          self._time = 0
          self._one_note_time = sound.speed * AUDIO_ONE_SPEED
          self._total_note_time = self._one_note_time * len(sound.note)
          */
}

void Channel::Update() {
  /*
            if not self._is_playing:
              return

          if self._total_note_time == 0:
              self._next_sound()
              return

          # forward note
          if self._time % self._one_note_time == 0:
              sound = self._sound_list[self._sound_index]
              pos = int(self._time / self._one_note_time)
              self._note = sound.note[pos]
              self._volume = (
                  sound.volume[pos % len(sound.volume)] if sound.volume else 7
              ) * AUDIO_ONE_VOLUME

              if self._note >= 0 and self._volume > 0:
                  last_pitch = self._pitch
                  self._tone = sound.tone[pos % len(sound.tone)] if sound.tone
  else 0 self._pitch = self._note_to_pitch(self._note) self._effect = (
                      sound.effect[pos % len(sound.effect)] if sound.effect else
  0
                  )

                  self._oscillator.set_tone(self._tone)
                  self._oscillator.set_period(AUDIO_SAMPLE_RATE // self._pitch)
                  self._oscillator.set_volume(self._volume)

                  if self._effect == SOUND_EFFECT_SLIDE:
                      self._effect_time = self._time
                      self._effect_pitch = last_pitch or self._pitch
                  elif self._effect == SOUND_EFFECT_VIBRATO:
                      self._effect_time = self._time
                      self._effect_pitch = (
                          self._note_to_pitch(self._note + 0.5) - self._pitch
                      )
                  elif self._effect == SOUND_EFFECT_FADEOUT:
                      self._effect_time = self._time
                      self._effect_volume = self._volume
              else:
                  self._oscillator.stop()

          # play note
          if self._note >= 0:
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

          self._time += 1

          if self._time == self._total_note_time:
              self._next_sound()
              */
}

void Channel::NextSound() {
  //
  /*
            self._sound_index += 1

          if self._sound_index < len(self._sound_list):
              self._play_sound()
          elif self._is_loop:
              self._sound_index = 0
              self._play_sound()
          else:
              self.stop()
              */
}

int32_t Channel::NoteToPitch() {
  //
  /*
            return 440 * pow(2, (note - 33) / 12)
            */
}

int32_t Channel::Lfo(int32_t time) {
  //
  /*
            x = (time * 8 / AUDIO_SAMPLE_RATE + 0.25) % 1
          return abs(x * 4 - 2) - 1
          */
}

}  // namespace pyxelcore