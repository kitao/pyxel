import math
import pyxel


class App(pyxel.App):
    def __init__(self):
        super().__init__(200, 150, caption='Pyxel Draw Test')

        self.image = pyxel.Image(16, 16)
        image_data = [
            '3330333330333333', '3309033309033333', '309f00009f033303',
            '30ffffffff033090', '09ffffffff9030f0', '0f0fffff0ff03090',
            '0ffff0fffff030f0', '0fff0f0fff9000f0', '09ffffffff9f9ff0',
            '309fffffff9f9ff0', '330fffffffffff90', '330fffffffffff90',
            '330f99f999f99f03', '330f00f000f00f03', '3309009030900903',
            '3330330333033033'
        ]
        self.image.set(0, 0, 16, 16, image_data)
        self.bank(0, self.image)

    def update(self):
        if self.btnp(pyxel.KEY_Q):
            exit()

    def draw(self):
        self.test_cls(4, 6)
        self.test_pix(4, 20)
        self.test_line(104, 6)
        self.test_rect(4, 40)
        self.test_rectb(104, 40)
        self.test_circ(4, 64)
        self.test_circ(104, 64)
        self.test_blt(4, 94)
        self.test_text(4, 124)

    def test_cls(self, x, y):
        self.cls(2)

        self.text(x, y, 'cls(col)', 7)

    def test_pix(self, x, y):
        self.text(x, y, 'pix(x,y,col)', 7)

        x += 4
        y += 10

        for i in range(16):
            self.pix(x + i * 2, y, i)

    def test_line(self, x, y):
        self.text(x, y, 'line(x1,y1,x2,y2,col)', 7)

        x += 4
        y += 8
        col = 5

        for i in range(3):
            self.line(x, y + i * 8, x + 48, y + i * 8, col)
            col += 1

        for i in range(4):
            self.line(x + i * 16, y, x + i * 16, y + 16, col)
            col += 1

        for i in range(4):
            self.line(x + i * 16, y, x + (3 - i) * 16, y + 16, col)
            col += 1

    def test_rect(self, x, y):
        self.text(x, y, 'rect(x1,y1,x2,y2,col)', 7)

        x += 4
        y += 15

        for i in range(8):
            self.rect(x + i * 8, y, x + i * 9, y - i, i + 8)

    def test_rectb(self, x, y):
        self.text(x, y, 'rectb(x1,y1,x2,y2,col)', 7)

        x += 4
        y += 15

        for i in range(8):
            self.rectb(x + i * 8, y, x + i * 9, y - i, i + 8)

    def test_circ(self, x, y):
        self.text(x, y, 'circ(x,y,r,col)', 7)

        x += 4
        y += 15

        for i in range(8):
            self.circ(x + i * 8, y, i, i + 8)

    def test_circb(self, x, y):
        self.text(x, y, 'circb(x,y,r,col)', 7)

        x += 4
        y += 15

        for i in range(8):
            self.circb(x + i * 8, y, i, i + 8)

    def test_blt(self, x, y):
        self.text(x, y, 'blt(x,y,bank,sx,sy,w,h,[colkey])', 7)

        x += 4
        y += 8
        offset = math.sin(self.frame_count * 0.1) * 2

        self.blt(x, y, 0, 0, 0, 16, 16)
        self.blt(x + offset + 20, y, 0, 0, 0, 16, 16, 3)
        self.blt(x + 40, y, 0, 0, 0, -16, 16, 3)
        self.blt(x + 60, y, 0, 0, 0, 16, -16, 3)
        self.blt(x + 80, y, 0, 0, 0, -16, -16, 3)

    def test_text(self, x, y):
        self.text(x, y, 'text(x,y,str,col)', 7)

        x += 4
        y += 8
        text = 'Elapsed frame count is {}\n' \
               'Current mouse position is ({},{})'''.format(
                   self.frame_count, self.mouse_x, self.mouse_y)

        self.text(x + 1, y, text, 1)
        self.text(x, y, text, 9)


App().run()
