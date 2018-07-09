import pyxel


class Editor:
    def __init__(self):
        pyxel.init(160, 120, caption='Pyxel')
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        pyxel.text(9, 8, "Hello, Pyxel Editor!", 8)
        pyxel.text(8, 8, "Hello, Pyxel Editor!", 7)


def run():
    Editor()


if __name__ == '__main__':
    run()
