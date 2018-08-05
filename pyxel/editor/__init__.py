import pyxel


class Editor:
    def __init__(self):
        pyxel.init(240, 180, caption='Pyxel')
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        self.draw_menu()
        self.draw_help()

    def draw_menu(self):
        pyxel.rect(0, 0, 239, 6, 6)
        pyxel.text(2, 1, 'MENU', 0)

        pyxel.rect(230, 1, 234, 5, 13)

    def draw_help(self):
        pyxel.rect(0, 173, 239, 179, 13)
        pyxel.text(2, 174, 'Help Message', 0)


def run():
    Editor()


if __name__ == '__main__':
    run()
