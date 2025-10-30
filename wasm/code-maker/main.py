import pyxel


class App:
    def __init__(self):
        pyxel.init(160, 120, title="Hello Pyxel")
        pyxel.load("my_resource.pyxres")
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        pyxel.cls(0)
        pyxel.blt(10, 10, 0, 0, 0, 16, 16)
        pyxel.text(55, 41, "Hello, Pyxel!", pyxel.frame_count % 16)


App()
