import pyxel


def draw_text_with_border(x, y, s, col, bcol, font):
    for dx in range(-1, 2):
        for dy in range(-1, 2):
            if dx != 0 or dy != 0:
                pyxel.text(
                    x + dx,
                    y + dy,
                    s,
                    bcol,
                    font,
                )

    pyxel.text(x, y, s, col, font)


pyxel.init(128, 128, title="Custom Font")
pyxel.load("assets/sample.pyxres")

font10 = pyxel.Font("assets/PixelMplus10-Regular.ttf", 10)
font12 = pyxel.Font("assets/PixelMplus12-Regular.ttf", 12)

pyxel.cls(1)
pyxel.blt(0, 0, 1, 0, 0, 128, 128)

s = "▲Pyxel︎▲"
w = font10.text_width(s)
pyxel.rect(21, 18, w, 1, 15)
pyxel.text(21, 8, s, 8, font10)

draw_text_with_border(4, 98, "気軽に楽しく", 7, 5, font12)
draw_text_with_border(4, 113, "プログラミング！", 7, 5, font12)

pyxel.show()
