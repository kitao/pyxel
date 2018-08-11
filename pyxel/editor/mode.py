import os

import numpy as np

import pyxel


class Mode:
    def __init__(self, filename):
        dirname = os.path.join(os.path.dirname(__file__), 'assets')
        pyxel.image(0, system=True).load(0, 0, filename, dirname=dirname)

        self.image_data = np.copy(
            pyxel.image(0, system=True).data[0:180, 0:240])

    def show(self):
        pyxel.image(3, system=True).data[16:196, 0:240] = self.image_data

    def hide(self):
        pass

    def update(self):
        pass

    def draw(self):
        pyxel.blt(0, 0, 3, 0, 16, 240, 180, 6)
