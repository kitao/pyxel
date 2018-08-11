import pyxel
import pyxel.editor

from .mode import Mode


class ImageEditor(Mode):
    def __init__(self):
        super().__init__('image_editor.png')

    def update(self):
        super().update()

    def draw(self):
        super().draw()
