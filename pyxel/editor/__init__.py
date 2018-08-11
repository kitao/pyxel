import pyxel
from pyxel.editor.console import Console
from pyxel.editor.editor_constants import (BUTTON_SIZE, MODE_CONSOLE,
                                           MODE_IMAGE_EDITOR,
                                           MODE_MUSIC_EDITOR,
                                           MODE_SOUND_EDITOR,
                                           MODE_TILEMAP_EDITOR)
from pyxel.editor.image_editor import ImageEditor
from pyxel.editor.music_editor import MusicEditor
from pyxel.editor.sound_editor import SoundEditor
from pyxel.editor.tilemap_editor import TileMapEditor


class Editor:
    def __init__(self):
        pyxel.init(240, 180, caption='Pyxel')

        self._mode_list = [
            Console(),
            ImageEditor(),
            TileMapEditor(),
            SoundEditor(),
            MusicEditor(),
        ]

        self._mode = None
        self._set_mode(MODE_CONSOLE)

        pyxel.run(self.update, self.draw)

    def _set_mode(self, mode):
        if self._mode is not None:
            self._mode_list[self._mode].hide()
        self._mode_list[mode].show()
        self._mode = mode

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btnp(pyxel.KEY_LEFT_BUTTON):
            y = 1
            if pyxel.mouse_y >= y and pyxel.mouse_y < y + BUTTON_SIZE:
                for i in range(5):
                    x = i * 9 + 3
                    if pyxel.mouse_x >= x and pyxel.mouse_x < x + BUTTON_SIZE:
                        self._set_mode(i)

        self._mode_list[self._mode].update()

    def draw(self):
        pyxel.cls(6)

        self._mode_list[self._mode].draw()


def run():
    Editor()


if __name__ == '__main__':
    run()
