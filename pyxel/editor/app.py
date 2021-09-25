import os

import pyxel

from .image_editor import ImageEditor
from .music_editor import MusicEditor
from .settings import APP_HEIGHT, APP_WIDTH, EDITOR_IMAGE, HELP_MESSAGE_COLOR
from .sound_editor import SoundEditor
from .tilemap_editor import TilemapEditor
from .widgets import ImageButton, RadioButton, Widget
from .widgets.settings import (WIDGET_BACKGROUND_COLOR, WIDGET_HOLD_TIME,
                               WIDGET_PANEL_COLOR, WIDGET_REPEAT_TIME,
                               WIDGET_SHADOW_COLOR)


class App(Widget):
    def __init__(self, resource_file):
        resource_file = os.path.join(os.getcwd(), resource_file)
        file_ext = os.path.splitext(resource_file)[1]
        if file_ext != pyxel.RESOURCE_FILE_EXTENSION:
            resource_file += pyxel.RESOURCE_FILE_EXTENSION

        pyxel.init(APP_WIDTH, APP_HEIGHT)
        pyxel.mouse(True)
        App._set_title(resource_file)

        if os.path.exists(resource_file):
            pyxel.load(resource_file)

        super().__init__(None, 0, 0, pyxel.width, pyxel.height)

        self._resource_file = resource_file
        self._editors = [
            ImageEditor(self),
            TilemapEditor(self),
            SoundEditor(self),
            MusicEditor(self),
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

        self.editor_no = 0

        pyxel.run(self.update_widgets, self.draw_widgets)

    @property
    def editor_no(self):
        return self._editor_button.value

    @editor_no.setter
    def editor_no(self, value):
        self._editor_button.value = value
        for i, widget in enumerate(self._editors):
            widget.is_visible = i == value

    @property
    def _editor(self):
        return self._editors[self._editor_button.value]

    @staticmethod
    def _set_title(filename):
        pyxel.title("Pyxel Editor - {}".format(filename))

    def __on_update(self):
        if pyxel.drop_files:
            drop_file = pyxel.drop_files[-1]
            file_ext = os.path.splitext(drop_file)[1]

            if file_ext == pyxel.RESOURCE_FILE_EXTENSION:
                pyxel.stop()

                if pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI):
                    self._editor.reset_history()

                    load_flag = (
                        (True, False, False, False),
                        (False, True, False, False),
                        (False, False, True, False),
                        (False, False, False, True),
                    )[self.editor_no]

                    pyxel.load(
                        pyxel._drop_file,
                        image=load_flag[0],
                        tilemap=load_flag[1],
                        sound=load_flag[2],
                        music=load_flag[3],
                    )
                else:
                    for editor in self._editors:
                        editor.reset_history()

                    pyxel.load(drop_file)
                    App._set_title(drop_file)
            else:
                self._editor.call_event_handler("drop", drop_file)

        if pyxel.btn(pyxel.KEY_ALT):
            if pyxel.btnp(pyxel.KEY_LEFT):
                self.editor_no = (self.editor_no - 1) % len(self._editors)
            elif pyxel.btnp(pyxel.KEY_RIGHT):
                self.editor_no = (self.editor_no + 1) % len(self._editors)

        self._undo_button.is_enabled = self._editor.can_undo
        self._redo_button.is_enabled = self._editor.can_redo

        if pyxel.btn(pyxel.KEY_CTRL) or pyxel.btn(pyxel.KEY_GUI):
            if pyxel.btnp(pyxel.KEY_S):
                self._save_button.press()

            if self._editor.can_undo and pyxel.btnp(
                pyxel.KEY_Z, WIDGET_HOLD_TIME, WIDGET_REPEAT_TIME
            ):
                self._undo_button.press()

            if self._editor.can_redo and pyxel.btnp(
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
