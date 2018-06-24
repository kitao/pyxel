import pyxel


class Editor(pyxel.App):
    def __init__(self):
        super().__init__(160, 120)

    def update(self):
        self.text(9, 8, "Hello, Pyxel Editor!", 8)
        self.text(8, 8, "Hello, Pyxel Editor!", 7)

        if self.btnp(pyxel.KEY_Q):
            exit()


def run():
    Editor().run()
