import sounddevice as sd

SAMPLE_RATE = 44100
BLOCK_SIZE = 441
TRACK_COUNT = 4

NOTE_PITCH = [440 * pow(2, (note - 69) / 12) for note in range(128)]


class Track:
    def __init__(self):
        self._is_playing = False
        self._sound = None
        self._time = 0
        self._note = None
        self._pitch = 0
        self._volume = 0

        self._noise_seed = 0x8000
        self._noise_last = 0

        self._tones = [self._triangle, self._square, self._pulse, self._noise]

    def play(self, sound, loop):
        self._is_playing = True
        self._sound = sound
        self._time = 0

    def next_data(self):
        if not self._is_playing:
            return 0

        sound = self._sound
        offset = int(self._time / (sound.speed * SAMPLE_RATE / 120))

        if offset >= sound.length:
            self._is_playing = False
            return 0

        if self._time == 0:
            note = self._note = sound.note[offset]
            self._pitch = note and NOTE_PITCH[note] or 0
            self._volume = note and sound.volume[offset] or 0

        if self._note:
            data = self._tones[sound.tone[offset]](self._pitch, self._time,
                                                   self._volume * 1023)
        else:
            data = 0

        self._time += 1

        return data

    def _triangle(self, pitch, time, volume):
        x = (time * pitch / SAMPLE_RATE) % 1
        return (abs(x * 4 - 2) - 1) * 0.7 * volume

    def _square(self, pitch, time, volume):
        x = (time * pitch / SAMPLE_RATE) % 1
        return (x < 0.5 and 1 or -1) / 3 * volume

    def _pulse(self, pitch, time, volume):
        x = (time * pitch / SAMPLE_RATE) % 1
        return (x < 0.25 and 1 or -1) / 3 * volume

    def _noise(self, pitch, time, volume):
        if (time % (SAMPLE_RATE // pitch) == 0):
            self._noise_seed >>= 1
            self._noise_seed |= ((self._noise_seed ^
                                  (self._noise_seed >> 1)) & 1) << 15
            self._noise_last = self._noise_seed & 1

        return self._noise_last * volume


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
