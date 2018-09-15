import math
import random

import pyxel

BUBBLE_SPEED_MAX = 1.8
BUBBLE_COUNT = 50


class Vec2:
    x = -1.0
    y = -1.0


class Bubble:
    r = -1.0
    pos = Vec2()
    vel = Vec2()
    color = -1

    def rand(self):
        self.r = random.uniform(3.0, 10.0)
        self.pos = Vec2()
        self.pos.x = random.uniform(self.r, pyxel.width - self.r)
        self.pos.y = random.uniform(self.r, pyxel.height - self.r)
        self.vel = Vec2()
        self.vel.x = random.uniform(-BUBBLE_SPEED_MAX, BUBBLE_SPEED_MAX)
        self.vel.y = random.uniform(-BUBBLE_SPEED_MAX, BUBBLE_SPEED_MAX)
        self.color = random.randrange(1, 16)

    def update(self):
        self.pos.x += self.vel.x
        self.pos.y += self.vel.y

        if self.pos.x < self.r and self.vel.x < 0:
            self.vel.x *= -1
        if self.pos.x > pyxel.width - self.r and self.vel.x > 0:
            self.vel.x *= -1
        if self.pos.y < self.r and self.vel.y < 0:
            self.vel.y *= -1
        if self.pos.y > pyxel.height - self.r and self.vel.y > 0:
            self.vel.y *= -1


class App:
    bubbles = []
    first_exploded = False

    def __init__(self):
        pyxel.init(255, 255, caption="Pyxel Bubbles")

        for i in range(0, BUBBLE_COUNT):
            new_bubble = Bubble()
            new_bubble.rand()
            self.bubbles.append(new_bubble)

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        num_bubbles = len(self.bubbles)

        if pyxel.btnp(pyxel.KEY_LEFT_BUTTON):
            for i in range(0, num_bubbles):
                exploder = self.bubbles[i]
                dx = pyxel.mouse_x - exploder.pos.x
                dy = pyxel.mouse_y - exploder.pos.y
                d2 = dx * dx + dy * dy

                if d2 < exploder.r * exploder.r:
                    self.first_exploded = True
                    NUM_NEW_BUBBLES = 11
                    new_r = math.sqrt(exploder.r * exploder.r / NUM_NEW_BUBBLES)

                    for j in range(0, NUM_NEW_BUBBLES):
                        new_bubble = Bubble()
                        self.bubbles.append(new_bubble)
                        new_bubble.rand()
                        angle = 2 * math.pi * j / NUM_NEW_BUBBLES
                        new_bubble.r = new_r
                        new_bubble.pos.x = exploder.pos.x + (
                            exploder.r + new_r
                        ) * math.cos(angle)
                        new_bubble.pos.y = exploder.pos.y + (
                            exploder.r + new_r
                        ) * math.sin(angle)
                        new_bubble.vel.x = math.cos(angle) * BUBBLE_SPEED_MAX
                        new_bubble.vel.y = math.sin(angle) * BUBBLE_SPEED_MAX
                    del self.bubbles[i]
                    break

        for i in range(num_bubbles - 1, -1, -1):
            bi = self.bubbles[i]
            bi.update()

            for j in range(i - 1, -1, -1):
                bj = self.bubbles[j]
                total_r = bi.r + bj.r
                dx = bi.pos.x - bj.pos.x
                dy = bi.pos.y - bj.pos.y
                d2 = dx * dx + dy * dy

                if d2 < total_r * total_r:
                    new_bubble = Bubble()
                    new_bubble.rand()
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

        if (not self.first_exploded) and ((pyxel.frame_count // 10) % 2):
            pyxel.text(100, 50, "CLICK ON BUBBLES", pyxel.frame_count % 16)


App()
