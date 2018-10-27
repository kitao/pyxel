import sounddevice as sd

from .constants import (
    AUDIO_BLOCK_SIZE,
    AUDIO_CHANNEL_COUNT,
    AUDIO_MUSIC_COUNT,
    AUDIO_ONE_SPEED,
    AUDIO_ONE_VOLUME,
    AUDIO_SAMPLE_RATE,
    AUDIO_SOUND_COUNT,
    SOUND_EFFECT_FADEOUT,
    SOUND_EFFECT_SLIDE,
    SOUND_EFFECT_VIBRATO,
)
from .music import Music
from .oscillator import Oscillator
from .sound import Sound


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
        self._one_note_time = sound.speed * AUDIO_ONE_SPEED
        self._total_note_time = self._one_note_time * len(sound.note)

    def _update(self):
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
                self._tone = sound.tone[pos % len(sound.tone)] if sound.tone else 0
                self._pitch = self._note_to_pitch(self._note)
                self._effect = (
                    sound.effect[pos % len(sound.effect)] if sound.effect else 0
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
                pitch = self._pitch + self._lfo(self._time) * self._effect_pitch
                self._oscillator.set_period(AUDIO_SAMPLE_RATE // pitch)
            elif self._effect == SOUND_EFFECT_FADEOUT:
                self._oscillator.set_volume(
                    self._effect_volume
                    * (1 - ((self._time - self._effect_time) / self._one_note_time))
                )

        self._time += 1

        if self._time == self._total_note_time:
            self._next_sound()

    def _next_sound(self):
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
        try:
            self._output_stream = sd.OutputStream(
                samplerate=AUDIO_SAMPLE_RATE,
                blocksize=AUDIO_BLOCK_SIZE,
                channels=1,
                dtype="int16",
                callback=self._output_stream_callback,
            )
        except sd.PortAudioError:
            self._output_stream = None

        self._channel_list = [Channel() for _ in range(AUDIO_CHANNEL_COUNT)]
        self._sound_list = [Sound() for _ in range(AUDIO_SOUND_COUNT)]
        self._music_list = [Music() for _ in range(AUDIO_MUSIC_COUNT)]

    @property
    def output_stream(self):
        return self._output_stream

    def sound(self, snd, *, system=False):
        if not system and snd == AUDIO_SOUND_COUNT - 1:
            raise ValueError("sound bank {} is reserved for system".format(snd))

        return self._sound_list[snd]

    def music(self, msc):
        return self._music_list[msc]

    def play(self, ch, snd, *, loop=False):
        if isinstance(snd, list):
            sound_list = [self._sound_list[s] for s in snd]
        else:
            sound_list = [self._sound_list[snd]]

        self._channel_list[ch].play(sound_list, loop)

    def playm(self, msc, *, loop=False):
        music = self._music_list[msc]

        if music.ch0:
            self.play(0, music.ch0, loop=loop)

        if music.ch1:
            self.play(1, music.ch1, loop=loop)

        if music.ch2:
            self.play(2, music.ch2, loop=loop)

        if music.ch3:
            self.play(3, music.ch3, loop=loop)

    def stop(self, ch=None):
        if ch is None:
            for i in range(AUDIO_CHANNEL_COUNT):
                self._channel_list[i].stop()
        else:
            self._channel_list[ch].stop()

    def _output_stream_callback(self, outdata, frames, time, status):
        for i in range(frames):
            output = 0
            for channel in self._channel_list:
                output += channel.output()
            outdata[i] = output
