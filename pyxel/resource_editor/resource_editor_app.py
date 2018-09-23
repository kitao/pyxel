import pyxel
from pyxel.ui import Widget

from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .resource_editor_button import ResourceEditorButton
from .resource_editor_constants import SCREEN_HEIGHT, SCREEN_WIDTH
from .resource_editor_radio_button import ResourceEditorRadioButton
from .sound_editor import SoundEditor
from .tilemap_editor import TileMapEditor


class ResourceEditorApp:
    def __init__(self, resoure_file):
        pyxel.init(SCREEN_WIDTH, SCREEN_HEIGHT, caption="Pyxel Resource Editor")

        self._root_widget = Widget(None, 0, 0, 0, 0)

        self._screen_list = [
            ImageEditor(self._root_widget),
            TileMapEditor(self._root_widget),
            SoundEditor(self._root_widget),
            MusicEditor(self._root_widget),
        ]

        self._screen_button = ResourceEditorRadioButton(
            self._root_widget, 3, 1, 4, 1, 2
        )
        self._screen_button.add_event_handler(
            "change", lambda value: self.set_screen(value)
        )
        self.set_screen(0)

        self._undo_button = ResourceEditorButton(self._root_widget, 48, 1, 7, 7)
        self._undo_button.add_event_handler("press", self.__on_undo_press)

        self._redo_button = ResourceEditorButton(self._root_widget, 57, 1, 7, 7)
        self._redo_button.add_event_handler("press", self.__on_redo_press)

        self._save_button = ResourceEditorButton(
            self._root_widget, 75, 1, 7, 7, is_enabled=False
        )
        self._save_button.add_event_handler("press", self.__on_save_press)

        pyxel.run(self.update, self.draw)

    def set_screen(self, screen):
        self._screen_button.value = screen

        for i, widget in enumerate(self._screen_list):
            widget.is_visible = i == screen

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

    def __on_undo_press(self):
        self._screen_list[self._screen_button.value].undo()

    def __on_redo_press(self):
        self._screen_list[self._screen_button.value].redo()

    def __on_save_press(self):
        pass
