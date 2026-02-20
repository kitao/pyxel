import pyxel


def draw_corner_markers(x, y, w, h, main_color, border_color):
    # Top-left
    cx, cy = x, y
    pyxel.rectb(cx - 2, cy - 2, 2, 4, border_color)
    pyxel.rectb(cx - 2, cy - 2, 4, 2, border_color)
    pyxel.pset(cx, cy, border_color)
    pyxel.pset(cx - 1, cy - 1, main_color)
    pyxel.pset(cx - 1, cy, main_color)
    pyxel.pset(cx, cy - 1, main_color)

    # Top-right
    cx = x + w - 1
    cy = y
    pyxel.rectb(cx + 1, cy - 2, 2, 4, border_color)
    pyxel.rectb(cx - 1, cy - 2, 4, 2, border_color)
    pyxel.pset(cx, cy, border_color)
    pyxel.pset(cx + 1, cy - 1, main_color)
    pyxel.pset(cx + 1, cy, main_color)
    pyxel.pset(cx, cy - 1, main_color)

    # Bottom-left
    cx = x
    cy = y + h - 1
    pyxel.rectb(cx - 2, cy - 1, 2, 4, border_color)
    pyxel.rectb(cx - 2, cy + 1, 4, 2, border_color)
    pyxel.pset(cx, cy, border_color)
    pyxel.pset(cx - 1, cy + 1, main_color)
    pyxel.pset(cx - 1, cy, main_color)
    pyxel.pset(cx, cy + 1, main_color)

    # Bottom-right
    cx = x + w - 1
    cy = y + h - 1
    pyxel.rectb(cx + 1, cy - 1, 2, 4, border_color)
    pyxel.rectb(cx - 1, cy + 1, 4, 2, border_color)
    pyxel.pset(cx, cy, border_color)
    pyxel.pset(cx + 1, cy + 1, main_color)
    pyxel.pset(cx + 1, cy, main_color)
    pyxel.pset(cx, cy + 1, main_color)


def draw_small_corner_markers(x, y, w, h, main_color, border_color):
    for cx, cy, dx, dy in [
        (x, y, -1, -1),
        (x + w - 1, y, 1, -1),
        (x, y + h - 1, -1, 1),
        (x + w - 1, y + h - 1, 1, 1),
    ]:
        pyxel.pset(cx + dx, cy + dy, border_color)
        pyxel.pset(cx, cy + dy, border_color)
        pyxel.pset(cx + dx, cy, border_color)
        pyxel.pset(cx, cy, main_color)
