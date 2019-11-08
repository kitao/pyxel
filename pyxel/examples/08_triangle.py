import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 110)
        pyxel.run(self.update, self.draw)

    def update(self):
        pass

    def draw(self):
        pyxel.cls(5)

        pyxel.tri(100, 0, 0, 100, 199, 100, 8)


App()
