#include "pyxelcore/oscillator.h"

namespace pyxelcore {

Oscillator::Oscillator() {
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

int32_t Oscillator::Triangle(int32_t period, int32_t phase) {
  float x = static_cast<float>(phase) / period + 0.25f;
  x -= static_cast<int32_t>(x);

  return Abs(x * 4.0f - 2.0f) - 1.0f;
}

int32_t Oscillator::Square(int32_t period, int32_t phase) {
  float x = static_cast<float>(phase) / period;
  x -= static_cast<int32_t>(x);

  return (x < 0.5f ? 1.0f : -1.0f) * 0.2f;
}

int32_t Oscillator::Pulse(int32_t period, int32_t phase) {
  float x = static_cast<float>(phase) / period;
  x -= static_cast<int32_t>(x);

  return (x < 0.25f ? 1.0f : -1.0f) * 0.2f;
}

int32_t Oscillator::Noise(int32_t period, int32_t phase) {
  if (phase % (period / 4) == 0) {
    noise_seed_ >>= 1;
    noise_seed_ |= ((noise_seed_ ^ (noise_seed_ >> 1)) & 1) << 15;
    noise_last_ = noise_seed_ & 1;
  }

  return noise_last_ * 0.5f;
}

}  // namespace pyxelcore
