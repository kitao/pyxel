import os

import pyxel
from pyxel.ui import ImageButton, RadioButton, Widget
from pyxel.ui.constants import (
    WIDGET_BACKGROUND_COLOR,
    WIDGET_HOLD_TIME,
    WIDGET_PANEL_COLOR,
    WIDGET_REPEAT_TIME,
    WIDGET_SHADOW_COLOR,
)

from .constants import (
    APP_HEIGHT,
    APP_WIDTH,
    EDITOR_IMAGE_NAME,
    EDITOR_IMAGE_X,
    EDITOR_IMAGE_Y,
)
from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .sound_editor import SoundEditor
from .tilemap_editor import TileMapEditor


class App(Widget):
    def __init__(self, resource_file):
        resource_file = os.path.join(os.getcwd(), resource_file)
        root, ext = os.path.splitext(resource_file)
        if ext != pyxel.RESOURCE_FILE_EXTENSION and ext != ".pyxel":
            resource_file += pyxel.RESOURCE_FILE_EXTENSION

        pyxel.init(
            APP_WIDTH, APP_HEIGHT, caption="Pyxel Editor - {}".format(resource_file)
        )
        pyxel.mouse(True)

        if os.path.exists(resource_file):
            pyxel.load(resource_file)

        if ext == ".pyxel":
            resource_file = root + pyxel.RESOURCE_FILE_EXTENSION

        super().__init__(None, 0, 0, pyxel.width, pyxel.height)

        self._resource_file = resource_file
        self._editor_list = [
            ImageEditor(self),
            TileMapEditor(self),
            SoundEditor(self),
            MusicEditor(self),
        ]
        self._editor_button = RadioButton(
            self,
            1,
            1,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X,
            EDITOR_IMAGE_Y,
            4,
            0,
        )
        self._undo_button = ImageButton(
            self,
            48,
            1,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X + 36,
            EDITOR_IMAGE_Y,
        )
        self._redo_button = ImageButton(
            self,
            57,
            1,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X + 45,
            EDITOR_IMAGE_Y,
        )
        self._save_button = ImageButton(
            self,
            75,
            1,
            pyxel.IMAGE_BANK_FOR_SYSTEM,
            EDITOR_IMAGE_X + 54,
            EDITOR_IMAGE_Y,
        )
        self.help_message = ""

        self._editor_button.add_event_handler(
            "change", lambda value: self._set_editor(value)
        )
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self._undo_button.add_event_handler("press", self.__on_undo_button_press)
        self._undo_button.add_event_handler("repeat", self.__on_undo_button_press)
        self._redo_button.add_event_handler("press", self.__on_redo_button_press)
        self._redo_button.add_event_handler("repeat", self.__on_redo_button_press)
        self._save_button.add_event_handler("press", self.__on_save_button_press)
        self._editor_button.add_event_handler(
            "mouse_hover", self.__on_editor_button_mouse_hover
        )
        self._undo_button.add_event_handler(
            "mouse_hover", self.__on_undo_button_mouse_hover
        )
        self._redo_button.add_event_handler(
            "mouse_hover", self.__on_redo_button_mouse_hover
        )
        self._save_button.add_event_handler(
            "mouse_hover", self.__on_save_button_mouse_hover
        )

        image_file = os.path.join(
            os.path.dirname(__file__), "assets", EDITOR_IMAGE_NAME
        )
        pyxel.image(pyxel.IMAGE_BANK_FOR_SYSTEM, system=True).load(
            EDITOR_IMAGE_X, EDITOR_IMAGE_Y, image_file
        )

        self._set_editor(0)

        pyxel.run(self.update_widgets, self.draw_widgets)

    def _set_editor(self, editor):
        self._editor_button.value = editor

        for i, widget in enumerate(self._editor_list):
            widget.is_visible = i == editor

    def __on_update(self):
        if pyxel._drop_file:
            ext = os.path.splitext(pyxel._drop_file)[1]

            if ext == pyxel.RESOURCE_FILE_EXTENSION:
                pyxel.stop()
                for editor in self._editor_list:
                    editor.reset_history()
                pyxel.load(pyxel._drop_file)
                pyxel._caption(pyxel._drop_file)
            else:
                self._editor_list[self._editor_button.value].call_event_handler(
                    "drop", pyxel._drop_file
                )

        if pyxel.btn(pyxel.KEY_LEFT_ALT) or pyxel.btn(pyxel.KEY_RIGHT_ALT):
            editor = self._editor_button.value
            editor_count = len(self._editor_list)

            if pyxel.btnp(pyxel.KEY_LEFT):
                self._set_editor((editor - 1) % editor_count)
            elif pyxel.btnp(pyxel.KEY_RIGHT):
                self._set_editor((editor + 1) % editor_count)

        editor = self._editor_list[self._editor_button.value]
        self._undo_button.is_enabled = editor.can_undo
        self._redo_button.is_enabled = editor.can_redo

        if pyxel.btn(pyxel.KEY_CONTROL):
            if pyxel.btnp(pyxel.KEY_S):
                self._save_button.press()

            if editor.can_undo and pyxel.btnp(
                pyxel.KEY_Z, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
            ):
                self._undo_button.press()

            if editor.can_redo and pyxel.btnp(
                pyxel.KEY_Y, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
            ):
                self._redo_button.press()

    def __on_draw(self):
        pyxel.cls(WIDGET_BACKGROUND_COLOR)
        pyxel.rect(0, 0, 240, 9, WIDGET_PANEL_COLOR)
        pyxel.line(0, 9, 239, 9, WIDGET_SHADOW_COLOR)

        pyxel.text(93, 2, self.help_message, 13)
        self.help_message = ""

    def __on_undo_button_press(self):
        self._editor_list[self._editor_button.value].undo()

    def __on_redo_button_press(self):
        self._editor_list[self._editor_button.value].redo()

    def __on_save_button_press(self):
        pyxel.save(self._resource_file)

    def __on_editor_button_mouse_hover(self, x, y):
        self.help_message = "EDITOR:ALT+LEFT/RIGHT"

    def __on_undo_button_mouse_hover(self, x, y):
        self.help_message = "UNDO:CTRL+Z"

    def __on_redo_button_mouse_hover(self, x, y):
        self.help_message = "REDO:CTRL+Y"

    def __on_save_button_mouse_hover(self, x, y):
        self.help_message = "SAVE:CTRL+S"
