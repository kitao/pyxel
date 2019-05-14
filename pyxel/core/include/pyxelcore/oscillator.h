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
  int32_t Output();

 private:
  int32_t phase_;
  int32_t (Oscillator::*tone_)(int32_t period, int32_t phase);
  int32_t period_;
  int32_t volume_;

  int32_t (Oscillator::*next_tone_)(int32_t period, int32_t phase);
  int32_t next_period_;
  int32_t next_volume_;

  int32_t noise_seed_;
  int32_t noise_last_;

  int32_t Triangle(int32_t period, int32_t phase);
  int32_t Square(int32_t period, int32_t phase);
  int32_t Pulse(int32_t period, int32_t phase);
  int32_t Noise(int32_t period, int32_t phase);
};

}  // namespace pyxelcore

#endif  // PYXELCORE_OSCILLATOR_H_
