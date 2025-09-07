import pyxel


class App:
    def __init__(self):
        pyxel.init(400, 60, title="Pyxel MML Studio", quit_key=pyxel.KEY_NONE)
        pyxel.run(self.update, self.draw)

    def update(self):
        pass

    def draw(self):
        pyxel.cls(0)
        pyxel.text(10, 10, "Hello, Pyxel!", pyxel.COLOR_WHITE)


App()
