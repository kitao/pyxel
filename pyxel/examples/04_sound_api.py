import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 150, caption='Pixel Sound API')

        pyxel.sound(0).set(
            'e2e2c2g1 g1g1c2e2 d2d2b1g1 g1g1b1d2'
            'c2c2a1e1 e1e1a1c2 b1b1c2d2 d2b1c2d2', 'p', '6', 'vffn ffff', 30)

        pyxel.sound(1).set(
            'ra1b1c2 b1b1c2d2 g2g2g2g2 c2c2d2e2'
            'f2f2f2a1 b1b1c2d2 c2c2c2c2 c2c2rr', 'p', '6',
            'ffff vfff vvvv sfff svff vfff vvvv vfff', 30)

        pyxel.sound(2).set(
            'c1g1c1g1 c1g1c1g1 b0g1b0g1 b0g1b0g1'
            'a0e1a0e1 a0e1a0e1 g0d1g0d1 g0d1g0d1', 't', '7', 'n', 30)

        pyxel.sound(3).set(
            'f0c1f0c1 g0d1g0d1 e0b0e0b0 a0e1a0e1'
            'f0c1f0c1 g0d1g0d1 c0g0c0g0 c0g0c0g0', 't', '7', 'n', 30)

        pyxel.sound(4).set('f0ra4r', 'n', '6622', 'f', 30)

        pyxel.play(0, [0, 0, 1], loop=True)
        pyxel.play(1, [2, 2, 3], loop=True)
        pyxel.play(2, 4, loop=True)

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_1):
            pyxel.play(0, [0, 0, 1], loop=True)
            pyxel.play(1, [2, 2, 3], loop=True)
            pyxel.play(2, 4, loop=True)

        if pyxel.btnp(pyxel.KEY_2):
            pyxel.play(0, [0, 0, 1], loop=True)
            pyxel.stop(1)
            pyxel.stop(2)

        if pyxel.btnp(pyxel.KEY_3):
            pyxel.stop(0)
            pyxel.play(1, [2, 2, 3], loop=True)
            pyxel.stop(2)

        if pyxel.btnp(pyxel.KEY_4):
            pyxel.stop(0)
            pyxel.stop(1)
            pyxel.play(2, 4, loop=True)

        if pyxel.btnp(pyxel.KEY_5):
            pyxel.stop(0)
            pyxel.stop(1)
            pyxel.stop(2)

    def draw(self):
        pyxel.cls(1)

        pyxel.text(6, 6, 'sound(no).set(note,tone,volume,effect,speed)', 7)
        pyxel.rect(12, 16, 188, 52, 2)
        pyxel.text(16, 20, 'note  :[CDEFGAB] + [ #-] + [0-4]', 9)
        pyxel.text(16, 28, 'tone  :[T]riangle [S]quare [P]ulse [N]oise', 9)
        pyxel.text(16, 36, 'volume:[0-7]', 9)
        pyxel.text(16, 44, 'effect:[N]one [S]lide [V]ibrato [F]adeOut', 9)

        pyxel.text(6, 62, 'play(ch,no,loop=False)', 7)
        pyxel.text(6, 76, 'stop(ch)', 7)

        pyxel.rectb(6, 97, 193, 143, 14)
        pyxel.rect(6, 91, 34, 97, 14)
        pyxel.text(7, 92, 'CONTROL', 1)

        pyxel.text(12, 102, '1: Play all channels', 14)
        pyxel.text(12, 110, '2: Play channel #0 (Melody)', 14)
        pyxel.text(12, 118, '3: Play channel #1 (Bass)', 14)
        pyxel.text(12, 126, '4: Play channel #2 (Drums)', 14)
        pyxel.text(12, 134, '5: Stop playing', 14)


App()
