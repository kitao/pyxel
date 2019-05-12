#include "pyxelcore/oscillator.h"

namespace pyxelcore {

Oscillator::Oscillator() {
  phase_ = 0;
  tone_ = TONE_TRIANGLE;
  period_ = 0;
  volume_ = 0;

  next_tone_ = nullptr;
  next_period_ = 0;
  next_volume_ = 0;

  noise_seed_ = 0x8000;
  noist_last_ = 0;
}

void Oscillator::SetTone(int32_t tone) {
  if (tone == TONE_TRIANGLE) {
    next_tone_ = &Oscillator::Triangle;
  } else if (tone == TONE_SQUARE) {
    next_tone_ = &Oscillator::Square;
  } else if (tone == TONE_PULSE) {
    next_tone_ = &Oscillator::Pulse;
  } else if (tone == TONE_NOISE) {
    next_tone_ = &Oscillator::Noise;
  } else {
    next_tone_ = nullptr;
  }
}

void Oscillator::SetPeriod(int32_t period) {
  next_period_ = period;
}

void Oscillator::SetVolume(int32_t volume) {
  next_volume_ = volume;
}

void Oscillator::Stop() {
  next_tone_ = nullptr;
  next_period_ = 0;
  next_volume_ = 0;
}

int32_t Oscillator::Output() {
  /*
        if self._phase == 0:
            self._period = self._next_period
            self._tone = self._next_tone
            self._volume = self._next_volume

        if self._tone:
            output = self._tone(self._period, self._phase) * self._volume
            self._phase = (self._phase + 1) % self._period
        else:
            output = 0

        return output
        */

  //(this->*next_tone_)(10, 10);

  return 0;
}

int32_t Oscillator::Triangle(int32_t period, int32_t phase) {
  /*
        x = (phase / period + 0.25) % 1
        return abs(x * 4 - 2) - 1
        */
  return 0;
}

int32_t Oscillator::Square(int32_t period, int32_t phase) {
  /*
        x = (phase / period) % 1
        return (x < 0.5 and 1 or -1) * 0.2
        */
  return 0;
}

int32_t Oscillator::Pulse(int32_t period, int32_t phase) {
  /*
        x = (phase / period) % 1
        return (x < 0.25 and 1 or -1) * 0.2
        */
  return 0;
}

int32_t Oscillator::Noise(int32_t period, int32_t phase) {
  /*
          if phase % (period // 4) == 0:
            self._noise_seed >>= 1
            self._noise_seed |= ((self._noise_seed ^ (self._noise_seed >> 1)) &
1) << 15 self._noise_last = self._noise_seed & 1

        return self._noise_last * 0.5
        */
  return 0;
}

}  // namespace pyxelcore
