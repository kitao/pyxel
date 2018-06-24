import pyxel


class App(pyxel.App):
    def __init__(self):
        super().__init__(160, 120)

        self.image = pyxel.Image(16, 16)
        image_data = [
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 7, 0, 0, 7, 0, 0],
            [0, 0, 7, 0, 0, 7, 0, 0],
            [0, 0, 7, 0, 0, 7, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 7, 0, 0, 0, 0, 7, 0],
            [0, 0, 7, 7, 7, 7, 0, 0],
        ]
        self.image.set(0, 0, 8, 8, image_data)
        self.bank(0, self.image)

        self.x = 0
        self.press_a = False

    def update(self):
        self.x = (self.x + 1) % 160

        if self.btnp(pyxel.KEY_Q):
            exit()

        self.press_a = self.btnp(pyxel.KEY_A, 30, 15)

    def draw(self):
        self.cls(2)

        self.text(11, 10, 'Hello, Pyxel!', 8)
        self.text(10, 10, 'Hello, Pyxel!', 7)

        self.text(80, 0, '{},{}'.format(self.mouse_x, self.mouse_y), 7)

        self.pix(0, 0, 8)
        self.pix(159, 0, 8)
        self.pix(0, 119, 8)
        self.pix(159, 119, 8)

        self.blt(80, 40, 7, 0, 0, 64, 64, 0)
        self.blt(30, 60, 0, 0, 0, 8, 8)

        self.rectb(50, 50, 79, 89, 3)

        self.rectb(90, 90, 110, 110, 8)
        self.circb(100, 100, 30, 7)
        self.circ(30, 100, 4, 7)

        self.rect(30, 30, 59, 39, 0)
        self.line(29, 29, 60, 20, 8)

        self.pix(self.x, 0, 7)

        if self.press_a:
            self.rect(10, 10, 19, 19, 8)


App().run()
