from random import randint

import pyxel


class App:
    def __init__(self):
        pyxel.init(160, 120, caption='Pyxel Jump Game')

        pyxel.image(0).load(0, 0, 'assets/jump_game_160x120.png')

        self.score = 0
        self.player_x = 72
        self.player_y = -16
        self.player_vy = 0
        self.player_is_alive = True

        self.far_cloud = [(-10, 75), (40, 65), (90, 60)]
        self.near_cloud = [(10, 25), (70, 35), (120, 15)]
        self.floor = [(i * 60, randint(8, 104), True) for i in range(4)]
        self.fruit = [(i * 60, randint(0, 104), randint(0, 2), True)
                      for i in range(4)]

        # bgm
        a = 'c3e2g2c3 e2g2c3e2'
        b = 'c3d2g2c3 d2g2c3d2'
        pyxel.sound(0).set(a * 3 + b * 1, 't', '2', 'f', 30)

        a = 'g1c2d2e2 e2e2f2f2'
        b = 'e2e2e2c2 c2c2c2c2'
        c = 'g2g2g2d2 d2d2d2d2'
        pyxel.sound(1).set(a + b + a + c, 's', '4', 'nnnn vvnn vvff nvvf', 30)

        pyxel.sound(2).set('c1c1f0f0a0a0g0g0', 'p', '4', 'nf', 120)

        pyxel.play(0, 0, loop=True)
        pyxel.play(1, 1, loop=True)
        pyxel.play(2, 2, loop=True)

        # jump sound
        pyxel.sound(3).set('g1a#1d#2b2', 's', '7654', 's', 9)

        # eat sound
        pyxel.sound(4).set('g1c2f2a2c#2f#3', 'p', '6', 's', 4)

        # fall sound
        pyxel.sound(5).set('a3d#3a#2f#2d2b1g1d#1', 's', '77654321', 's', 10)

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.update_player()

        for i, v in enumerate(self.floor):
            self.floor[i] = self.update_floor(*v)

        for i, v in enumerate(self.fruit):
            self.fruit[i] = self.update_fruit(*v)

    def update_player(self):
        if pyxel.btn(pyxel.KEY_LEFT):
            self.player_x = max(self.player_x - 2, 0)

        if pyxel.btn(pyxel.KEY_RIGHT):
            self.player_x = min(self.player_x + 2, pyxel.width - 16)

        self.player_y += self.player_vy
        self.player_vy = min(self.player_vy + 1, 8)

        if self.player_y > pyxel.height:
            if self.player_is_alive:
                self.player_is_alive = False
                pyxel.play(3, 5)

            if self.player_y > 600:
                self.score = 0
                self.player_x = 72
                self.player_y = -16
                self.player_vy = 0
                self.player_is_alive = True

    def update_floor(self, x, y, is_active):
        if is_active:
            if (self.player_x + 16 >= x and self.player_x <= x + 40
                    and self.player_y + 16 >= y and self.player_y <= y + 8
                    and self.player_vy > 0):
                is_active = False
                self.score += 10
                self.player_vy = -12
                pyxel.play(3, 3)
        else:
            y += 6

        x -= 4

        if x < -40:
            x += 240
            y = randint(8, 104)
            is_active = True

        return (x, y, is_active)

    def update_fruit(self, x, y, kind, is_active):
        if (is_active and abs(x - self.player_x) < 12
                and abs(y - self.player_y) < 12):
            is_active = False
            self.score += (kind + 1) * 100
            self.player_vy = min(self.player_vy, -8)
            pyxel.play(3, 4)

        x -= 2

        if x < -40:
            x += 240
            y = randint(0, 104)
            kind = randint(0, 2)
            is_active = True

        return (x, y, kind, is_active)

    def draw(self):
        pyxel.cls(12)

        # draw sky
        pyxel.blt(0, 88, 0, 0, 88, 160, 32)

        # draw mountain
        pyxel.blt(0, 88, 0, 0, 64, 160, 24, 12)

        # draw forest
        offset = pyxel.frame_count % 160
        for i in range(2):
            pyxel.blt(i * 160 - offset, 104, 0, 0, 48, 160, 16, 12)

        # draw clouds
        offset = (pyxel.frame_count // 16) % 160
        for i in range(2):
            for x, y in self.far_cloud:
                pyxel.blt(x + i * 160 - offset, y, 0, 64, 32, 32, 8, 12)

        offset = (pyxel.frame_count // 8) % 160
        for i in range(2):
            for x, y in self.near_cloud:
                pyxel.blt(x + i * 160 - offset, y, 0, 0, 32, 56, 8, 12)

        # draw floors
        for x, y, is_active in self.floor:
            pyxel.blt(x, y, 0, 0, 16, 40, 8, 12)

        # draw fruits
        for x, y, kind, is_active in self.fruit:
            if is_active:
                pyxel.blt(x, y, 0, 32 + kind * 16, 0, 16, 16, 12)

        # draw player
        pyxel.blt(self.player_x, self.player_y, 0, 16
                  if self.player_vy > 0 else 0, 0, 16, 16, 12)

        # draw score
        s = 'SCORE {:>4}'.format(self.score)
        pyxel.text(5, 4, s, 1)
        pyxel.text(4, 4, s, 7)


App()
