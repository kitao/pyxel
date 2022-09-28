import pyxel

SCREEN_WDITH = 282
SCREEN_HEIGHT = 98

CROSS_KEY_X = 12
CROSS_KEY_Y = 12
CROSS_KEY_WIDTH = 24
CROSS_KEY_LENGTH = 25

SYSTEM_BUTTON_X = 111
SYSTEM_BUTTON_Y = 74
SYSTEM_BUTTON_WIDTH = 24
SYSTEM_BUTTON_HEIGHT = 12
SYSTEM_BUTTON_MARGIN = 12

BUTTON_X = 185
BUTTON_Y = 1
BUTTON_WIDTH = 34
BUTTON_MARGIN = 28


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

    sx1 = SYSTEM_BUTTON_X + x
    sx2 = sx1 + SYSTEM_BUTTON_WIDTH - 1
    sx3 = sx1 + SYSTEM_BUTTON_WIDTH + SYSTEM_BUTTON_MARGIN
    sx4 = sx3 + SYSTEM_BUTTON_WIDTH - 1

    sy1 = SYSTEM_BUTTON_Y + y
    sy2 = sy1 + SYSTEM_BUTTON_HEIGHT - 1

    pyxel.line(sx1 + 2, sy1, sx2 - 2, sy1, color)
    pyxel.line(sx1 + 2, sy2, sx2 - 2, sy2, color)
    pyxel.line(sx1, sy1 + 2, sx1, sy2 - 2, color)
    pyxel.line(sx2, sy1 + 2, sx2, sy2 - 2, color)
    pyxel.pset(sx1 + 1, sy1 + 1, color)
    pyxel.pset(sx2 - 1, sy1 + 1, color)
    pyxel.pset(sx1 + 1, sy2 - 1, color)
    pyxel.pset(sx2 - 1, sy2 - 1, color)

    pyxel.line(sx3 + 2, sy1, sx4 - 2, sy1, color)
    pyxel.line(sx3 + 2, sy2, sx4 - 2, sy2, color)
    pyxel.line(sx3, sy1 + 2, sx3, sy2 - 2, color)
    pyxel.line(sx4, sy1 + 2, sx4, sy2 - 2, color)
    pyxel.pset(sx3 + 1, sy1 + 1, color)
    pyxel.pset(sx4 - 1, sy1 + 1, color)
    pyxel.pset(sx3 + 1, sy2 - 1, color)
    pyxel.pset(sx4 - 1, sy2 - 1, color)

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


show_guide = False


def update():
    if pyxel.btnp(pyxel.KEY_SPACE):
        global show_guide
        show_guide = not show_guide


def draw():
    pyxel.cls(0)

    for i in range(-1, 2):
        for j in range(-1, 2):
            draw_gamepad(j, i, 1)
    draw_gamepad(0, 0, 7)

    pyxel.line(98, 1, 98, 96, 8)
    pyxel.line(100, 61, 181, 61, 8)
    pyxel.line(183, 1, 183, 96, 8)

    if show_guide:
        pyxel.rectb(0, 38, 12, 24, 3)
        pyxel.rectb(38, 0, 24, 12, 3)
        pyxel.rectb(86, 38, 12, 24, 3)
        pyxel.rectb(38, 86, 24, 12, 3)

        pyxel.rectb(99, 74, 12, 12, 3)
        pyxel.rectb(135, 74, 12, 12, 3)
        pyxel.rectb(171, 74, 12, 12, 3)
        pyxel.rectb(111, 62, 24, 12, 3)
        pyxel.rectb(147, 62, 24, 12, 3)
        pyxel.rectb(111, 86, 24, 12, 3)
        pyxel.rectb(147, 86, 24, 12, 3)

        pyxel.rectb(184, 0, 98, 98, 9)
        pyxel.rectb(185, 1, 31, 31, 3)
        pyxel.rectb(250, 1, 31, 31, 3)
        pyxel.rectb(185, 66, 31, 31, 3)
        pyxel.rectb(250, 66, 31, 31, 3)


pyxel.init(SCREEN_WDITH, SCREEN_HEIGHT, capture_scale=4)
pyxel.run(update, draw)
