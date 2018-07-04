import sounddevice as sd

TRACK_COUNT = 4

SAMPLE_RATE = 22050
BLOCK_SIZE = 220
NOTE_PITCH = [440 * pow(2, (note - 69) / 12) for note in range(128)]


def osc_triangle(x):
    return (abs((x % 1) * 4 - 2) - 1) * 0.7


def osc_tilted_saw(x):
    x = x % 1
    return (((x < 0.875) and (x * 16 / 7) or ((1 - x) * 16)) - 1) * 0.7


def osc_saw(x):
    return (x % 1 - 0.5) * 0.9


def osc_square(x):
    return (x % 1 < 0.5 and 1 or -1) / 3


def osc_pulse(x):
    return (x % 1 < 0.3125 and 1 or -1) / 3


def osc_organ(x):
    x *= 4
    return (abs((x % 2) - 1) - 0.5 + (abs((
        (x * 0.5) % 2) - 1) - 0.5) / 2 - 0.1) * 0.7


def osc_noise(x):
    osc_noise.reg >>= 1
    osc_noise.reg |= ((osc_noise.reg ^ (osc_noise.reg >> 1)) & 1) << 15
    return osc_noise.reg & 1


osc_noise.reg = 0x8000


def osc_phaser(x):
    x = x * 2
    return abs((x % 2) - 1.5 +
               (abs((x * 127 / 128) % 2 - 1) - 0.5) / 2) - (1 / 4)


OSCILLATOR = [
    osc_triangle, osc_tilted_saw, osc_saw, osc_square, osc_pulse, osc_organ,
    osc_noise, osc_phaser
]


class Track:
    def __init__(self):
        self._note = 0x45
        self._tone = 0
        self._volume = 5000

        self._is_playing = False

        self._cur_note = 0

        self._sound = None

        self._speed = 1
        self._time = 0

        self._noise_seed = 0x8000
        self._noise_short = False

    def next_data(self):
        if not self._is_playing:
            return 0

        sound = self._sound

        no = self._cur_note
        note = sound.note[no]

        if note:
            pitch = NOTE_PITCH[note]
            data = (OSCILLATOR[sound.tone[no]](
                pitch / SAMPLE_RATE * self._time) * self._volume)
        else:
            data = 0

        self._time += 1

        if self._time == 3000:
            self._time = 0
            self._cur_note += 1
            if self._cur_note >= sound.length:
                self._is_playing = False

        return data

    def play(self, sound, loop):
        self._is_playing = True
        self._sound = sound
        self._cur_note = 0
        self._time = 0


class AudioPlayer:
    def __init__(self):
        self._step = 0

        self._output_stream = sd.OutputStream(
            samplerate=SAMPLE_RATE,
            blocksize=BLOCK_SIZE,
            channels=1,
            dtype='int16',
            callback=self._output_stream_callback)

        self._track_list = [Track() for _ in range(TRACK_COUNT)]

        self._track_list[1]._note = 0x3d
        self._track_list[2]._note = 0x40
        self._track_list[3]._note = 0x45

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
