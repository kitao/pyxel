"""
Copyright (c) Cookie Yang. All right reserved.
"""
import pyxel

class App:
    def __init__(self):
        pyxel.init(160, 120)
        self.x = 0
        pyxel.run(self.update, self.draw)

    def update(self):
        self.x = (self.x + 1) % pyxel.width

    def draw(self):
        pyxel.cls(2)
        pyxel.rect(self.x, 0, 8, 8, 9)

App()