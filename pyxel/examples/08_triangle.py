import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 150)
        pyxel.run(self.update, self.draw)

    def update(self):
        pass

    def draw(self):
        pyxel.cls(5)

        pyxel.text(6, 6, "tri(x1,y1,x2,y2,x3,y3,col)", 7)
        pyxel.text(6, 20, "trib(x1,y1,x2,y2,x3,y3,col)", 7)

        pyxel.tri(150, 0, 0, 100, 180, 100, 8)
        pyxel.trib(150, 100, 0, 140, 180, 140, 3)


App()
