import sounddevice as sd

from .oscillator import Oscillator
from .sound import (
    EFFECT_SLIDE,
    EFFECT_VIBRATO,
    EFFECT_FADEOUT,
)

SAMPLE_RATE = 44100
BLOCK_SIZE = 441
TRACK_COUNT = 4


class Track:
    def __init__(self):
        self._oscillator = Oscillator()

        self._is_playing = False
        self._sound = None

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

    def play(self, sound, loop):
        self._is_playing = True
        self._sound = sound

        self._time = 0
        self._one_note_time = int(sound.speed * SAMPLE_RATE / 120)
        self._total_note_time = self._one_note_time * len(sound.note)

    def stop(self):
        self._is_playing = False
        self._pitch = 0
        self._oscillator.stop()

    def output(self):
        self._update()
        return self._oscillator.output()

    def _update(self):
        if not self._is_playing:
            return

        sound = self._sound

        # forward note
        if self._time % self._one_note_time == 0:
            offset = int(self._time / self._one_note_time)
            self._note = sound.note[offset]
            self._volume = sound.volume[offset] * 1023

            if self._note >= 0 and self._volume > 0:
                last_pitch = self._pitch
                self._tone = sound.tone[offset]
                self._pitch = self._note_to_pitch(self._note)
                self._effect = sound.effect[offset]

                self._oscillator.set_tone(self._tone)
                self._oscillator.set_period(SAMPLE_RATE // self._pitch)
                self._oscillator.set_volume(self._volume)

                if self._effect == EFFECT_SLIDE:
                    self._effect_time = self._time
                    self._effect_pitch = last_pitch or self._pitch
                elif self._effect == EFFECT_VIBRATO:
                    self._effect_time = self._time
                    self._effect_pitch = self._note_to_pitch(self._note +
                                                             0.5) - self._pitch
                elif self._effect == EFFECT_FADEOUT:
                    self._effect_time = self._time
                    self._effect_volume = self._volume
            else:
                self._oscillator.stop()

        # play note
        if self._note >= 0:
            if self._effect == EFFECT_SLIDE:
                a = ((self._time - self._effect_time) / self._one_note_time)
                pitch = self._pitch * a + self._effect_pitch * (1 - a)
                self._oscillator.set_period(SAMPLE_RATE // pitch)
            elif self._effect == EFFECT_VIBRATO:
                pitch = self._pitch + self._lfo(
                    self._time) * self._effect_pitch
                self._oscillator.set_period(SAMPLE_RATE // pitch)
            elif self._effect == EFFECT_FADEOUT:
                self._oscillator.set_volume(self._effect_volume * (1 - (
                    (self._time - self._effect_time) / self._one_note_time)))

        self._time += 1

        if self._time >= self._total_note_time:
            self.stop()

    @staticmethod
    def _note_to_pitch(note):
        return 440 * pow(2, (note - 33) / 12)

    @staticmethod
    def _lfo(time):
        x = (time * 8 / SAMPLE_RATE + 0.25) % 1
        return (abs(x * 4 - 2) - 1) * 0.7


class AudioPlayer:
    def __init__(self):
        self._output_stream = sd.OutputStream(
            samplerate=SAMPLE_RATE,
            blocksize=BLOCK_SIZE,
            channels=1,
            dtype='int16',
            callback=self._output_stream_callback)

        self._track_list = [Track() for _ in range(TRACK_COUNT)]

    @property
    def output_stream(self):
        return self._output_stream

    def play(self, track, sound, loop=False):
        self._track_list[track].play(sound, loop)

    def stop(self, track):
        self._track_list[track].stop()

    def _output_stream_callback(self, outdata, frames, time, status):
        for i in range(frames):
            output = 0
            for track in self._track_list:
                output += track.output()
            outdata[i] = output
