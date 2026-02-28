# title: Pyxel Bubbles
# author: ttrkaya
# desc: A Pyxel click game example
# site: https://github.com/kitao/pyxel
# license: MIT
# version: 1.0

import pyxel

MAX_SPEED = 1.8
NUM_INITIAL_BUBBLES = 50
NUM_EXPLODE_BUBBLES = 11


class Bubble:
    def __init__(self, x, y, r, vx, vy):
        self.x = x
        self.y = y
        self.r = r
        self.vx = vx
        self.vy = vy
        self.color = pyxel.rndi(1, 15)

    def update(self):
        self.x += self.vx
        self.y += self.vy

        if self.vx < 0 and self.x < self.r:
            self.vx *= -1
        if self.vx > 0 and self.x > pyxel.width - self.r:
            self.vx *= -1
        if self.vy < 0 and self.y < self.r:
            self.vy *= -1
        if self.vy > 0 and self.y > pyxel.height - self.r:
            self.vy *= -1


def random_bubble():
    r = pyxel.rndf(3, 10)
    return Bubble(
        pyxel.rndf(r, pyxel.width - r),
        pyxel.rndf(r, pyxel.height - r),
        r,
        pyxel.rndf(-MAX_SPEED, MAX_SPEED),
        pyxel.rndf(-MAX_SPEED, MAX_SPEED),
    )


class App:
    def __init__(self):
        pyxel.init(256, 256, title="Pyxel Bubbles", capture_scale=1)
        pyxel.mouse(True)

        self.is_exploded = False
        self.bubbles = [random_bubble() for _ in range(NUM_INITIAL_BUBBLES)]

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        num_bubbles = len(self.bubbles)

        # Explode clicked bubble
        if pyxel.btnp(pyxel.MOUSE_BUTTON_LEFT):
            for i in range(num_bubbles):
                b = self.bubbles[i]
                dx = b.x - pyxel.mouse_x
                dy = b.y - pyxel.mouse_y

                if dx * dx + dy * dy < b.r * b.r:
                    self.is_exploded = True
                    new_r = pyxel.sqrt(b.r * b.r / NUM_EXPLODE_BUBBLES)

                    for j in range(NUM_EXPLODE_BUBBLES):
                        angle = 360 * j / NUM_EXPLODE_BUBBLES
                        self.bubbles.append(
                            Bubble(
                                b.x + (b.r + new_r) * pyxel.cos(angle),
                                b.y + (b.r + new_r) * pyxel.sin(angle),
                                new_r,
                                pyxel.cos(angle) * MAX_SPEED,
                                pyxel.sin(angle) * MAX_SPEED,
                            )
                        )

                    del self.bubbles[i]
                    break

        # Update and merge bubbles
        for i in range(num_bubbles - 1, -1, -1):
            bi = self.bubbles[i]
            bi.update()

            for j in range(i - 1, -1, -1):
                bj = self.bubbles[j]
                dx = bi.x - bj.x
                dy = bi.y - bj.y
                total_r = bi.r + bj.r

                if dx * dx + dy * dy < total_r * total_r:
                    self.bubbles.append(
                        Bubble(
                            (bi.x * bi.r + bj.x * bj.r) / total_r,
                            (bi.y * bi.r + bj.y * bj.r) / total_r,
                            pyxel.sqrt(bi.r * bi.r + bj.r * bj.r),
                            (bi.vx * bi.r + bj.vx * bj.r) / total_r,
                            (bi.vy * bi.r + bj.vy * bj.r) / total_r,
                        )
                    )

                    del self.bubbles[i]
                    del self.bubbles[j]
                    num_bubbles -= 1
                    break

    def draw(self):
        pyxel.cls(0)

        for b in self.bubbles:
            pyxel.circ(b.x, b.y, b.r, b.color)

        if not self.is_exploded and pyxel.frame_count % 20 < 10:
            pyxel.text(96, 50, "CLICK ON BUBBLE", pyxel.frame_count % 15 + 1)


App()
