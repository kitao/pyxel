#ifndef PYXELCORE_OSCILLATOR_H_
#define PYXELCORE_OSCILLATOR_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Oscillator {
 public:
  Oscillator();

  void SetTone(int32_t tone);
  void SetPeriod(float period);
  void SetVolume(int32_t volume);
  void Stop();
  int16_t Output();

 private:
  float phase_;
  float (Oscillator::*tone_)(float period, float phase);
  float period_;
  int32_t volume_;

  float (Oscillator::*next_tone_)(float period, float phase);
  float next_period_;
  int32_t next_volume_;

  int32_t noise_seed_;
  int32_t noise_last_;

  float Triangle(float period, float phase);
  float Square(float period, float phase);
  float Pulse(float period, float phase);
  float Noise(float period, float phase);
};

inline Oscillator::Oscillator() {
  phase_ = 0.0f;
  tone_ = nullptr;
  period_ = 0.0f;
  volume_ = 0;

  next_tone_ = nullptr;
  next_period_ = 0.0f;
  next_volume_ = 0;

  noise_seed_ = 0x8000;
  noise_last_ = 0;
}

inline void Oscillator::SetTone(int32_t tone) {
  switch (tone) {
    case TONE_TRIANGLE:
      next_tone_ = &Oscillator::Triangle;
      break;

    case TONE_SQUARE:
      next_tone_ = &Oscillator::Square;
      break;

    case TONE_PULSE:
      next_tone_ = &Oscillator::Pulse;
      break;

    case TONE_NOISE:
      next_tone_ = &Oscillator::Noise;
      break;
  }
}

inline void Oscillator::SetPeriod(float period) {
  next_period_ = period;
}

inline void Oscillator::SetVolume(int32_t volume) {
  next_volume_ = volume;
}

inline void Oscillator::Stop() {
  next_tone_ = nullptr;
  next_period_ = 0.0f;
  next_volume_ = 0;
}

inline int16_t Oscillator::Output() {
  if (phase_ >= 0.0f && phase_ < 1.0f) {
    period_ = next_period_;
    tone_ = next_tone_;
    volume_ = next_volume_;
  }

  int32_t output;

  if (tone_) {
    output = (this->*tone_)(period_, phase_) * volume_;
    phase_ = fmod(phase_ + 1.0f, period_);
  } else {
    output = 0;
  }

  return output;
}

inline float Oscillator::Triangle(float period, float phase) {
  float x = phase / period + 0.75f;

  float n;
  x = modff(x, &n);

  return Abs(x * 4.0f - 2.0f) - 1.0f;
}

inline float Oscillator::Square(float period, float phase) {
  float x = phase / period;

  float n;
  x = modff(x, &n);

  return (x < 0.5f ? 1.0f : -1.0f) * 0.2f;
}

inline float Oscillator::Pulse(float period, float phase) {
  float x = phase / period;

  float n;
  x = modff(x, &n);

  return (x < 0.25f ? 1.0f : -1.0f) * 0.2f;
}

inline float Oscillator::Noise(float period, float phase) {
  float x = fmod(phase, period / 4.0f);

  if (x >= 0.0f && x < 1.0f) {
    noise_seed_ >>= 1;
    noise_seed_ |= ((noise_seed_ ^ (noise_seed_ >> 1)) & 1) << 15;
    noise_last_ = noise_seed_ & 1;
  }

  return noise_last_ * 0.5f;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_OSCILLATOR_H_
