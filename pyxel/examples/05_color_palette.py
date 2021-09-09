import pyxel


def draw_palette(x, y, col):
    col_val = pyxel.DEFAULT_PALETTE[col]
    hex_col = "#{:06X}".format(col_val)
    rgb_col = "{},{},{}".format(col_val >> 16, (col_val >> 8) & 0xFF, col_val & 0xFF)

    pyxel.rect(x, y, 13, 13, col)
    pyxel.text(x + 16, y + 1, hex_col, 7)
    pyxel.text(x + 16, y + 8, rgb_col, 7)
    pyxel.text(x + 5 - (col // 10) * 2, y + 4, "{}".format(col), 7 if col < 6 else 0)

    if col == 0:
        pyxel.rectb(x, y, 13, 13, 13)


pyxel.init(255, 81, caption="Pyxel Color Palette")
pyxel.cls(0)

for i in range(16):
    draw_palette(2 + (i % 4) * 64, 4 + (i // 4) * 20, i)

pyxel.show()
