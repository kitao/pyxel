import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 150, title="Pyxel Draw API")
        pyxel.mouse(True)

        pyxel.images[0].load(0, 0, "assets/cat_16x16.png")
        pyxel.images[1].load(0, 0, "assets/tileset_24x32.png")

        pyxel.tilemaps[0].set(
            0,
            0,
            [
                "0201 0000 0200 0400 0100 0000 0003 0103 0203 0000 0002",
                "0202 0300 0001 0101 0201 0300 0000 0100 0200 0300 0003",
            ],
        )
        pyxel.tilemaps[0].imgsrc = 1

        self.pal_test_is_enabled = False
        self.clip_test_is_enabled = False

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        self.pal_test_is_enabled = (pyxel.frame_count // 30) % 10 >= 5
        self.clip_test_is_enabled = pyxel.btn(pyxel.KEY_SPACE)

    def draw(self):
        self.test_pal1()
        self.test_cls(6, 6)
        self.test_clip()
        self.test_pset(6, 20)
        self.test_line(106, 6)
        self.test_rect(6, 38)
        self.test_rectb(106, 38)
        self.test_circ(6, 61)
        self.test_circb(106, 61)
        self.test_blt(6, 88)
        self.test_bltm(106, 88)
        self.test_text(6, 124)
        self.test_pal2(106, 124)

    def test_pal1(self):
        if self.pal_test_is_enabled:
            pyxel.pal(5, 2)
            pyxel.pal(12, 7)
            pyxel.pal(7, 10)

    def test_pal2(self, x, y):
        pyxel.text(x, y, "pal(col1,col2)", 12)
        pyxel.pal()

    def test_cls(self, x, y):
        pyxel.cls(5)
        pyxel.text(x, y, "cls(col)", 7)

    def test_clip(self):
        pyxel.clip()

        if not self.clip_test_is_enabled:
            return

        x = pyxel.sin(pyxel.frame_count * 1.14) * 39 + 40
        y = pyxel.sin(pyxel.frame_count * 1.71) * 29 + 30
        w = 120
        h = 90

        pyxel.text(x, y - 8, "clip(x,y,w,h)", 14)
        pyxel.rectb(x - 1, y - 1, w + 2, h + 2, 14)

        pyxel.clip(x, y, w, h)

    def test_pset(self, x, y):
        pyxel.text(x, y, "pset(x,y,col)", 7)

        x += 4
        y += 10
        for i in range(16):
            pyxel.pset(x + i * 2, y, i)

    def test_line(self, x, y):
        pyxel.text(x, y, "line(x1,y1,x2,y2,col)", 7)

        x += 4
        y += 9
        col = 5

        for i in range(3):
            pyxel.line(x, y + i * 8, x + 48, y + i * 8, col)
            col += 1

        for i in range(4):
            pyxel.line(x + i * 16, y, x + i * 16, y + 16, col)
            col += 1

        for i in range(4):
            pyxel.line(x + i * 16, y, x + (3 - i) * 16, y + 16, col)
            col += 1

    def test_rect(self, x, y):
        pyxel.text(x, y, "rect(x,y,w,h,col)", 7)

        x += 4
        y += 16
        for i in range(8):
            pyxel.rect(x + i * 8, y - i, i + 1, i + 1, i + 8)

    def test_rectb(self, x, y):
        pyxel.text(x, y, "rectb(x,y,w,h,col)", 7)

        x += 4
        y += 16
        for i in range(8):
            pyxel.rectb(x + i * 8, y - i, i + 1, i + 1, i + 8)

    def test_circ(self, x, y):
        pyxel.text(x, y, "circ(x,y,r,col)", 7)

        x += 4
        y += 15
        for i in range(8):
            pyxel.circ(x + i * 8, y, i, i + 8)

    def test_circb(self, x, y):
        pyxel.text(x, y, "circb(x,y,r,col)", 7)

        x += 4
        y += 15
        for i in range(8):
            pyxel.circb(x + i * 8, y, i, i + 8)

    def test_blt(self, x, y):
        pyxel.text(x, y, "blt(x,y,img,u,v,\n    w,h,[colkey])", 7)

        y += 15
        offset = pyxel.sin(pyxel.frame_count * 5.73) * 2

        pyxel.blt(x, y, 0, 0, 0, 16, 16)
        pyxel.blt(x + offset + 19, y, 0, 0, 0, 16, 16, 13)
        pyxel.blt(x + 38, y, 0, 0, 0, -16, 16, 13)
        pyxel.blt(x + 57, y, 0, 0, 0, 16, -16, 13)
        pyxel.blt(x + 76, y, 0, 0, 0, -16, -16, 13)

    def test_bltm(self, x, y):
        pyxel.text(x, y, "bltm(x,y,tm,u,v,\n     w,h,[colkey])", 7)

        y += 15
        pyxel.bltm(x, y, 0, 0, 0, 88, 16, 2)

    def test_text(self, x, y):
        pyxel.text(x, y, "text(x,y,s,col)", 7)

        x += 4
        y += 8
        s = (
            f"Elapsed frame count is {pyxel.frame_count}\n"
            f"Current mouse position is ({pyxel.mouse_x},{pyxel.mouse_y})"
        )
        pyxel.text(x + 1, y, s, 1)
        pyxel.text(x, y, s, 9)


App()
