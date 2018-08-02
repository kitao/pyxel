import math

import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 150, caption='Pyxel Draw API')

        pyxel.image(0).load(0, 0, 'assets/cat_16x16.png')

        self.pal_test_is_enabled = False
        self.clip_test_is_enabled = False

        pyxel.run(self.update, self.draw)

    def update(self):
        self.pal_test_is_enabled = (pyxel.frame_count // 30) % 10 >= 5
        self.clip_test_is_enabled = pyxel.btn(pyxel.KEY_SPACE)

        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        self.test_pal1()
        self.test_cls(6, 6)
        self.test_clip()
        self.test_pix(6, 20)
        self.test_line(106, 6)
        self.test_rect(6, 40)
        self.test_rectb(106, 40)
        self.test_circ(6, 64)
        self.test_circb(106, 64)
        self.test_blt(6, 94)
        self.test_text(6, 124)
        self.test_pal2(106, 124)

    def test_pal1(self):
        if self.pal_test_is_enabled:
            pyxel.pal(2, 3)
            pyxel.pal(4, 7)
            pyxel.pal(7, 10)

    def test_pal2(self, x, y):
        pyxel.text(x, y, 'pal(col1,col2)', 4)
        pyxel.pal()

    def test_cls(self, x, y):
        pyxel.cls(2)

        pyxel.text(x, y, 'cls(col)', 7)

    def test_clip(self):
        pyxel.clip()

        if not self.clip_test_is_enabled:
            return

        x1 = math.sin(pyxel.frame_count * 0.02) * 39 + 40
        y1 = math.sin(pyxel.frame_count * 0.03) * 29 + 30
        x2 = x1 + 119
        y2 = y1 + 89

        pyxel.text(x1, y1 - 8, 'clip(x1,y1,x2,y2)', 14)
        pyxel.rectb(x1 - 1, y1 - 1, x2 + 1, y2 + 1, 14)
        pyxel.clip(x1, y1, x2, y2)

    def test_pix(self, x, y):
        pyxel.text(x, y, 'pix(x,y,col)', 7)

        x += 4
        y += 10

        for i in range(16):
            pyxel.pix(x + i * 2, y, i)

    def test_line(self, x, y):
        pyxel.text(x, y, 'line(x1,y1,x2,y2,col)', 7)

        x += 4
        y += 8
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
        pyxel.text(x, y, 'rect(x1,y1,x2,y2,col)', 7)

        x += 4
        y += 15

        for i in range(8):
            pyxel.rect(x + i * 8, y, x + i * 9, y - i, i + 8)

    def test_rectb(self, x, y):
        pyxel.text(x, y, 'rectb(x1,y1,x2,y2,col)', 7)

        x += 4
        y += 15

        for i in range(8):
            pyxel.rectb(x + i * 8, y, x + i * 9, y - i, i + 8)

    def test_circ(self, x, y):
        pyxel.text(x, y, 'circ(x,y,r,col)', 7)

        x += 4
        y += 15

        for i in range(8):
            pyxel.circ(x + i * 8, y, i, i + 8)

    def test_circb(self, x, y):
        pyxel.text(x, y, 'circb(x,y,r,col)', 7)

        x += 4
        y += 15

        for i in range(8):
            pyxel.circb(x + i * 8, y, i, i + 8)

    def test_blt(self, x, y):
        pyxel.text(x, y, 'blt(x,y,img,sx,sy,w,h,[colkey])', 7)

        x += 4
        y += 8
        offset = math.sin(pyxel.frame_count * 0.1) * 2

        pyxel.blt(x, y, 0, 0, 0, 16, 16)
        pyxel.blt(x + offset + 20, y, 0, 0, 0, 16, 16, 5)
        pyxel.blt(x + 40, y, 0, 0, 0, -16, 16, 5)
        pyxel.blt(x + 60, y, 0, 0, 0, 16, -16, 5)
        pyxel.blt(x + 80, y, 0, 0, 0, -16, -16, 5)

    def test_text(self, x, y):
        pyxel.text(x, y, 'text(x,y,s,col)', 7)

        x += 4
        y += 8
        s = 'Elapsed frame count is {}\n' \
            'Current mouse position is ({},{})'.format(
                pyxel.frame_count, pyxel.mouse_x, pyxel.mouse_y)

        pyxel.text(x + 1, y, s, 1)
        pyxel.text(x, y, s, 9)


App()
