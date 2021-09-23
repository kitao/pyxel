import os

import pyxel

from .image_button import ImageButton
from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .radio_button import RadioButton
from .settings import (
    APP_HEIGHT,
    APP_WIDTH,
    EDITOR_IMAGE,
    HELP_MESSAGE_COLOR,
    WIDGET_BACKGROUND_COLOR,
    WIDGET_HOLD_TIME,
    WIDGET_PANEL_COLOR,
    WIDGET_REPEAT_TIME,
    WIDGET_SHADOW_COLOR,
)
from .sound_editor import SoundEditor
from .tilemap_editor import TileMapEditor
from .widget import Widget

IMAGE_EDITOR = 0
TILEMAP_EDITOR = 1
SOUND_EDITOR = 2
MUSIC_EDITOR = 3


class App(Widget):
    def __init__(self, resource_file):
        resource_file = os.path.join(os.getcwd(), resource_file)
        file_ext = os.path.splitext(resource_file)[1]
        if file_ext != pyxel.RESOURCE_FILE_EXTENSION:
            resource_file += pyxel.RESOURCE_FILE_EXTENSION

        pyxel.init(
            APP_WIDTH, APP_HEIGHT, title="Pyxel Editor - {}".format(resource_file)
        )
        pyxel.mouse(True)

        if os.path.exists(resource_file):
            pyxel.load(resource_file)

        super().__init__(None, 0, 0, pyxel.width, pyxel.height)

        self._resource_file = resource_file
        self._editor_list = [
            ImageEditor(self),
            # TileMapEditor(self),
            # SoundEditor(self),
            # MusicEditor(self),
        ]
        self._editor_button = RadioButton(
            self,
            1,
            1,
            EDITOR_IMAGE,
            0,
            0,
            4,
            0,
        )
        self._undo_button = ImageButton(
            self,
            48,
            1,
            EDITOR_IMAGE,
            36,
            0,
        )
        self._redo_button = ImageButton(
            self,
            57,
            1,
            EDITOR_IMAGE,
            45,
            0,
        )
        self._save_button = ImageButton(
            self,
            75,
            1,
            EDITOR_IMAGE,
            54,
            0,
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

        self._set_editor(IMAGE_EDITOR)

        pyxel.run(self.update_widgets, self.draw_widgets)

    @property
    def _editor(self):
        return self._editor_list[self._editor_button.value]

    def _set_editor(self, editor):
        self._editor_button.value = editor

        for i, widget in enumerate(self._editor_list):
            widget.is_visible = i == editor

    def __on_update(self):
        if pyxel.drop_files:
            drop_file = pyxel.drop_files[-1]
            file_ext = os.path.splitext(drop_file)[1]

            if file_ext == pyxel.RESOURCE_FILE_EXTENSION:
                pyxel.stop()

                if pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI):
                    editor_index = self._editor_button.value
                    self._editor.reset_history()

                    if editor_index == IMAGE_EDITOR:
                        pyxel.load(
                            pyxel._drop_file, tilemap=False, sound=False, music=False
                        )
                    elif editor_index == TILEMAP_EDITOR:
                        pyxel.load(
                            pyxel._drop_file, image=False, sound=False, music=False
                        )
                    elif editor_index == SOUND_EDITOR:
                        pyxel.load(
                            pyxel._drop_file, image=False, tilemap=False, music=False
                        )
                    elif editor_index == MUSIC_EDITOR:
                        pyxel.load(
                            pyxel._drop_file, image=False, tilemap=False, sound=False
                        )
                else:
                    for editor in self._editor_list:
                        editor.reset_history()
                    pyxel.load(drop_file)

                pyxel._caption(drop_file)
            else:
                self._editor.call_event_handler("drop", drop_file)

        if pyxel.btn(pyxel.KEY_ALT):
            editor_index = self._editor_button.value
            editor_count = len(self._editor_list)

            if pyxel.btnp(pyxel.KEY_LEFT):
                self._set_editor((editor_index - 1) % editor_count)
            elif pyxel.btnp(pyxel.KEY_RIGHT):
                self._set_editor((editor_index + 1) % editor_count)

        self._undo_button.is_enabled = self._editor.can_undo
        self._redo_button.is_enabled = self._editor.can_redo

        if pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI):
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

        pyxel.text(93, 2, self.help_message, HELP_MESSAGE_COLOR)
        self.help_message = ""

    def __on_undo_button_press(self):
        self._editor.undo()

    def __on_redo_button_press(self):
        self._editor.redo()

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
