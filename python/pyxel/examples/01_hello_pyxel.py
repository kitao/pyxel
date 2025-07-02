import pyxel


class App:
    def __init__(self):
        pyxel.init(160, 120, title="Hello Pyxel")
        pyxel.images[0].load(0, 0, "assets/pyxel_logo_38x16.png")
        pyxel.play(
            0,
            "T200 Y0 q3 ccccccccc T100 @GLI1{100,0} @GLI0 V8 c V10 d V15 e r8 f L16 fg @ENV1{15,100,0,100,15} @VIB1{48,10,100} L2 c..",
        )
        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

    def draw(self):
        pyxel.cls(0)
        pyxel.text(55, 41, "Hello, Pyxel!", pyxel.frame_count % 16)
        pyxel.blt(61, 66, 0, 0, 0, 38, 16)


App()
