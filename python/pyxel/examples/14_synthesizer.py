import pyxel

EXTENDED_CHANNELS = [
    (0.1 / 2.0, 0),  # Lead Melody
    (0.1 / 2.0, 10),  # Detuned Lead Melody
    (0.1, 0),  # Sub Melody
    (0.1 / 3.0, 0),  # Chord Backing 1
    (0.1 / 3.0, 0),  # Chord Backing 2
    (0.1 / 3.0, 0),  # Chord Backing 3
    (0.1, 0),  # Bass Line
    (0.1, 0),  # Drums
]
# [(gain, detune), (gain, detune), ...]
# 'gain' ranges from 0.0 to 1.0.
# Ensure that the total gain during simultaneous playback does not exceed 1.0.
# 'detune' is the amount of detuning in cents (1/100 of a semitone).
# 'detune' must be set carefully according to the pitch of the notes.

EXTENDED_TONES = [
    (  # Sine Wave
        0,
        4,
        [15, 15, 15, 15, 15, 15, 15, 15, 15, 14, 13, 12, 11, 10, 9, 8]
        + [7, 6, 5, 4, 3, 2, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        0.8,
    ),
    (  # Sine Wave
        0,
        4,
        [8, 9, 10, 12, 13, 14, 14, 15, 15, 15, 14, 14, 13, 12, 10, 9]
        + [8, 6, 5, 3, 2, 1, 1, 0, 0, 0, 1, 1, 2, 3, 5, 6],
        0.4,
    ),
    (  # Narrow (1:7) Pulse Wave
        0,
        4,
        [15] * 4 + [0] * 28,
        0.7,
    ),
    (  # Saw Wave
        0,
        4,
        [15, 15, 14, 14, 13, 13, 12, 12, 11, 11, 10, 10, 9, 9, 8, 8]
        + [7, 7, 6, 6, 5, 5, 4, 4, 3, 3, 2, 2, 1, 1, 0, 0],
        1.0,
    ),
    (  # Short Period Noise
        1,
        4,
        [0] * 32,
        0.8,
    ),
]
# [(mode, sample_bits, wavetable, gain), (mode, sample_bits, wavetable, gain), ...]
# 'mode' corresponds to:
#  0 for wavetable, 1 for short-period noise, 2 for long-period noise.
# 'wavetable' can be any length, but all are 32 elements in this example.
# 'wavetable' value range depends on 'sample_bits'. For 4 bits, the range is 0-15.

WAVETABLE_EDITOR_PARAMS = [
    (8, 8, 0, "Lead Melody"),
    (8, 71, 1, "Sub Melody"),
    (8, 134, 2, "Chord Backing"),
    (8, 197, 3, "Bass Line"),
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
    for mode, sample_bits, wavetable, gain in EXTENDED_TONES:
        tone = pyxel.Tone()
        tone.mode = mode
        tone.sample_bits = sample_bits
        tone.wavetable.from_list(wavetable)
        tone.gain = gain
        tones.append(tone)
    pyxel.tones.from_list(tones)


def setup_music():
    pyxel.sounds[0].set(
        "b-2b-2b-2b-2a-2a-2a-2a-2 g2g2e-2e-2c2c2f2f2 f2f2g2g2f2f2e2e2 e2e2c2c2c2c2rr",
        "0",
        "5",
        "vvvfnnnf nfnfnfvv vfnfnfvv vfvvvfvv",
        16,
    )
    pyxel.sounds[1].set(
        "rrb-1b-1c2c2e-2e-2 f2f2f2f2e-2e-2f2f2 g2g2b-2b-2c3c3f2f2 f2f2e-2e-2e-2e-2f2f2",
        "0",
        "5",
        "vfnfnfvf vfnfnfnf nfnfnfvv vfnnnfnf",
        16,
    )

    pyxel.sounds[2].set(
        "rrc3c3e-3e-3g3g3 f3f3f3g3g3g3g3g3 rrb-3b-3a-3a-3f3f3 a-3a-3a-3g3g3g3g3g3",
        "1",
        "3",
        "vvvvvvvv",
        32,
    )
    pyxel.sounds[3].set(
        "rrc3c3e-3e-3g3g3 f3f3f3g3g3g3g3g3 rrb-3b-3a-3a-3f3f3 a-3a-3a-3g3g3g3g3g3",
        "1",
        "3",
        "vvvvvvvf",
        32,
    )

    pyxel.sounds[4].set("a-2a-2ra-2 a-2a-2ra-2", "2", "5", "f", 32)
    pyxel.sounds[5].set("g2g2rg2 g2g2rg2", "2", "5", "f", 32)

    pyxel.sounds[6].set("c3c3rc3 b-2b-2rb-2", "2", "5", "f", 32)
    pyxel.sounds[7].set("b-2b-2rb-2 c3c3rc3", "2", "5", "f", 32)

    pyxel.sounds[8].set(
        "e-3e-3re-3 d3d3rd3 d3d3rd3 e3e3re3",
        "2",
        "5",
        "f",
        32,
    )
    pyxel.sounds[9].set(
        "e-3e-3re-3 d3d3rd3 d3d3rd3 e-3e-3re-3",
        "2",
        "5",
        "f",
        32,
    )

    pyxel.sounds[10].set("a-0rra-0 b-0rrb-0", "3", "5", "f", 32)
    pyxel.sounds[11].set("g0rrg0 c1rrc1", "3", "5", "f", 32)

    pyxel.sounds[12].set("g1rrrd2rrr" * 2, "4", "50006000" * 3 + "50506000", "f", 16)
    pyxel.sounds[13].set(
        "g1rrrd2rrr g1rd2rd2rrr", "4", "50006000" * 3 + "50506000", "f", 16
    )

    pyxel.musics[0].set(
        [0, 1], [0, 1], [2, 3], [4, 5], [6, 7], [8, 9], [10, 11], [12, 13]
    )


class WavetableEditor:
    def __init__(self, x, y, tone, desc):
        self.x = x
        self.y = y
        self.tone = tone
        self.desc = desc
        self.target = None
        self.last_col = 0

    def update(self):
        col = (pyxel.mouse_x - self.x - 1) // 5
        row = 15 - (pyxel.mouse_y - self.y - 8) // 3

        if pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT) and 0 <= row <= 15:
            if 0 <= col <= 31:
                self.target = "wave"
            elif 167 <= pyxel.mouse_x - self.x <= 174:
                self.target = "gain"

        if pyxel.btnr(pyxel.MOUSE_BUTTON_LEFT):
            self.target = None

        if self.target == "wave":
            for x in range(min(self.last_col, col), max(self.last_col, col) + 1):
                pyxel.tones[self.tone].wavetable[max(0, min(31, x))] = max(
                    0, min(15, row)
                )
        elif self.target == "gain":
            pyxel.tones[self.tone].gain = max(0, min(15, row)) / 15

        self.last_col = col

    def draw(self):
        pyxel.text(self.x, self.y, f"TONE:{self.tone} {self.desc}", 12)

        self.draw_panel(self.x, self.y + 7, 162, 50)
        pyxel.line(self.x + 1, self.y + 32, self.x + 161, self.y + 32, 15)
        pyxel.line(self.x + 81, self.y + 8, self.x + 81, self.y + 56, 15)

        for i in range(32):
            amp = pyxel.tones[self.tone].wavetable[i]
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
        pyxel.init(191, 264, title="Synthesizer")
        extend_audio()
        setup_music()

        self.wavetable_editors = [
            WavetableEditor(*param) for param in WAVETABLE_EDITOR_PARAMS
        ]

        pyxel.mouse(True)
        pyxel.playm(0, loop=True)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        for wavetable_editor in self.wavetable_editors:
            wavetable_editor.update()

    def draw(self):
        pyxel.cls(1)

        for wavetable_editor in self.wavetable_editors:
            wavetable_editor.draw()


App()
