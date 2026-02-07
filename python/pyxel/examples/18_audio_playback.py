import pyxel

BGM1_IMAGE_PATH = "assets/audio_title.png"
BGM1_AUDIO_PATH = "assets/audio_bgm1.ogg"
BGM2_IMAGE_PATH = "assets/audio_play.png"
BGM2_AUDIO_PATH = "assets/audio_bgm2.ogg"


class App:
    def __init__(self):
        pyxel.init(256, 240, title="Audio Playback")
        pyxel.integer_scale(True)

        self.images = [
            pyxel.Image.from_image(BGM1_IMAGE_PATH),
            pyxel.Image.from_image(BGM2_IMAGE_PATH),
        ]

        pyxel.sounds[0].pcm(BGM1_AUDIO_PATH)
        pyxel.sounds[1].pcm(BGM2_AUDIO_PATH)

        # To avoid quality loss, use audio files pre-converted to 22.05kHz.
        # Change gain from the default 0.125 to adjust volume.
        pyxel.channels[0].gain = 0.8

        self.bgm_index = 0
        pyxel.play(0, 0, loop=True)

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_RETURN) or pyxel.btnp(pyxel.GAMEPAD1_BUTTON_A):
            self.bgm_index = 1 - self.bgm_index
            pyxel.play(0, self.bgm_index, loop=True)

    def draw(self):
        pyxel.blt(
            0,
            0,
            self.images[self.bgm_index],
            0,
            0,
            pyxel.width,
            pyxel.height,
        )

        x = 26 - ((pyxel.frame_count // 2) % pyxel.width)
        s = "Pyxel Audio Playback Sample - Press Enter to Toggle"
        c = 15 if self.bgm_index == 0 else 8
        for i in range(2):
            pyxel.text(x + i * pyxel.width, 4, s, c)


App()
