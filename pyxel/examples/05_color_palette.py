import pyxel


def draw_palette(x, y, col):
    rgb = pyxel.colors[col]
    hex = f"#{rgb:06X}"
    dec = f"{rgb >> 16},{(rgb >> 8) & 0xFF},{rgb & 0xFF}"
    pyxel.rect(x, y, 13, 13, col)
    pyxel.text(x + 16, y + 1, hex, 7)
    pyxel.text(x + 16, y + 8, dec, 7)
    pyxel.text(x + 5 - (col // 10) * 2, y + 4, f"{col}", 7 if col < 6 else 0)
    if col == 0:
        pyxel.rectb(x, y, 13, 13, 13)


pyxel.init(255, 81, title="Pyxel Color Palette")
pyxel.cls(0)
for i in range(16):
    draw_palette(2 + (i % 4) * 64, 4 + (i // 4) * 20, i)
pyxel.show()
