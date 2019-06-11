import os.path

import pyxel
from pyxel.ui import ColorPicker, NumberPicker, RadioButton

from .constants import EDITOR_IMAGE_X, EDITOR_IMAGE_Y, IMAGE_BANK_COUNT, TOOL_PENCIL
from .drawing_panel import DrawingPanel
from .editor import Editor
from .image_panel import ImagePanel


class ImageEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self._drawing_panel = DrawingPanel(self, is_tilemap_mode=False)
        self._image_panel = ImagePanel(self, is_tilemap_mode=False)
        self._color_picker = ColorPicker(self, 11, 156, 7, with_shadow=False)
        self._tool_button = RadioButton(
            self, 81, 161, 3, EDITOR_IMAGE_X + 63, EDITOR_IMAGE_Y, 7, TOOL_PENCIL
        )
        self._image_picker = NumberPicker(self, 192, 161, 0, IMAGE_BANK_COUNT - 2, 0)

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)
        self.add_event_handler("drop", self.__on_drop)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self.add_tool_button_help(self._tool_button)
        self.add_number_picker_help(self._image_picker)

    @property
    def color(self):
        return self._color_picker.value

    @color.setter
    def color(self, value):
        self._color_picker.value = int(value)

    @property
    def tool(self):
        return self._tool_button.value

    @tool.setter
    def tool(self, value):
        self._tool_button.value = value

    @property
    def image(self):
        return self._image_picker.value

    @property
    def drawing_x(self):
        return self._drawing_panel.viewport_x

    @drawing_x.setter
    def drawing_x(self, value):
        self._drawing_panel.viewport_x = value

    @property
    def drawing_y(self):
        return self._drawing_panel.viewport_y

    @drawing_y.setter
    def drawing_y(self, value):
        self._drawing_panel.viewport_y = value

    def __on_undo(self, data):
        img = data["image"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self.drawing_x = x
        self.drawing_y = y
        self.parent.image = img

    def __on_redo(self, data):
        img = data["image"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self.drawing_x = x
        self.drawing_y = y
        self.parent.image = img

    def __on_drop(self, filename):
        _, ext = os.path.splitext(filename)

        if ext.lower() == ".png":
            pyxel.image(self.image).load(0, 0, filename)
            return

    def __on_update(self):
        self.check_tool_button_shortcuts()

    def __on_draw(self):
        self.draw_panel(11, 156, 136, 17)
        self.draw_panel(157, 156, 72, 17)
        pyxel.text(170, 162, "IMAGE", 6)
