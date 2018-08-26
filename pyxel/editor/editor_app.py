import pyxel

from .button import Button
from .console import Console
from .editor_constants import SCREEN_HEIGHT, SCREEN_WIDTH
from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .radio_button import RadioButton
from .sound_editor import SoundEditor
from .tilemap_editor import TileMapEditor
from .widget import Widget


class EditorApp:
    def __init__(self, resoure_file, app_file):
        pyxel.init(SCREEN_WIDTH, SCREEN_HEIGHT, caption='pyxel')

        self._root_widget = Widget(None, 0, 0, 0, 0)

        self._screen_list = [
            Console(self._root_widget),
            ImageEditor(self._root_widget),
            TileMapEditor(self._root_widget),
            SoundEditor(self._root_widget),
            MusicEditor(self._root_widget),
        ]

        self._screen_button = RadioButton(self._root_widget, 3, 1, 5, 1, 9)
        self._screen_button.add_event_handler(
            'change', lambda value: self.set_screen(value))
        self.set_screen(1)

        self._undo_button = Button(self._root_widget, 57, 1, 7, 7)
        self._undo_button.add_event_handler('press', self.on_undo_press)

        self._redo_button = Button(self._root_widget, 66, 1, 7, 7)
        self._redo_button.add_event_handler('press', self.on_redo_press)

        pyxel.run(self.update, self.draw)

    def set_screen(self, screen):
        self._screen_button.value = screen

        for i, widget in enumerate(self._screen_list):
            widget.is_visible = (i == screen)

    def on_undo_press(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
            self._screen_list[self._screen_button.value].undo()

    def on_redo_press(self, key, x, y):
        if key == pyxel.KEY_LEFT_BUTTON:
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

        if pyxel.btn(pyxel.KEY_LEFT_CONTROL) or pyxel.btn(
                pyxel.KEY_RIGHT_CONTROL):
            if pyxel.btnp(pyxel.KEY_Z):
                x = self._undo_button.x
                y = self._undo_button.y
                self._undo_button.call_event_handler(
                    'press', pyxel.KEY_LEFT_BUTTON, x, y)
            elif pyxel.btnp(pyxel.KEY_Y):
                x = self._redo_button.x
                y = self._redo_button.y
                self._redo_button.call_event_handler(
                    'press', pyxel.KEY_LEFT_BUTTON, x, y)

        screen = self._screen_list[self._screen_button.value]
        self._undo_button.is_enabled = screen.can_undo
        self._redo_button.is_enabled = screen.can_redo

        Widget.process_input(self._root_widget)
        Widget.update(self._root_widget)

    def draw(self):
        pyxel.cls(6)
        Widget.draw(self._root_widget)
