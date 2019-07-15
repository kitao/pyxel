import os.path

import pyxel
from pyxel.ui import Widget

from .constants import (
    TOOL_BUCKET,
    TOOL_CIRC,
    TOOL_CIRCB,
    TOOL_PENCIL,
    TOOL_RECT,
    TOOL_RECTB,
    TOOL_SELECT,
)


class Editor(Widget):
    """
    Events:
        __on_undo(data)
        __on_redo(data)
        __on_drop(filenames)
    """

    def __init__(self, parent):
        super().__init__(parent, 0, 0, 0, 0, is_visible=False)

        self._history_list = []
        self._history_index = 0

        self.add_event_handler("drop", self.__on_drop)

    @property
    def help_message(self):
        return self.parent.help_message

    @help_message.setter
    def help_message(self, value):
        self.parent.help_message = value

    @property
    def can_undo(self):
        return self._history_index > 0

    @property
    def can_redo(self):
        return self._history_index < len(self._history_list)

    def undo(self):
        if not self.can_undo:
            return

        self._history_index -= 1
        self.call_event_handler("undo", self._history_list[self._history_index])

    def redo(self):
        if not self.can_redo:
            return

        self.call_event_handler("redo", self._history_list[self._history_index])
        self._history_index += 1

    def add_history(self, data):
        self._history_list = self._history_list[: self._history_index]
        self._history_list.append(data)
        self._history_index += 1

    def reset_history(self):
        self._history_list = []
        self._history_index = 0

    def add_number_picker_help(self, number_picker):
        number_picker.dec_button.add_event_handler(
            "mouse_hover", self.__on_number_picker_dec_mouse_hover
        )
        number_picker.inc_button.add_event_handler(
            "mouse_hover", self.__on_number_picker_inc_mouse_hover
        )

    def __on_number_picker_dec_mouse_hover(self, x, y):
        self.help_message = "-10:SHIFT+CLICK"

    def __on_number_picker_inc_mouse_hover(self, x, y):
        self.help_message = "+10:SHIFT+CLICK"

    def check_tool_button_shortcuts(self):
        if pyxel.btn(pyxel.KEY_CONTROL):
            return

        if pyxel.btnp(pyxel.KEY_S):
            self._tool_button.value = TOOL_SELECT
        elif pyxel.btnp(pyxel.KEY_P):
            self._tool_button.value = TOOL_PENCIL
        elif pyxel.btnp(pyxel.KEY_R):
            self._tool_button.value = (
                TOOL_RECT if pyxel.btn(pyxel.KEY_SHIFT) else TOOL_RECTB
            )
        elif pyxel.btnp(pyxel.KEY_C):
            self._tool_button.value = (
                TOOL_CIRC if pyxel.btn(pyxel.KEY_SHIFT) else TOOL_CIRCB
            )
        elif pyxel.btnp(pyxel.KEY_B):
            self._tool_button.value = TOOL_BUCKET

    def add_tool_button_help(self, tool_button):
        tool_button.add_event_handler("mouse_hover", self.__on_tool_button_mouse_hover)

    def __on_tool_button_mouse_hover(self, x, y):
        value = self._tool_button.check_value(x, y)

        if value == TOOL_SELECT:
            s = "SELECT:S"
        elif value == TOOL_PENCIL:
            s = "PENCIL:P"
        elif value == TOOL_RECTB:
            s = "RECTANGLE:R"
        elif value == TOOL_RECT:
            s = "FILLED-RECT:SHIFT+R"
        elif value == TOOL_CIRCB:
            s = "CIRCLE:C"
        elif value == TOOL_CIRC:
            s = "FILLED-CIRC:SHIFT+C"
        elif value == TOOL_BUCKET:
            s = "BUCKET:B"
        else:
            s = ""

        self.help_message = s

    def __on_drop(self, filenames):
        for filename in filenames:
            _, ext = os.path.splitext(filename)

            if ext.lower() == ".pyxel":
                pyxel.load(filename)
                return
