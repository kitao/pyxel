#ifndef PYXELCORE_OSCILLATOR_H_
#define PYXELCORE_OSCILLATOR_H_

#include "pyxelcore/common.h"

namespace pyxelcore {

class Oscillator {
 public:
  Oscillator();

  void SetTone(int32_t tone);
  void SetPeriod(int32_t period);
  void SetVolume(int32_t volume);
  void Stop();
  int16_t Output();

 private:
  int32_t phase_;
  float (Oscillator::*tone_)(int32_t period, int32_t phase);
  int32_t period_;
  int32_t volume_;

  float (Oscillator::*next_tone_)(int32_t period, int32_t phase);
  int32_t next_period_;
  int32_t next_volume_;

  int32_t noise_seed_;
  int32_t noise_last_;

  float Triangle(int32_t period, int32_t phase);
  float Square(int32_t period, int32_t phase);
  float Pulse(int32_t period, int32_t phase);
  float Noise(int32_t period, int32_t phase);
};

inline Oscillator::Oscillator() {
  phase_ = 0;
  tone_ = nullptr;
  period_ = 0;
  volume_ = 0;

  next_tone_ = nullptr;
  next_period_ = 0;
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

    default:
      next_tone_ = nullptr;
  }
}

inline void Oscillator::SetPeriod(int32_t period) {
  next_period_ = period;
}

inline void Oscillator::SetVolume(int32_t volume) {
  next_volume_ = volume;
}

inline void Oscillator::Stop() {
  next_tone_ = nullptr;
  next_period_ = 0;
  next_volume_ = 0;
}

inline int16_t Oscillator::Output() {
  if (phase_ == 0) {
    period_ = next_period_;
    tone_ = next_tone_;
    volume_ = next_volume_;
  }

  int32_t output;

  if (tone_) {
    output = (this->*tone_)(period_, phase_) * volume_;
    phase_ = (phase_ + 1) % period_;
  } else {
    output = 0;
  }

  return output;
}

inline float Oscillator::Triangle(int32_t period, int32_t phase) {
  float x = static_cast<float>(phase) / period + 0.75f;
  x -= static_cast<int32_t>(x);

  return Abs(x * 4.0f - 2.0f) - 1.0f;
}

inline float Oscillator::Square(int32_t period, int32_t phase) {
  float x = static_cast<float>(phase) / period;
  x -= static_cast<int32_t>(x);

  return (x < 0.5f ? 1.0f : -1.0f) * 0.2f;
}

inline float Oscillator::Pulse(int32_t period, int32_t phase) {
  float x = static_cast<float>(phase) / period;
  x -= static_cast<int32_t>(x);

  return (x < 0.25f ? 1.0f : -1.0f) * 0.2f;
}

inline float Oscillator::Noise(int32_t period, int32_t phase) {
  if (phase % (period / 4) == 0) {
    noise_seed_ >>= 1;
    noise_seed_ |= ((noise_seed_ ^ (noise_seed_ >> 1)) & 1) << 15;
    noise_last_ = noise_seed_ & 1;
  }

  return noise_last_ * 0.5f;
}

}  // namespace pyxelcore

#endif  // PYXELCORE_OSCILLATOR_H_
