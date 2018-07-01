import pyxel


class App():
    def __init__(self):
        pyxel.init(160, 120, caption='Simple App')

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        pyxel.cls(2)


App()
