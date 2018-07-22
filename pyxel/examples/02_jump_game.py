from random import randint
import pyxel


class App:
    def __init__(self):
        pyxel.init(160, 120, caption='Pyxel Jump Game')

        pyxel.image(0).load(0, 0, 'assets/jump_game_128x128.png')

        def loop_list(l):
            return l + list(map(lambda pos: (pos[0] + 240, pos[1]), l))

        self.cloud1 = loop_list(
            [(i * 80 + randint(-20, 20), randint(-10, 30)) for i in range(3)])
        self.cloud2 = loop_list([(i * 40 + randint(-15, 15), randint(10, 80))
                                 for i in range(6)] * 2)
        self.hill1 = loop_list([(i * 60 + randint(-30, 30), randint(50, 80))
                                for i in range(4)] * 2)
        self.hill2 = loop_list([(i * 30 + randint(-20, 20), randint(70, 100))
                                for i in range(8)] * 2)

        self.frame = [(i * 60, randint(0, 120)) for i in range(4)]

        self.score = 0

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        for i, (x, y) in enumerate(self.frame):
            x -= 3
            if x < -40:
                x += 240
            self.frame[i] = (x, y)

    def draw(self):
        pyxel.cls(0)

        for x, y in self.cloud2:
            x -= (pyxel.frame_count // 16) % 240
            pyxel.blt(x, y, 0, 96, 40, 28, 16, 1)

        for x, y in self.cloud1:
            x -= (pyxel.frame_count // 8) % 240
            pyxel.blt(x, y, 0, 40, 32, 52, 24, 1)

        for x, y in self.hill2:
            x -= (pyxel.frame_count // 4) % 240
            pyxel.blt(x, y, 0, 40, 77, 22, 51, 1)

        for x, y in self.hill1:
            x -= (pyxel.frame_count // 2) % 240
            pyxel.blt(x, y, 0, 0, 56, 33, 72, 1)

        for x, y in self.frame:
            pyxel.blt(x, y, 0, 0, 16, 40, 8, 1)

        pyxel.blt(30, 30, 0, 0, 0, 16, 16, 1)

        pyxel.blt(100, 30, 0, 32, 0, 16, 16, 1)
        pyxel.blt(120, 60, 0, 64, 0, 16, 16, 1)

        s = 'SCORE {:>4}'.format(self.score)
        pyxel.text(5, 4, s, 1)
        pyxel.text(4, 4, s, 7)


App()
