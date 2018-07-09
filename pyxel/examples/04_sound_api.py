import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 150)

        self.sound = pyxel.Sound.fromstring(
            ['c2c2g2g2a2a2g2.', 'p', '7', 'ffffffvf ffffffvf'], 60)

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_1):
            pyxel.play(0, self.sound, loop=True)

        if pyxel.btnp(pyxel.KEY_2):
            pyxel.play(1, [self.sound, self.sound], loop=True)

        if pyxel.btnp(pyxel.KEY_3):
            pyxel.play(2, [self.sound, self.sound])

        if pyxel.btnp(pyxel.KEY_4):
            pyxel.play(3, self.sound)

        if pyxel.btnp(pyxel.KEY_5):
            pyxel.stop(0)
            pyxel.stop(1)
            pyxel.stop(2)
            pyxel.stop(3)

    def draw(self):
        pyxel.cls(4)


App()
