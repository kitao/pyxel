import pyxel


class App:
    def __init__(self):
        pyxel.init(128, 128)
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_1):
            pyxel.play(0, None)

        if pyxel.btnp(pyxel.KEY_2):
            pyxel.play(1, None)

        if pyxel.btnp(pyxel.KEY_3):
            pyxel.play(2, None)

        if pyxel.btnp(pyxel.KEY_4):
            pyxel.play(3, None)

    def draw(self):
        pyxel.cls(4)


App()
