import pyxel


class App:
    def __init__(self):
        pyxel.init(128, 128)

        self.sound = pyxel.Sound(60, ['c2.d2.', 't', '7', 'n'])

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_1):
            pyxel.play(0, self.sound)

        if pyxel.btnp(pyxel.KEY_2):
            pyxel.play(1, self.sound)

        if pyxel.btnp(pyxel.KEY_3):
            pyxel.play(2, self.sound)

        if pyxel.btnp(pyxel.KEY_4):
            pyxel.play(3, self.sound)

        if pyxel.btnp(pyxel.KEY_5):
            pyxel.stop(0)
            pyxel.stop(1)
            pyxel.stop(2)
            pyxel.stop(3)

    def draw(self):
        pyxel.cls(4)

        for i in range(128):
            x = (i * 2 / 128 + 0.25) % 1
            y = (abs(x * 4 - 2) - 1) * 0.7 * 40
            pyxel.pix(i, y + 64, 7)

        pyxel.line(0, 64, 128, 64, 7)


App()
