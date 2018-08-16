import pyxel

from .widget import Widget
from .console import Console
from .image_editor import ImageEditor
from .tilemap_editor import TileMapEditor
from .sound_editor import SoundEditor
from .music_editor import MusicEditor
from .radio_button import RadioButton


class EditorApp:
    def __init__(self, resoure_file, app_file):
        pyxel.init(240, 180, caption='pyxel')

        self._root = Widget(None, 0, 0, 0, 0)

        self._screen_list = [
            Console(self._root),
            ImageEditor(self._root),
            TileMapEditor(self._root),
            SoundEditor(self._root),
            MusicEditor(self._root),
        ]
        self._screen = None
        self._screen_button = RadioButton(self._root, 3, 1, 5, 1, 9)

        def on_value_change(value):
            self.set_screen(value)
        self._screen_button.on_value_change = on_value_change

        self.set_screen(0)

        pyxel.run(self.update, self.draw)

    def set_screen(self, screen):
        self._screen = self._screen_button.value = screen
        for i, widget in enumerate(self._screen_list):
            widget.set_visible(i == screen)

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btn(pyxel.KEY_LEFT_ALT) or pyxel.btn(pyxel.KEY_RIGHT_ALT):
            if pyxel.btnp(pyxel.KEY_LEFT):
                self.set_screen((self._screen - 1) % len(self._screen_list))
            elif pyxel.btnp(pyxel.KEY_RIGHT):
                self.set_screen((self._screen + 1) % len(self._screen_list))

        self._root.process_mouse_event()
        self._root.update()

    def draw(self):
        pyxel.cls(6)
        self._root.draw()
