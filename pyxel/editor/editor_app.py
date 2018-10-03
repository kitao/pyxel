import os

import pyxel
from pyxel.ui import ImageButton, RadioButton, Widget
from pyxel.ui.ui_constants import WIDGET_BACKGROUND_COLOR

from .editor_constants import EDITOR_HEIGHT, EDITOR_WIDTH
from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .sound_editor import SoundEditor
from .tilemap_editor import TileMapEditor


class EditorApp(Widget):
    def __init__(self, resource_file):
        resource_file = os.path.join(os.getcwd(), resource_file)
        root, ext = os.path.splitext(resource_file)
        if ext != ".pyxel":
            resource_file += ".pyxel"

        pyxel.init(
            EDITOR_WIDTH,
            EDITOR_HEIGHT,
            caption="Pyxel Editor - {}".format(resource_file),
        )

        try:
            pyxel.load(resource_file)
        except FileNotFoundError:
            pass

        super().__init__(None, 0, 0, pyxel.width, pyxel.height)

        self._resource_file = resource_file

        self._screen_list = [
            ImageEditor(self),
            TileMapEditor(self),
            SoundEditor(self),
            MusicEditor(self),
        ]

        self._editor_button = RadioButton(self, 3, 1, 3, 3, 13, 4)
        self._undo_button = ImageButton(self, 48, 1, 3, 48, 13)
        self._redo_button = ImageButton(self, 57, 1, 3, 57, 13)
        self._save_button = ImageButton(self, 75, 1, 3, 75, 13)

        self._editor_button.add_event_handler(
            "change", lambda value: self.set_screen(value)
        )
        self.set_screen(0)

        self._undo_button.add_event_handler("press", self.__on_undo_press)
        self._redo_button.add_event_handler("press", self.__on_redo_press)
        self._save_button.add_event_handler("press", self.__on_save_press)

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

        pyxel.run(self.update_widgets, self.draw_widgets)

    def set_screen(self, screen):
        self._editor_button.value = screen

        for i, widget in enumerate(self._screen_list):
            widget.is_visible = i == screen

    def __on_update(self):
        if pyxel.btn(pyxel.KEY_LEFT_ALT) or pyxel.btn(pyxel.KEY_RIGHT_ALT):
            screen = self._screen_button.value
            screen_count = len(self._screen_list)

            if pyxel.btnp(pyxel.KEY_LEFT):
                self.set_screen((screen - 1) % screen_count)
            elif pyxel.btnp(pyxel.KEY_RIGHT):
                self.set_screen((screen + 1) % screen_count)

        screen = self._screen_list[self._editor_button.value]
        self._undo_button.is_enabled = screen.can_undo
        self._redo_button.is_enabled = screen.can_redo

        if pyxel.btn(pyxel.KEY_CONTROL):
            if screen.can_undo and pyxel.btnp(pyxel.KEY_S):
                self._save_button.press()
            elif screen.can_undo and pyxel.btnp(pyxel.KEY_Z):
                self._undo_button.press()
            elif screen.can_redo and pyxel.btnp(pyxel.KEY_Y):
                self._redo_button.press()

    def __on_draw(self):
        pyxel.cls(WIDGET_BACKGROUND_COLOR)
        self.draw_frame(-2, 0, 241, 8)

    def __on_undo_press(self):
        self._screen_list[self._screen_button.value].undo()

    def __on_redo_press(self):
        self._screen_list[self._screen_button.value].redo()

    def __on_save_press(self):
        pyxel.save(self._resource_file)
