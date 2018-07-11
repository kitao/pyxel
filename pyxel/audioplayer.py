import sounddevice as sd
from .oscillator import Oscillator
from .sound import Sound
from .constants import (
    AUDIO_SAMPLE_RATE,
    AUDIO_BLOCK_SIZE,
    AUDIO_CHANNEL_COUNT,
    AUDIO_SOUND_COUNT,
    AUDIO_ONE_SPEED,
    AUDIO_ONE_VOLUME,
    SOUND_EFFECT_SLIDE,
    SOUND_EFFECT_VIBRATO,
    SOUND_EFFECT_FADEOUT,
)


class Channel:
    def __init__(self):
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

    def play(self, sound_list, loop):
        self._is_playing = True
        self._is_loop = loop
        self._sound_list = sound_list
        self._sound_index = 0

        self._play_sound()

    def stop(self):
        self._is_playing = False
        self._pitch = 0
        self._oscillator.stop()

    def output(self):
        self._update()
        return self._oscillator.output()

    def _play_sound(self):
        sound = self._sound_list[self._sound_index]

        self._time = 0
        self._one_note_time = sound._speed * AUDIO_ONE_SPEED
        self._total_note_time = self._one_note_time * len(sound._note)

    def _update(self):
        if not self._is_playing:
            return

        # forward note
        if self._time % self._one_note_time == 0:
            sound = self._sound_list[self._sound_index]
            pos = int(self._time / self._one_note_time)
            self._note = sound._note[pos]
            self._volume = sound._volume[pos] * AUDIO_ONE_VOLUME

            if self._note >= 0 and self._volume > 0:
                last_pitch = self._pitch
                self._tone = sound._tone[pos]
                self._pitch = self._note_to_pitch(self._note)
                self._effect = sound._effect[pos]

                self._oscillator.set_tone(self._tone)
                self._oscillator.set_period(AUDIO_SAMPLE_RATE // self._pitch)
                self._oscillator.set_volume(self._volume)

                if self._effect == SOUND_EFFECT_SLIDE:
                    self._effect_time = self._time
                    self._effect_pitch = last_pitch or self._pitch
                elif self._effect == SOUND_EFFECT_VIBRATO:
                    self._effect_time = self._time
                    self._effect_pitch = self._note_to_pitch(self._note +
                                                             0.5) - self._pitch
                elif self._effect == SOUND_EFFECT_FADEOUT:
                    self._effect_time = self._time
                    self._effect_volume = self._volume
            else:
                self._oscillator.stop()

        # play note
        if self._note >= 0:
            if self._effect == SOUND_EFFECT_SLIDE:
                a = ((self._time - self._effect_time) / self._one_note_time)
                pitch = self._pitch * a + self._effect_pitch * (1 - a)
                self._oscillator.set_period(AUDIO_SAMPLE_RATE // pitch)
            elif self._effect == SOUND_EFFECT_VIBRATO:
                pitch = self._pitch + self._lfo(
                    self._time) * self._effect_pitch
                self._oscillator.set_period(AUDIO_SAMPLE_RATE // pitch)
            elif self._effect == SOUND_EFFECT_FADEOUT:
                self._oscillator.set_volume(self._effect_volume * (1 - (
                    (self._time - self._effect_time) / self._one_note_time)))

        self._time += 1

        if self._time == self._total_note_time:
            self._sound_index += 1

            if self._sound_index < len(self._sound_list):
                self._play_sound()
            elif self._is_loop:
                self._sound_index = 0
                self._play_sound()
            else:
                self.stop()

    @staticmethod
    def _note_to_pitch(note):
        return 440 * pow(2, (note - 33) / 12)

    @staticmethod
    def _lfo(time):
        x = (time * 8 / AUDIO_SAMPLE_RATE + 0.25) % 1
        return abs(x * 4 - 2) - 1


class AudioPlayer:
    def __init__(self):
        self._output_stream = sd.OutputStream(
            samplerate=AUDIO_SAMPLE_RATE,
            blocksize=AUDIO_BLOCK_SIZE,
            channels=1,
            dtype='int16',
            callback=self._output_stream_callback)

        self._channel_list = [Channel() for _ in range(AUDIO_CHANNEL_COUNT)]
        self._sound_list = [Sound() for _ in range(AUDIO_SOUND_COUNT)]

    @property
    def output_stream(self):
        return self._output_stream

    def sound(self, index):
        return self._sound_list[index]

    def play(self, ch, sound, *, loop=False):
        if isinstance(sound, list):
            sound_list = [self._sound_list[s] for s in sound]
        else:
            sound_list = [self._sound_list[sound]]

        self._channel_list[ch].play(sound_list, loop)

    def stop(self, ch):
        self._channel_list[ch].stop()

    def _output_stream_callback(self, outdata, frames, time, status):
        for i in range(frames):
            output = 0
            for channel in self._channel_list:
                output += channel.output()
            outdata[i] = output
