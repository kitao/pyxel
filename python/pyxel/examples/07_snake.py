# title: Snake!
# author: Marcus Croucher
# desc: A Pyxel snake game example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0

from collections import deque

import pyxel

SCREEN_W = 40
SCREEN_H = 50
SCORE_H = pyxel.FONT_HEIGHT

UP = (0, -1)
DOWN = (0, 1)
LEFT = (-1, 0)
RIGHT = (1, 0)


class App:
    def __init__(self):
        pyxel.init(
            SCREEN_W, SCREEN_H, title="Snake!", fps=20, display_scale=12, capture_scale=6
        )
        self.init_sound()
        self.reset()
        pyxel.run(self.update, self.draw)

    def init_sound(self):
        pyxel.sounds[0].set(
            notes="c3e3g3c4c4", tones="s", volumes="4", effects="nnnnf", speed=7
        )
        pyxel.sounds[1].set(
            notes="f3 b2 f2 b1  f1 f1 f1 f1",
            tones="p",
            volumes="44444321",
            effects="nnnnnnnf",
            speed=9,
        )

        melody1 = (
            "c3 c3 c3 d3 e3 r e3 r" "rrrrrrrr"
            "e3 e3 e3 f3 d3 r c3 r" "rrrrrrrr"
            "c3 c3 c3 d3 e3 r e3 r" "rrrrrrrr"
            "b2 b2 b2 f3 d3 r c3 r" "rrrrrrrr"
        )
        melody2 = (
            "rrrr e3e3e3e3 d3d3c3c3 b2b2c3c3"
            "a2a2a2a2 c3c3c3c3 d3d3d3d3 e3e3e3e3"
            "rrrr e3e3e3e3 d3d3c3c3 b2b2c3c3"
            "a2a2a2a2 g2g2g2g2 c3c3c3c3 g2g2a2a2"
            "rrrr e3e3e3e3 d3d3c3c3 b2b2c3c3"
            "a2a2a2a2 c3c3c3c3 d3d3d3d3 e3e3e3e3"
            "f3f3f3a3 a3a3a3a3 g3g3g3b3 b3b3b3b3"
            "b3b3b3b4 rrrr e3d3c3g3 a2g2e2d2"
        )
        pyxel.sounds[2].set(
            notes=melody1 * 2 + melody2 * 2,
            tones="s",
            volumes="3",
            effects="nnnsffff",
            speed=20,
        )

        harmony1 = (
            "a1 a1 a1 b1  f1 f1 c2 c2  c2 c2 c2 c2  g1 g1 b1 b1" * 3
            + "f1 f1 f1 f1 f1 f1 f1 f1 g1 g1 g1 g1 g1 g1 g1 g1"
        )
        harmony2 = (
            ("f1" * 8 + "g1" * 8 + "a1" * 8 + "c2" * 7 + "d2") * 3
            + "f1" * 16
            + "g1" * 16
        )
        pyxel.sounds[3].set(
            notes=harmony1 * 2 + harmony2 * 2,
            tones="t",
            volumes="5",
            effects="f",
            speed=20,
        )
        pyxel.sounds[4].set(
            notes="f0 r a4 r  f0 f0 a4 r  f0 r a4 r  f0 f0 a4 f0",
            tones="n",
            volumes="6622 6622 6622 6426",
            effects="f",
            speed=20,
        )

        pyxel.musics[0].set([], [2], [3], [4])

    def reset(self):
        self.direction = RIGHT
        self.snake = deque([(5, 5 + SCORE_H)])
        self.death = False
        self.score = 0
        self.generate_apple()
        pyxel.playm(0, loop=True)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_R) or pyxel.btnp(pyxel.GAMEPAD1_BUTTON_START):
            self.reset()

        if not self.death:
            self.update_direction()
            self.update_snake()
            self.check_death()
            self.check_apple()

    def update_direction(self):
        if pyxel.btn(pyxel.KEY_UP) or pyxel.btn(pyxel.GAMEPAD1_BUTTON_DPAD_UP):
            if self.direction != DOWN:
                self.direction = UP
        elif pyxel.btn(pyxel.KEY_DOWN) or pyxel.btn(pyxel.GAMEPAD1_BUTTON_DPAD_DOWN):
            if self.direction != UP:
                self.direction = DOWN
        elif pyxel.btn(pyxel.KEY_LEFT) or pyxel.btn(pyxel.GAMEPAD1_BUTTON_DPAD_LEFT):
            if self.direction != RIGHT:
                self.direction = LEFT
        elif pyxel.btn(pyxel.KEY_RIGHT) or pyxel.btn(pyxel.GAMEPAD1_BUTTON_DPAD_RIGHT):
            if self.direction != LEFT:
                self.direction = RIGHT

    def update_snake(self):
        hx, hy = self.snake[0]
        dx, dy = self.direction
        self.snake.appendleft((hx + dx, hy + dy))
        self.popped = self.snake.pop()

    def check_apple(self):
        if self.snake[0] == self.apple:
            self.score += 1
            self.snake.append(self.popped)
            self.generate_apple()
            pyxel.play(0, 0)

    def generate_apple(self):
        snake_set = set(self.snake)
        self.apple = self.snake[0]
        while self.apple in snake_set:
            self.apple = (
                pyxel.rndi(0, SCREEN_W - 1),
                pyxel.rndi(SCORE_H + 1, SCREEN_H - 1),
            )

    def check_death(self):
        hx, hy = self.snake[0]
        if hx < 0 or hy < SCORE_H or hx >= SCREEN_W or hy >= SCREEN_H:
            self.die()
        elif len(self.snake) != len(set(self.snake)):
            self.die()

    def die(self):
        self.death = True
        pyxel.stop()
        pyxel.play(0, 1)

    def draw(self):
        if self.death:
            self.draw_death()
            return

        pyxel.cls(3)

        # Draw snake
        for i, (x, y) in enumerate(self.snake):
            pyxel.pset(x, y, 7 if i == 0 else 11)

        # Draw apple
        pyxel.pset(*self.apple, 8)

        # Draw score
        pyxel.rect(0, 0, SCREEN_W, SCORE_H, 5)
        pyxel.text(1, 1, f"{self.score:04}", 6)

    def draw_death(self):
        pyxel.cls(8)
        for i, text in enumerate(["GAME OVER", f"{self.score:04}", "(Q)UIT", "(R)ESTART"]):
            x = (SCREEN_W - len(text) * pyxel.FONT_WIDTH) // 2
            pyxel.text(x, 5 + (pyxel.FONT_HEIGHT + 2) * i, text, 0)


App()
