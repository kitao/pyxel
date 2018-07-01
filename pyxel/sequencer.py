import sounddevice as sd

TRACK_COUNT = 4

SAMPLE_RATE = 22050
BLOCK_SIZE = 220
NOTE_PITCH = [440 * pow(2, (note - 69) / 12) for note in range(128)]


def osc_triangle(x):
    return (abs((x % 1) * 2 - 1) * 2 - 1) * 0.7


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

        self._pattern = [0x45, 0x46, 0x47, 0x48, 0x49]
        self._cur_note = 0

        self._speed = 1
        self._time = 0

        self._noise_seed = 0x8000
        self._noise_short = False

    def next_data(self):
        if not self._is_playing:
            return 0

        pitch = NOTE_PITCH[self._pattern[self._cur_note]]

        data = OSCILLATOR[0](pitch / SAMPLE_RATE * self._time) * self._volume

        self._time += 1

        if self._time == 3000:
            self._time = 0
            self._cur_note += 1
            if self._cur_note >= len(self._pattern):
                self._is_playing = False

        return data

    def play(self, sound, loop):
        self._is_playing = True
        self._cur_note = 0
        self._time = 0


class Sequencer:
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


'''
[NOTES]

00: C -1    0C: C  0    18: C  1    24: C  2    30: C  3
01: C#-1    0D: C# 0    19: C# 1    25: C# 2    31: C# 3
02: D -1    0E: D  0    1A: D  1    26: D  2    32: D  3
03: Eb-1    0F: Eb 0    1B: Eb 1    27: Eb 2    33: Eb 3
04: E -1    10: E  0    1C: E  1    28: E  2    34: E  3
05: F -1    11: F  0    1D: F  1    29: F  2    35: F  3
06: F#-1    12: F# 0    1E: F# 1    2A: F# 2    36: F# 3
07: G -1    13: G  0    1F: G  1    2B: G  2    37: G  3
08: G#-1    14: G# 0    20: G# 1    2C: G# 2    38: G# 3
09: A -1    15: A  0    21: A  1    2D: A  2    39: A  3
0A: Bb-1    16: Bb 0    22: Bb 1    2E: Bb 2    3A: Bb 3
0B: B -1    17: B  0    23: B  1    2F: B  2    3B: B  3

3C: C  4    48: C  5    54: C  6    60: C  7    6C: C  8
3D: C# 4    49: C# 5    55: C# 6    61: C# 7    6D: C# 8
3E: D  4    4A: D  5    56: D  6    62: D  7    6E: D  8
3F: Eb 4    4B: Eb 5    57: Eb 6    63: Eb 7    6F: Eb 8
40: E  4    4C: E  5    58: E  6    64: E  7    70: E  8
41: F  4    4D: F  5    59: F  6    65: F  7    71: F  8
42: F# 4    4E: F# 5    5A: F# 6    66: F# 7    72: F# 8
43: G  4    4F: G  5    5B: G  6    67: G  7    73: G  8
44: G# 4    50: G# 5    5C: G# 6    68: G# 7    74: G# 8
45: A  4    51: A  5    5D: A  6    69: A  7    75: A  8
46: Bb 4    52: Bb 5    5E: Bb 6    6A: Bb 7    76: Bb 8
47: B  4    53: B  5    5F: B  6    6B: B  7    77: B  8
'''
