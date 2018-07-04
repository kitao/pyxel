import sounddevice as sd
from .instruments import INSTRUMENTS

TRACK_COUNT = 4

SAMPLE_RATE = 22050
BLOCK_SIZE = 220
NOTE_PITCH = [440 * pow(2, (note - 69) / 12) for note in range(128)]


class Track:
    def __init__(self):
        self._note = 0x45
        self._volume = 5000

        self._is_playing = False

        self._cur_note = 0

        self._sound = None

        self._speed = 1
        self._time = 0

    def next_data(self):
        if not self._is_playing:
            return 0

        sound = self._sound

        no = self._cur_note
        note = sound.note[no]

        if note:
            pitch = NOTE_PITCH[note]
            data = (INSTRUMENTS[sound.inst[no]](
                pitch / SAMPLE_RATE * self._time) * self._volume)
        else:
            data = 0

        self._time += 1

        if self._time % 3000 == 0:
            self._cur_note += 1
            if self._cur_note >= sound.length:
                self._is_playing = False
                self._time = 0

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
