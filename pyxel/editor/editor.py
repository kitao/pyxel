import os

import numpy as np

import pyxel
from pyxel.ui import Widget

from .constants import EDITOR_HEIGHT, EDITOR_WIDTH


class Editor(Widget):
    """
    Events:
        __on_undo(data)
        __on_redo(data)
    """

    def __init__(self, parent):
        super().__init__(parent, 0, 0, EDITOR_WIDTH, EDITOR_HEIGHT, is_visible=False)

        data = pyxel.image(3, system=True).data
        self._image_data = np.copy(data[12 : EDITOR_HEIGHT + 12, 0:EDITOR_WIDTH])

        self._edit_history_list = []
        self._edit_history_index = 0

    @property
    def help_message(self):
        return self.parent.help_message

    @help_message.setter
    def help_message(self, value):
        self.parent.help_message = value

    @property
    def can_undo(self):
        return self._edit_history_index > 0

    @property
    def can_redo(self):
        return self._edit_history_index < len(self._edit_history_list)

    def undo(self):
        if not self.can_undo:
            return

        self._edit_history_index -= 1
        self.call_event_handler(
            "undo", self._edit_history_list[self._edit_history_index]
        )

    def redo(self):
        if not self.can_redo:
            return

        self.call_event_handler(
            "redo", self._edit_history_list[self._edit_history_index]
        )
        self._edit_history_index += 1

    def add_edit_history(self, data):
        self._edit_history_list = self._edit_history_list[: self._edit_history_index]
        self._edit_history_list.append(data)
        self._edit_history_index += 1

    def add_number_picker_help(self, number_picker):
        number_picker.dec_button.add_event_handler(
            "mouse_hover", self.__number_picker_on_mouse_hover
        )
        number_picker.inc_button.add_event_handler(
            "mouse_hover", self.__number_picker_on_mouse_hover
        )

    def __number_picker_on_mouse_hover(self, x, y):
        self.help_message = "SHIFT:x10"

    def draw_not_implemented_message(self):
        pyxel.rect(78, 83, 163, 97, 11)
        pyxel.rectb(78, 83, 163, 97, 1)
        pyxel.text(84, 88, "NOT IMPLEMENTED YET", 1)
