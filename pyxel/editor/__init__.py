import pyxel


class Editor(pyxel.App):
    def __init__(self):
        super().__init__(160, 120, 4)

        self.image = pyxel.Image(16, 16)

        data = self.image.data
        data[0, 0] = 7
        data[0, 1] = 3
        data[0, 2] = 7
        data[1, 0] = 8
        data[2, 0] = 7
        data[7, 7] = 7
        self.bank(0, self.image)

        self.x = 0

        self.count = 0
        self.time = 0

    def update(self):
        self.cls(2)

        self.pix(0, 0, 8)
        self.pix(159, 0, 8)
        self.pix(0, 119, 8)
        self.pix(159, 119, 8)

        self.blt(80, 40, 7, 0, 0, 64, 64, 0)

        self.rectb(50, 50, 30, 40, 3)

        self.rectb(90, 90, 21, 21, 8)
        self.circb(100, 100, 30, 7)
        self.circ(30, 100, 4, 7)

        self.rect(30, 30, 30, 10, 0)
        self.line(29, 29, 60, 20, 8)

        self.x = (self.x + 1) % 160
        self.pix(self.x, 0, 7)

        self.text(11, 11, "Hello, Pyxel!", 8)
        self.text(10, 10, "Hello, Pyxel!", 7)


def run():
    Editor().run()
