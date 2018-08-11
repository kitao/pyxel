import pyxel
from pyxel.editor.console import Console
from pyxel.editor.editor_constants import (MODE_CONSOLE, MODE_IMAGE_EDITOR,
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

        self.editor_list = [
            Console(),
            ImageEditor(),
            TileMapEditor(),
            SoundEditor(),
            MusicEditor()
        ]

        self.cur_editor = MODE_IMAGE_EDITOR

        pyxel.run(self.update, self.draw)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        for editor in self.editor_list:
            editor.update()

    def draw(self):
        pyxel.cls(6)

        for editor in self.editor_list:
            editor.draw()


def run():
    Editor()


if __name__ == '__main__':
    run()
