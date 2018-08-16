import os

import numpy as np

import pyxel

from .editor_constants import SCREEN_HEIGHT, SCREEN_WIDTH
from .widget import Widget


class Screen(Widget):
    def __init__(self, parent, image_file):
        super().__init__(
            parent, 0, 0, SCREEN_WIDTH, SCREEN_HEIGHT, is_visible=False)

        dirname = os.path.join(os.path.dirname(__file__), 'assets')
        pyxel.image(3, system=True).load(0, 16, image_file, dirname=dirname)

        data = pyxel.image(3, system=True).data
        self._image_data = np.copy(data[16:SCREEN_HEIGHT + 16, 0:SCREEN_WIDTH])

    def on_show(self):
        data = pyxel.image(3, system=True).data
        data[16:16 + SCREEN_HEIGHT, 0:SCREEN_WIDTH] = self._image_data

    def on_draw(self):
        pyxel.blt(0, 0, 3, 0, 16, SCREEN_WIDTH, SCREEN_HEIGHT, 6)
