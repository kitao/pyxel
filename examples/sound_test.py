import pyxel


class App:
    def __init__(self):
        pyxel.init(128, 128)

        self.sound = pyxel.Sound(1, ['c5...d5e5f5g5', '0', '7', '7'])

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

    def draw(self):
        pyxel.cls(4)


App()
