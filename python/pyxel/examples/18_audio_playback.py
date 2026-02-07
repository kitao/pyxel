import pyxel

TITLE_IMAGE_PATH = "assets/audio_title.png"
TITLE_AUDIO_PATH = "assets/audio_title.ogg"
PLAY_IMAGE_PATH = "assets/audio_play.png"
PLAY_AUDIO_PATH = "assets/audio_play.ogg"


class App:
    def __init__(self):
        pyxel.init(256, 240, title="Audio Playback")
        pyxel.integer_scale(True)

        self.images = [
            pyxel.Image.from_image(TITLE_IMAGE_PATH),
            pyxel.Image.from_image(PLAY_IMAGE_PATH),
        ]

        pyxel.sounds[0].pcm(TITLE_AUDIO_PATH)
        pyxel.sounds[1].pcm(PLAY_AUDIO_PATH)

        self.state = 0
        self.play_current()

        pyxel.run(self.update, self.draw)

    def play_current(self):
        pyxel.play(0, pyxel.sounds[self.state], loop=True)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if (
            pyxel.btnp(pyxel.KEY_RETURN)
            or pyxel.btnp(pyxel.GAMEPAD1_BUTTON_A)
            or pyxel.frame_count == 150
        ):
            self.state = 1 - self.state
            self.play_current()

        if pyxel.frame_count == 300:
            pyxel.screencast()
            pyxel.quit()

    def draw(self):
        pyxel.blt(
            0,
            0,
            self.images[self.state],
            0,
            0,
            pyxel.width,
            pyxel.height,
        )

        x = 26 - ((pyxel.frame_count // 2) % pyxel.width)
        s = "Pyxel Audio Playback Sample - Press Enter to Toggle"
        c = 15 if self.state == 0 else 8
        for i in range(2):
            pyxel.text(x + i * pyxel.width, 4, s, c)


App()
