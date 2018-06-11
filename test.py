import pyxel
import time


class App(pyxel.App):
    def __init__(self):
        super().__init__(160, 120, 4)

        data = self.image(0).data
        data[0, 0] = 7
        data[0, 1] = 3
        data[0, 2] = 7
        data[1, 0] = 8
        data[2, 0] = 7
        data[7, 7] = 7

        self.x = 0

        self.count = 0
        self.time = 0

    def update(self):
        start = time.time()

        self.cls(1)

        # self.pal(7, 0)

        self.blt(80, 80, 0, 0, 0, 50, 50, 7)

        self.rectb(50, 50, 30, 40, 3)

        self.rectb(90, 90, 21, 21, 8)
        self.circb(100, 100, 30, 7)
        self.circ(30, 100, 4, 7)

        self.rect(30, 30, 30, 50, 0)
        self.line(29, 29, 20, 20, 8)

        self.x = (self.x + 1) % 160
        self.pix(self.x, 0, 7)

        self.text(11, 11, "Hello, I'm Gobliso. Nice to meet you!", 8)
        self.text(10, 10, "Hello, I'm Gobliso. Nice to meet you!", 7)

        self.time += time.time() - start
        self.count += 1
        if self.count == 50:
            print(self.time / self.count * 1000)
            self.time = 0
            self.count = 0

    def key_press(self, key, modifiers):
        if key == pyxel.key.ESCAPE or key == pyxel.key.Q:
            exit()


App().run()
