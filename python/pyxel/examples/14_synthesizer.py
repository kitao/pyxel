import pyxel

EXTENDED_CHANNELS = [
    (0.1, 0),
    (0.1, -20),
    (0.02, 0),
    (0.02, 0),
    (0.02, 0),
    (0.02, 0),
    (0.1, 0),
    (0.1, 0),
]
# [(gain, detune), (gain, detune), ...]
# 'gain' ranges from 0.0 to 1.0.
# Ensure that the total gain during simultaneous playback does not exceed 1.0.
# 'detune' is the amount of detuning in cents (1/100 of a semitone).
# 'detune' needs to be adjusted according to the pitch of the notes.

EXTENDED_TONES = [
    (0.5, 0, [15] * 16 + [0] * 16),
    (0.5, 0, [15] * 16 + [0] * 16),
    (0.5, 0, [15] * 16 + [0] * 16),
    (0.6, 1, [0] * 32),
    (0.6, 2, [0] * 32),
]
# [(gain, noise, waveform), (gain, noise, waveform), ...]
# 'noise' corresponds to:
#  0 for noise disabled (use waveform), 1 for short-period noise, 2 for long-period noise.
# 'waveform' is composed of 32 values ranging from 0 to 15.

WAVEFORM_EDITOR_PARAMS = [
    (8, 8, 0, "Detuned Melody"),
    (8, 71, 1, "Chord Backing"),
    (8, 134, 2, "Bass Line"),
]


def extend_audio():
    channels = []
    for gain, detune in EXTENDED_CHANNELS:
        channel = pyxel.Channel()
        channel.gain = gain
        channel.detune = detune
        channels.append(channel)
    pyxel.channels.from_list(channels)

    tones = []
    for gain, noise, waveform in EXTENDED_TONES:
        tone = pyxel.Tone()
        tone.gain = gain
        tone.noise = noise
        tone.waveform.from_list(waveform)
        tones.append(tone)
    pyxel.tones.from_list(tones)


def setup_music():
    pyxel.sounds[5].set("g2g2 b-2f2 g2g2 e-2f2", "0", "5", "n", 60)

    pyxel.sounds[0].set("a-2a-2 g2g2 g2g2 g2b-2", "1", "5", "n", 60)
    pyxel.sounds[1].set("c3c3 b2b2 b-2b-2 b-2d-3", "1", "5", "n", 60)
    pyxel.sounds[2].set("e-3e-3 d3d3 c3c3 c3e-3", "1", "5", "n", 60)
    pyxel.sounds[3].set("g3g3 f3f3 e-3e-3 e-3g3", "1", "5", "n", 60)
    pyxel.sounds[4].set("a-0a-1a-0a-1 g0g1b0b1 c1c2c1c2 c1c2e-1e-2", "2", "5", "n", 30)
    pyxel.musics[0].set([5], [5], [0], [1], [2], [3], [4])


class WaveformEditor:
    def __init__(self, x, y, tone, desc):
        self.x = x
        self.y = y
        self.tone = tone
        self.desc = desc

    def update(self):
        if not pyxel.btn(pyxel.MOUSE_BUTTON_LEFT):
            return

        wx = (pyxel.mouse_x - self.x - 1) // 5
        wy = 15 - (pyxel.mouse_y - self.y - 8) // 3
        if 0 <= wx <= 31 and 0 <= wy <= 15:
            pyxel.tones[self.tone].waveform[wx] = wy

        gx = (pyxel.mouse_x - self.x - 168) // 5
        gy = 16 - (pyxel.mouse_y - self.y - 8) // 3
        if gx == 0 and 0 <= gy <= 16:
            pyxel.tones[self.tone].gain = gy / 16

    def draw(self):
        pyxel.text(self.x, self.y, f"TONE:{self.tone} {self.desc}", 12)

        self.draw_panel(self.x, self.y + 7, 162, 50)
        pyxel.line(self.x + 1, self.y + 32, self.x + 161, self.y + 32, 15)
        pyxel.line(self.x + 81, self.y + 8, self.x + 81, self.y + 56, 15)
        for i in range(32):
            amp = pyxel.tones[self.tone].waveform[i]
            for j in range(amp, 8) if amp < 8 else range(8, amp + 1):
                self.draw_rect(self.x + i * 5 + 2, self.y + 54 - j * 3)

        self.draw_panel(self.x + 167, self.y + 7, 7, 50)
        for i in range(int(pyxel.tones[self.tone].gain * 16)):
            self.draw_rect(self.x + 169, self.y + 54 - i * 3)

    @classmethod
    def draw_panel(cls, x, y, w, h):
        pyxel.rectb(x, y, w + 1, h + 1, 5)
        pyxel.rectb(x, y, w, h, 4)
        pyxel.rect(x + 1, y + 1, w - 1, h - 1, 9)

    def draw_rect(cls, x, y):
        pyxel.rect(x, y, 4, 2, 1)


class App:
    def __init__(self):
        pyxel.init(191, 200, title="Synthesizer")
        extend_audio()
        setup_music()
        self.waveform_editors = [
            WaveformEditor(*param) for param in WAVEFORM_EDITOR_PARAMS
        ]
        pyxel.mouse(True)
        pyxel.images[0].blt(0, 0, pyxel.cursor, 0, 0, 16, 16)
        pyxel.playm(0, loop=True)
        pyxel.run(self.update, self.draw)

    def update(self):
        for waveform_editor in self.waveform_editors:
            waveform_editor.update()

    def draw(self):
        pyxel.cls(1)
        for waveform_editor in self.waveform_editors:
            waveform_editor.draw()


App()
App()
App()
