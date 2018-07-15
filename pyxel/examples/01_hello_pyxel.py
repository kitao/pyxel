import pyxel


class App:
    def __init__(self):
        pyxel.init(128, 128, caption='Hello Pyxel')
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        pyxel.cls(0)
        pyxel.text(40, 48, 'Hello, Pyxel!', pyxel.frame_count % 16)
        pyxel.logo(46, 70)


App()
