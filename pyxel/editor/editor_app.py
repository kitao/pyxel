import pyxel
from pyxel.ui import Widget

from .console import Console
from .editor_button import EditorButton
from .editor_constants import SCREEN_HEIGHT, SCREEN_WIDTH
from .editor_radio_button import EditorRadioButton
from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .sound_editor import SoundEditor
from .tilemap_editor import TileMapEditor


class EditorApp:
    def __init__(self, resoure_file, app_file):
        pyxel.init(SCREEN_WIDTH, SCREEN_HEIGHT, caption="pyxel")

        self._root_widget = Widget(None, 0, 0, 0, 0)

        self._screen_list = [
            Console(self._root_widget),
            ImageEditor(self._root_widget),
            TileMapEditor(self._root_widget),
            SoundEditor(self._root_widget),
            MusicEditor(self._root_widget),
        ]

        self._screen_button = EditorRadioButton(self._root_widget, 3, 1, 5, 1, 2)
        self._screen_button.add_event_handler(
            "change", lambda value: self.set_screen(value)
        )
        self.set_screen(1)

        self._undo_button = EditorButton(self._root_widget, 57, 1, 7, 7)
        self._undo_button.add_event_handler("press", self.on_undo_press)

        self._redo_button = EditorButton(self._root_widget, 66, 1, 7, 7)
        self._redo_button.add_event_handler("press", self.on_redo_press)

        pyxel.run(self.update, self.draw)

    def set_screen(self, screen):
        self._screen_button.value = screen

        for i, widget in enumerate(self._screen_list):
            widget.is_visible = i == screen

    def on_undo_press(self):
        self._screen_list[self._screen_button.value].undo()

    def on_redo_press(self):
        self._screen_list[self._screen_button.value].redo()

    def update(self):
        if pyxel.btnp(pyxel.KEY_Q):
            pyxel.quit()

        if pyxel.btn(pyxel.KEY_LEFT_ALT) or pyxel.btn(pyxel.KEY_RIGHT_ALT):
            screen = self._screen_button.value
            screen_count = len(self._screen_list)

            if pyxel.btnp(pyxel.KEY_LEFT):
                self.set_screen((screen - 1) % screen_count)
            elif pyxel.btnp(pyxel.KEY_RIGHT):
                self.set_screen((screen + 1) % screen_count)

        screen = self._screen_list[self._screen_button.value]
        self._undo_button.is_enabled = screen.can_undo
        self._redo_button.is_enabled = screen.can_redo

        if pyxel.btn(pyxel.KEY_CONTROL):
            if screen.can_undo and pyxel.btnp(pyxel.KEY_Z):
                self._undo_button.press()
            elif screen.can_redo and pyxel.btnp(pyxel.KEY_Y):
                self._redo_button.press()

        Widget.update(self._root_widget)

    def draw(self):
        pyxel.cls(6)
        Widget.draw(self._root_widget)
