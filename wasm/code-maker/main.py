import pyxel


class App:
    def __init__(self):
        pyxel.init(160, 120)
        pyxel.load("my_resource.pyxres")
        self.x = 0
        pyxel.run(self.update, self.draw)

    def update(self):
        self.x += 1
        if self.x >= pyxel.width:
            self.x = -8

    def draw(self):
        pyxel.cls(0)
        pyxel.blt(self.x, 60, 0, 0, 0, 8, 8, 2)
        pyxel.text(48, 53, "Pyxel Code Maker", pyxel.rndi(1, 15))


App()
