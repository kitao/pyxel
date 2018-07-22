from random import randint
import pyxel


class App:
    def __init__(self):
        pyxel.init(160, 120, caption='Pyxel Jump Game')

        pyxel.image(0).load(0, 0, 'assets/jump_game_128x128.png')

        def circulate_list(l):
            return l + list(map(lambda pos: (pos[0] + 240, pos[1]), l))

        self.cloud1 = circulate_list(
            [(i * 80 + randint(-20, 20), randint(-10, 30)) for i in range(3)])
        self.cloud2 = circulate_list(
            [(i * 40 + randint(-15, 15), randint(10, 80)) for i in range(6)])
        self.hill1 = circulate_list(
            [(i * 60 + randint(-30, 30), randint(50, 80)) for i in range(4)])
        self.hill2 = circulate_list(
            [(i * 30 + randint(-10, 10), randint(70, 100)) for i in range(8)])

        self.frame = [(i * 60, randint(10, 110), 40, False) for i in range(4)]

        self.player_x = 10
        self.player_y = 30
        self.player_vy = 1
        self.score = 0

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btn(pyxel.KEY_LEFT):
            self.player_x -= 2

        if pyxel.btn(pyxel.KEY_RIGHT):
            self.player_x += 2

        self.player_y += self.player_vy
        self.player_vy = min(self.player_vy + 1, 8)

        if self.player_y > 104:
            self.player_vy = -12

        for i, (x, y, w, f) in enumerate(self.frame):
            if f:
                y += 6

            if (not f and self.player_x + 16 >= x and self.player_x <= x + w
                    and self.player_y + 16 >= y and self.player_y <= y + 8
                    and self.player_vy > 0):
                self.player_vy = -12
                f = True

            x -= 4

            if x < -40:
                x += 240
                y = randint(10, 110)
                f = False
            self.frame[i] = (x, y, w, f)

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

        pyxel.blt(self.player_x, self.player_y, 0, 16
                  if self.player_vy > 0 else 0, 0, 16, 16, 1)

        for x, y, w, f in self.frame:
            pyxel.blt(x, y, 0, 0, 16, w, 8, 1)

        pyxel.blt(100, 30, 0, 32, 0, 16, 16, 1)
        pyxel.blt(120, 40, 0, 48, 0, 16, 16, 1)
        pyxel.blt(120, 60, 0, 64, 0, 16, 16, 1)

        s = 'SCORE {:>4}'.format(self.score)
        pyxel.text(5, 4, s, 1)
        pyxel.text(4, 4, s, 7)


App()
