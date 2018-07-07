import sounddevice as sd

SAMPLE_RATE = 44100
BLOCK_SIZE = 441
TRACK_COUNT = 4


class Track:
    def __init__(self):
        self._is_playing = False
        self._sound = None
        self._time = 0
        self._note = None
        self._pitch = 1
        self._period = 0
        self._tone = 0
        self._volume = 0
        self._effect = 0

        self._cur_phase = 0
        self._cur_period = 0
        self._cur_tone = None
        self._cur_volume = 0

        self._noise_seed = 0x8000
        self._noise_last = 0

        self._tone_list = [
            self._triangle, self._square, self._pulse, self._noise
        ]

    def play(self, sound, loop):
        self._is_playing = True
        self._sound = sound
        self._time = 0
        self._one_note_time = int(sound.speed * SAMPLE_RATE / 120)
        self._total_note_time = self._one_note_time * len(sound.note)

    def next_data(self):
        self.update()
        return self.output()

    def update(self):
        if not self._is_playing:
            return

        sound = self._sound

        if self._time % self._one_note_time == 0:
            offset = int(self._time / self._one_note_time)
            volume = sound.volume[offset]
            note = self._note = sound.note[offset]

            if note >= 0:
                last_pitch = self._pitch
                self._effect = sound.effect[offset]
                self._pitch = self._note_to_pitch(note)

                self._tone = self._tone_list[sound.tone[offset]]
                self._period = SAMPLE_RATE // self._pitch
                self._volume = volume * 1023

                if self._effect == 1:  # EFFECT_SLIDE
                    self._effect_pitch = last_pitch
                    self._effect_start_time = self._time
                elif self._effect == 2:  # EFFECT_VIBRATO
                    self._effect_pitch = self._note_to_pitch(note + 0.5) - self._pitch
                    self._effect_start_time = self._time
                elif self._effect == 3:  # EFFECT_FADEOUT
                    self._effect_volume = self._volume
                    self._effect_start_time = self._time
            else:
                self._tone = None
                self._period = 0
                self._volume = 0

        if self._note >= 0:
            pitch = self._pitch
            volume = self._volume * 1023

            if self._effect == 1:  # EFFECT_SLIDE
                alpha = ((self._time - self._effect_start_time) /
                         self._one_note_time)
                pitch = self._pitch * alpha + self._effect_pitch * (1 - alpha)
                self._period = SAMPLE_RATE // pitch
            elif self._effect == 2:  # EFFECT_VIBRATO
                alpha = self._triangle(SAMPLE_RATE // 8, self._time)
                pitch = self._pitch + self._effect_pitch * alpha
                self._period = SAMPLE_RATE // pitch
            elif self._effect == 3:  # EFFECT_FADEOUT
                self._volume = self._effect_volume * (1 - (
                    (self._time - self._effect_start_time) /
                    self._one_note_time))

        self._time += 1

        if self._time >= self._total_note_time:
            self._is_playing = False

            self._tone = None
            self._period = 0
            self._volume = 0

    def output(self):
        if self._cur_phase == 0:
            self._cur_period = self._period
            self._cur_tone = self._tone
            self._cur_volume = self._volume

        if self._cur_tone:
            data = self._cur_tone(self._cur_period,
                                  self._cur_phase) * self._cur_volume
            self._cur_phase = (self._cur_phase + 1) % self._cur_period
        else:
            data = 0

        return data

    @staticmethod
    def _note_to_pitch(note):
        return 440 * pow(2, (note - 33) / 12)

    def _triangle(self, period, phase):
        x = (phase / period + 0.25) % 1
        return (abs(x * 4 - 2) - 1) * 0.7

    def _square(self, period, phase):
        x = (phase / period) % 1
        return (x < 0.5 and 1 or -1) / 3

    def _pulse(self, period, phase):
        x = (phase / period) % 1
        return (x < 0.25 and 1 or -1) / 3

    def _noise(self, period, phase):
        if phase % (period // 4) == 0:
            self._noise_seed >>= 1
            self._noise_seed |= ((self._noise_seed ^
                                  (self._noise_seed >> 1)) & 1) << 15
            self._noise_last = self._noise_seed & 1

        return self._noise_last


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

    def _output_stream_callback(self, outdata, frames, time, status):
        for i in range(frames):
            data = 0

            for track in self._track_list:
                data += track.next_data()

            outdata[i] = data
