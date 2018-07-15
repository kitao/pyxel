import pyxel


class App:
    def __init__(self):
        pyxel.init(200, 150, caption='Pixel Sound API')

        pyxel.sound(0).set('c2c2g2g2a2a2g2.', 'p', '7', 'ffffffvf ffffffvf',
                           60)

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_1):
            pyxel.play(0, 0, loop=True)

        if pyxel.btnp(pyxel.KEY_2):
            pyxel.play(1, [0, 0], loop=True)

        if pyxel.btnp(pyxel.KEY_3):
            pyxel.play(2, [0, 0])

        if pyxel.btnp(pyxel.KEY_4):
            pyxel.play(3, 0)

        if pyxel.btnp(pyxel.KEY_5):
            pyxel.stop(0)
            pyxel.stop(1)
            pyxel.stop(2)
            pyxel.stop(3)

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


App()
