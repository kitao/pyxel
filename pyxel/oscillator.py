from .constants import (
    SOUND_TONE_TRIANGLE,
    SOUND_TONE_SQUARE,
    SOUND_TONE_PULSE,
    SOUND_TONE_NOISE,
)


class Oscillator:
    def __init__(self):
        self._phase = 0
        self._tone = None
        self._period = 0
        self._volume = 0

        self._next_tone = None
        self._next_period = 0
        self._next_volume = 0

        self._noise_seed = 0x8000
        self._noise_last = 0

        self._tone_list = {
            SOUND_TONE_TRIANGLE: self._triangle,
            SOUND_TONE_SQUARE: self._square,
            SOUND_TONE_PULSE: self._pulse,
            SOUND_TONE_NOISE: self._noise,
        }

    def set_tone(self, tone):
        self._next_tone = self._tone_list[tone] if tone is not None else None

    def set_period(self, period):
        self._next_period = period

    def set_volume(self, volume):
        self._next_volume = volume

    def stop(self):
        self._next_tone = None
        self._next_period = 0
        self._next_volume = 0

    def output(self):
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

    def _triangle(self, period, phase):
        x = (phase / period + 0.25) % 1
        return (abs(x * 4 - 2) - 1)

    def _square(self, period, phase):
        x = (phase / period) % 1
        return (x < 0.5 and 1 or -1) * 0.2

    def _pulse(self, period, phase):
        x = (phase / period) % 1
        return (x < 0.25 and 1 or -1) * 0.2

    def _noise(self, period, phase):
        if phase % (period // 4) == 0:
            self._noise_seed >>= 1
            self._noise_seed |= ((self._noise_seed ^
                                  (self._noise_seed >> 1)) & 1) << 15
            self._noise_last = self._noise_seed & 1

        return self._noise_last * 0.5
