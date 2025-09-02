import pyxel

BT1_W = 24
BT1_L = 25
BT1_M = 12
BT1_AREA_X = 0
BT1_AREA_W = BT1_W + BT1_L * 2 + BT1_M * 2

BT2_W = 32
BT2_H = 12
BT2_M = 7
BT2_AREA_W = BT2_W * 2 + BT2_M * 4
BT2_AREA_H = BT2_H + BT2_M * 2
BT2_AREA_X = BT1_AREA_W + 1
BT2_AREA_Y = BT1_AREA_W - BT2_AREA_H

BT3_W = 32
BT3_M = 5
BT3_I = 24
BT3_X = BT3_M + BT3_W + (BT3_I - BT3_W) // 2
BT3_AREA_X = BT2_AREA_X + BT2_AREA_W + 1
BT3_AREA_W = BT3_M * 2 + BT3_W * 2 + BT3_I

SCR_W = BT3_AREA_X + BT3_AREA_W
SCR_H = BT1_AREA_W


def draw_gamepad_cross(x, y, color):
    pyxel.camera(-x, -y)

    x1 = BT1_M
    x2 = x1 + BT1_L
    x3 = x2 + BT1_W - 1
    x4 = x3 + BT1_L

    y1 = x1
    y2 = x2
    y3 = x3
    y4 = x4

    pyxel.line(x2 + 2, y1, x3 - 2, y1, color)
    pyxel.line(x2 + 2, y4, x3 - 2, y4, color)
    pyxel.line(x1, y2 + 2, x1, y3 - 2, color)
    pyxel.line(x4, y2 + 2, x4, y3 - 2, color)

    pyxel.line(x1 + 2, y2, x2, y2, color)
    pyxel.line(x3, y2, x4 - 2, y2, color)
    pyxel.line(x1 + 2, y3, x2, y3, color)
    pyxel.line(x3, y3, x4 - 2, y3, color)

    pyxel.line(x2, y1 + 2, x2, y2, color)
    pyxel.line(x2, y3, x2, y4 - 2, color)
    pyxel.line(x3, y1 + 2, x3, y2, color)
    pyxel.line(x3, y3, x3, y4 - 2, color)

    pyxel.pset(x2 + 1, y1 + 1, color)
    pyxel.pset(x3 - 1, y1 + 1, color)
    pyxel.pset(x2 + 1, y4 - 1, color)
    pyxel.pset(x3 - 1, y4 - 1, color)
    pyxel.pset(x1 + 1, y2 + 1, color)
    pyxel.pset(x1 + 1, y3 - 1, color)
    pyxel.pset(x4 - 1, y2 + 1, color)
    pyxel.pset(x4 - 1, y3 - 1, color)

    pyxel.camera()


def draw_gamepad_menu(x, y, color):
    pyxel.camera(-x, -y)

    x1 = BT2_M
    x2 = x1 + BT2_W - 1
    x3 = x2 + BT2_M * 2 + 1
    x4 = x3 + BT2_W - 1

    y1 = BT2_M
    y2 = BT2_M + BT2_H - 1

    pyxel.line(x1 + 2, y1, x2 - 2, y1, color)
    pyxel.line(x1 + 2, y2, x2 - 2, y2, color)
    pyxel.line(x1, y1 + 2, x1, y2 - 2, color)
    pyxel.line(x2, y1 + 2, x2, y2 - 2, color)

    pyxel.pset(x1 + 1, y1 + 1, color)
    pyxel.pset(x2 - 1, y1 + 1, color)
    pyxel.pset(x1 + 1, y2 - 1, color)
    pyxel.pset(x2 - 1, y2 - 1, color)

    pyxel.line(x3 + 2, y1, x4 - 2, y1, color)
    pyxel.line(x3 + 2, y2, x4 - 2, y2, color)
    pyxel.line(x3, y1 + 2, x3, y2 - 2, color)
    pyxel.line(x4, y1 + 2, x4, y2 - 2, color)

    pyxel.pset(x3 + 1, y1 + 1, color)
    pyxel.pset(x4 - 1, y1 + 1, color)
    pyxel.pset(x3 + 1, y2 - 1, color)
    pyxel.pset(x4 - 1, y2 - 1, color)

    pyxel.camera()


def draw_gamepad_button(x, y, color):
    pyxel.camera(-x, -y)

    x1 = BT3_M
    x2 = BT3_X
    x3 = x1 + BT3_W + BT3_I

    y1 = x1
    y2 = x2
    y3 = x3

    pyxel.ellib(x1, y2, BT3_W, BT3_W, color)
    pyxel.ellib(x3, y2, BT3_W, BT3_W, color)
    pyxel.ellib(x2, y1, BT3_W, BT3_W, color)
    pyxel.ellib(x2, y3, BT3_W, BT3_W, color)

    pyxel.camera()


class App:
    def __init__(self):
        pyxel.init(SCR_W, SCR_H)
        self.show_guide = False
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_ESCAPE):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_SPACE):
            self.show_guide = not self.show_guide

        if pyxel.btnp(pyxel.KEY_RETURN):
            cross_image = pyxel.Image(BT1_AREA_W, BT1_AREA_W)
            cross_image.blt(0, 0, pyxel.screen, BT1_AREA_X, 0, BT1_AREA_W, BT1_AREA_W)
            cross_image.save(f"gamepad_cross_{BT1_AREA_W}x{BT1_AREA_W}.png", 1)

            menu_image = pyxel.Image(BT2_AREA_W, BT2_AREA_H)
            menu_image.blt(
                0, 0, pyxel.screen, BT2_AREA_X, BT2_AREA_Y, BT2_AREA_W, BT2_AREA_W
            )
            menu_image.save(f"gamepad_menu_{BT2_AREA_W}x{BT2_AREA_H}.png", 1)

            button_image = pyxel.Image(BT3_AREA_W, BT3_AREA_W)
            button_image.blt(0, 0, pyxel.screen, BT3_AREA_X, 0, BT3_AREA_W, BT3_AREA_W)
            button_image.save(f"gamepad_button_{BT3_AREA_W}x{BT3_AREA_W}.png", 1)

    def draw(self):
        pyxel.cls(0)

        for i in range(-1, 2):
            for j in range(-1, 2):
                draw_gamepad_cross(BT1_AREA_X + j, i, 1)
                draw_gamepad_menu(
                    BT2_AREA_X + j,
                    BT2_AREA_Y + i,
                    1,
                )
                draw_gamepad_button(BT3_AREA_X + j, i, 1)

        draw_gamepad_cross(BT1_AREA_X, 0, 12)
        draw_gamepad_menu(BT2_AREA_X, BT2_AREA_Y, 12)
        draw_gamepad_button(BT3_AREA_X, 0, 12)

        pyxel.rect(BT2_AREA_X - 1, 0, 1, SCR_H, 3)
        pyxel.rect(
            BT2_AREA_X,
            BT2_AREA_Y - 1,
            BT2_AREA_W,
            1,
            3,
        )
        pyxel.rect(BT3_AREA_X - 1, 0, 1, SCR_H, 3)

        if self.show_guide:
            pyxel.camera(-BT1_AREA_X, 0)

            pyxel.rectb(0, BT1_M + BT1_L, BT1_M, BT1_W, 8)
            pyxel.rectb(BT1_M + BT1_L * 2 + BT1_W, BT1_M + BT1_L, BT1_M, BT1_W, 8)
            pyxel.rectb(BT1_M + BT1_L, 0, BT1_W, BT1_M, 8)
            pyxel.rectb(BT1_M + BT1_L, BT1_M + BT1_L * 2 + BT1_W, BT1_W, BT1_M, 8)

            pyxel.rectb(0, 0, BT1_M + BT1_L, BT1_M + BT1_L, 9)
            pyxel.rectb(BT1_M + BT1_L + BT1_W, 0, BT1_M + BT1_L, BT1_M + BT1_L, 9)
            pyxel.rectb(0, BT1_M + BT1_L + BT1_W, BT1_M + BT1_L, BT1_M + BT1_L, 9)
            pyxel.rectb(
                BT1_M + BT1_L + BT1_W,
                BT1_M + BT1_L + BT1_W,
                BT1_M + BT1_L,
                BT1_M + BT1_L,
                9,
            )

            pyxel.camera(-BT2_AREA_X, -BT2_AREA_Y)

            pyxel.rectb(0, BT2_M, BT2_M, BT2_H, 8)
            pyxel.rectb(BT2_M + BT2_W, BT2_M, BT2_M, BT2_H, 8)
            pyxel.rectb(BT2_M, 0, BT2_W, BT2_M, 8)
            pyxel.rectb(BT2_M, BT2_M + BT2_H, BT2_W, BT2_M, 8)

            pyxel.rectb(BT2_M * 2 + BT2_W, BT2_M, BT2_M, BT2_H, 8)
            pyxel.rectb(BT2_M * 3 + BT2_W * 2, BT2_M, BT2_M, BT2_H, 8)
            pyxel.rectb(BT2_M * 3 + BT2_W, 0, BT2_W, BT2_M, 8)
            pyxel.rectb(BT2_M * 3 + BT2_W, BT2_M + BT2_H, BT2_W, BT2_M, 8)

            pyxel.camera(-BT3_AREA_X, 0)

            pyxel.rectb(0, BT3_X, BT3_M, BT3_W, 8)
            pyxel.rectb(BT3_M + BT3_W * 2 + BT3_I, BT3_X, BT3_M, BT3_W, 8)
            pyxel.rectb(BT3_X, 0, BT3_W, BT3_M, 8)
            pyxel.rectb(BT3_X, BT3_M + BT3_W * 2 + BT3_I, BT3_W, BT3_M, 8)

            pyxel.rectb(0, 0, BT3_X, BT3_X, 9)
            pyxel.rectb(BT3_X + BT3_W, 0, BT3_X, BT3_X, 9)
            pyxel.rectb(0, BT3_X + BT3_W, BT3_X, BT3_X, 9)
            pyxel.rectb(BT3_X + BT3_W, BT3_X + BT3_W, BT3_X, BT3_X, 9)

            pyxel.camera()


App()
