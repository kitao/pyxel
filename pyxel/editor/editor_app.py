import os

import pyxel
from pyxel.ui import ImageButton, RadioButton, Widget
from pyxel.ui.constants import WIDGET_BACKGROUND_COLOR

from .constants import EDITOR_HEIGHT, EDITOR_WIDTH
from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .sound_editor import SoundEditor
from .tilemap_editor import TileMapEditor

EDITOR_IMAGE = 0


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

        self._editor_list = [
            ImageEditor(self),
            TileMapEditor(self),
            SoundEditor(self),
            MusicEditor(self),
        ]

        self._editor_button = RadioButton(self, 1, 1, 3, 0, 16, 4, EDITOR_IMAGE)
        self._undo_button = ImageButton(self, 48, 1, 3, 36, 16)
        self._redo_button = ImageButton(self, 57, 1, 3, 45, 16)
        self._save_button = ImageButton(self, 75, 1, 3, 54, 16)

        self._editor_button.add_event_handler(
            "change", lambda value: self.set_editor(value)
        )
        self.set_editor(0)

        self._undo_button.add_event_handler("press", self.__on_undo_press)
        self._redo_button.add_event_handler("press", self.__on_redo_press)
        self._save_button.add_event_handler("press", self.__on_save_press)

        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)

        image_file = os.path.join(os.path.dirname(__file__), "assets", "editor.png")
        pyxel.image(3, system=True).load(0, 16, image_file)

        pyxel.run(self.update_widgets, self.draw_widgets)

    def set_editor(self, editor):
        self._editor_button.value = editor

        for i, widget in enumerate(self._editor_list):
            widget.is_visible = i == editor

    def __on_update(self):
        if pyxel.btn(pyxel.KEY_LEFT_ALT) or pyxel.btn(pyxel.KEY_RIGHT_ALT):
            editor = self._editor_button.value
            editor_count = len(self._editor_list)

            if pyxel.btnp(pyxel.KEY_LEFT):
                self.set_editor((editor - 1) % editor_count)
            elif pyxel.btnp(pyxel.KEY_RIGHT):
                self.set_editor((editor + 1) % editor_count)

        editor = self._editor_list[self._editor_button.value]
        self._undo_button.is_enabled = editor.can_undo
        self._redo_button.is_enabled = editor.can_redo

        if pyxel.btn(pyxel.KEY_CONTROL):
            if editor.can_undo and pyxel.btnp(pyxel.KEY_S):
                self._save_button.press()
            elif editor.can_undo and pyxel.btnp(pyxel.KEY_Z):
                self._undo_button.press()
            elif editor.can_redo and pyxel.btnp(pyxel.KEY_Y):
                self._redo_button.press()

    def __on_draw(self):
        pyxel.cls(WIDGET_BACKGROUND_COLOR)
        self.draw_frame(-2, 0, 241, 8)

    def __on_undo_press(self):
        self._editor_list[self._editor_button.value].undo()

    def __on_redo_press(self):
        self._editor_list[self._editor_button.value].redo()

    def __on_save_press(self):
        pyxel.save(self._resource_file)
