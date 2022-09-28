import pyxel

SCREEN_WDITH = 197
SCREEN_HEIGHT = 98

CROSS_KEY_X = 12
CROSS_KEY_Y = 12
CROSS_KEY_WIDTH = 24
CROSS_KEY_LENGTH = 25

BUTTON_X = 104
BUTTON_Y = 5
BUTTON_WIDTH = 32
BUTTON_MARGIN = 24


def draw_gamepad(x, y, color):
    cx1 = CROSS_KEY_X + x
    cx2 = cx1 + CROSS_KEY_LENGTH + 1
    cx3 = cx2 + CROSS_KEY_WIDTH - 1
    cx4 = cx3 + CROSS_KEY_LENGTH - 1

    cy1 = CROSS_KEY_Y + y
    cy2 = cy1 + CROSS_KEY_LENGTH + 1
    cy3 = cy2 + CROSS_KEY_WIDTH - 1
    cy4 = cy3 + CROSS_KEY_LENGTH - 1

    pyxel.line(cx2 + 2, cy1, cx3 - 2, cy1, color)
    pyxel.line(cx2 + 2, cy4, cx3 - 2, cy4, color)
    pyxel.line(cx1, cy2 + 2, cx1, cy3 - 2, color)
    pyxel.line(cx4, cy2 + 2, cx4, cy3 - 2, color)

    pyxel.line(cx1 + 2, cy2, cx2, cy2, color)
    pyxel.line(cx3, cy2, cx4 - 2, cy2, color)
    pyxel.line(cx1 + 2, cy3, cx2, cy3, color)
    pyxel.line(cx3, cy3, cx4 - 2, cy3, color)

    pyxel.line(cx2, cy1 + 2, cx2, cy2, color)
    pyxel.line(cx2, cy3, cx2, cy4 - 2, color)
    pyxel.line(cx3, cy1 + 2, cx3, cy2, color)
    pyxel.line(cx3, cy3, cx3, cy4 - 2, color)

    pyxel.pset(cx2 + 1, cy1 + 1, color)
    pyxel.pset(cx3 - 1, cy1 + 1, color)
    pyxel.pset(cx2 + 1, cy4 - 1, color)
    pyxel.pset(cx3 - 1, cy4 - 1, color)
    pyxel.pset(cx1 + 1, cy2 + 1, color)
    pyxel.pset(cx1 + 1, cy3 - 1, color)
    pyxel.pset(cx4 - 1, cy2 + 1, color)
    pyxel.pset(cx4 - 1, cy3 - 1, color)

    bx1 = BUTTON_X + x
    bx2 = bx1 + BUTTON_WIDTH + BUTTON_MARGIN
    bx3 = bx1 + BUTTON_WIDTH / 2 + BUTTON_MARGIN / 2

    by1 = BUTTON_Y + y
    by2 = by1 + BUTTON_WIDTH + BUTTON_MARGIN
    by3 = by1 + BUTTON_WIDTH / 2 + BUTTON_MARGIN / 2

    pyxel.ellib(bx1, by3, BUTTON_WIDTH, BUTTON_WIDTH, color)
    pyxel.ellib(bx2, by3, BUTTON_WIDTH, BUTTON_WIDTH, color)
    pyxel.ellib(bx3, by1, BUTTON_WIDTH, BUTTON_WIDTH, color)
    pyxel.ellib(bx3, by2, BUTTON_WIDTH, BUTTON_WIDTH, color)


class App:
    def __init__(self):
        pyxel.init(SCREEN_WDITH, SCREEN_HEIGHT, capture_scale=1)
        self.show_guide = False
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_SPACE):
            self.show_guide = not self.show_guide

    def draw(self):
        pyxel.cls(0)

        for i in range(-1, 2):
            for j in range(-1, 2):
                draw_gamepad(j, i, 1)
        draw_gamepad(0, 0, 13)

        pyxel.line(98, 0, 98, pyxel.height - 1, 3)

        if self.show_guide:
            pyxel.rectb(0, 38, 12, 24, 8)
            pyxel.rectb(38, 0, 24, 12, 8)
            pyxel.rectb(86, 38, 12, 24, 8)
            pyxel.rectb(38, 86, 24, 12, 8)

            pyxel.rectb(99, 45, 5, 8, 8)
            pyxel.rectb(192, 45, 5, 8, 8)
            pyxel.rectb(144, 0, 8, 5, 8)
            pyxel.rectb(144, 93, 8, 5, 8)

            pyxel.rectb(99, 0, 33, 33, 8)
            pyxel.rectb(164, 0, 33, 33, 8)
            pyxel.rectb(99, 65, 33, 33, 8)
            pyxel.rectb(164, 65, 33, 33, 8)


App()
