import math
import random

import pyxel

SCREEN_WIDTH = 256
SCREEN_HEIGHT = 256

MAX_BUBBLE_SPEED = 1.8
NUM_INITIAL_BUBBLES = 50
NUM_EXPLODE_BUBBLES = 11


class Vec2:
    def __init__(self, x, y):
        self.x = x
        self.y = y


class Bubble:
    def __init__(self):
        self.r = random.uniform(3, 10)

        self.pos = Vec2(
            random.uniform(self.r, SCREEN_WIDTH - self.r),
            random.uniform(self.r, SCREEN_HEIGHT - self.r),
        )

        self.vel = Vec2(
            random.uniform(-MAX_BUBBLE_SPEED, MAX_BUBBLE_SPEED),
            random.uniform(-MAX_BUBBLE_SPEED, MAX_BUBBLE_SPEED),
        )

        self.color = random.randint(1, 15)

    def update(self):
        self.pos.x += self.vel.x
        self.pos.y += self.vel.y

        if self.vel.x < 0 and self.pos.x < self.r:
            self.vel.x *= -1

        if self.vel.x > 0 and self.pos.x > SCREEN_WIDTH - self.r:
            self.vel.x *= -1

        if self.vel.y < 0 and self.pos.y < self.r:
            self.vel.y *= -1

        if self.vel.y > 0 and self.pos.y > SCREEN_HEIGHT - self.r:
            self.vel.y *= -1


class App:
    def __init__(self):
        pyxel.init(SCREEN_WIDTH, SCREEN_HEIGHT, title="Pyxel Bubbles", capture_scale=1)
        pyxel.mouse(True)

        self.is_exploded = False
        self.bubbles = [Bubble() for _ in range(NUM_INITIAL_BUBBLES)]

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        num_bubbles = len(self.bubbles)

        if pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT):
            for i in range(num_bubbles):
                bubble = self.bubbles[i]
                dx = bubble.pos.x - pyxel.mouse_x
                dy = bubble.pos.y - pyxel.mouse_y

                if dx * dx + dy * dy < bubble.r * bubble.r:
                    self.is_exploded = True
                    new_r = math.sqrt(bubble.r * bubble.r / NUM_EXPLODE_BUBBLES)

                    for j in range(NUM_EXPLODE_BUBBLES):
                        angle = math.pi * 2 * j / NUM_EXPLODE_BUBBLES

                        new_bubble = Bubble()
                        new_bubble.r = new_r
                        new_bubble.pos.x = bubble.pos.x + (bubble.r + new_r) * math.cos(
                            angle
                        )
                        new_bubble.pos.y = bubble.pos.y + (bubble.r + new_r) * math.sin(
                            angle
                        )
                        new_bubble.vel.x = math.cos(angle) * MAX_BUBBLE_SPEED
                        new_bubble.vel.y = math.sin(angle) * MAX_BUBBLE_SPEED
                        self.bubbles.append(new_bubble)

                    del self.bubbles[i]
                    break

        for i in range(num_bubbles - 1, -1, -1):
            bi = self.bubbles[i]
            bi.update()

            for j in range(i - 1, -1, -1):
                bj = self.bubbles[j]
                dx = bi.pos.x - bj.pos.x
                dy = bi.pos.y - bj.pos.y
                total_r = bi.r + bj.r

                if dx * dx + dy * dy < total_r * total_r:
                    new_bubble = Bubble()
                    new_bubble.r = math.sqrt(bi.r * bi.r + bj.r * bj.r)
                    new_bubble.pos.x = (bi.pos.x * bi.r + bj.pos.x * bj.r) / total_r
                    new_bubble.pos.y = (bi.pos.y * bi.r + bj.pos.y * bj.r) / total_r
                    new_bubble.vel.x = (bi.vel.x * bi.r + bj.vel.x * bj.r) / total_r
                    new_bubble.vel.y = (bi.vel.y * bi.r + bj.vel.y * bj.r) / total_r
                    self.bubbles.append(new_bubble)

                    del self.bubbles[i]
                    del self.bubbles[j]
                    num_bubbles -= 1
                    break

    def draw(self):
        pyxel.cls(0)

        for bubble in self.bubbles:
            pyxel.circ(bubble.pos.x, bubble.pos.y, bubble.r, bubble.color)

        if not self.is_exploded and pyxel.frame_count % 20 < 10:
            pyxel.text(96, 50, "CLICK ON BUBBLE", pyxel.frame_count % 15 + 1)


App()
