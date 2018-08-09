import os.path

import pyxel

from .widget import Widget


class ImageEditor(Widget):
    def __init__(self):
        dirname = os.path.join(os.path.dirname(__file__), 'assets')
        pyxel.image(
            3, system=True).load(
                0, 16, 'image_editor.png', dirname=dirname)

    def update(self):
        pass

    def draw(self):
        pyxel.blt(0, 0, 3, 0, 16, 240, 180, 6)
