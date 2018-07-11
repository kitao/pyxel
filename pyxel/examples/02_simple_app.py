import pyxel


class App():
    def __init__(self):
        pyxel.init(160, 120, caption='Simple App')

        pyxel.image(0).load('assets/sample_tile.png')
        # Pico 8 tiles by Kicked-in-Teeth
        # https://kicked-in-teeth.itch.io/pico-8-tiles

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        pyxel.cls(2)

        pyxel.blt(0, 0, 0, 0, 0, 96, 96)


App()
