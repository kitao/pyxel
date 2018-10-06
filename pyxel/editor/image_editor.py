import pyxel
from pyxel.constants import RENDERER_IMAGE_COUNT
from pyxel.ui import ColorPicker, NumberPicker, RadioButton

from .constants import (
    TOOL_BUCKET,
    TOOL_CIRC,
    TOOL_CIRCB,
    TOOL_PENCIL,
    TOOL_RECT,
    TOOL_RECTB,
    TOOL_SELECT,
)
from .edit_frame import EditFrame
from .editor import Editor
from .image_frame import ImageFrame


class ImageEditor(Editor):
    def __init__(self, parent):
        super().__init__(parent)

        self._edit_frame = EditFrame(self, is_tilemap_mode=False)
        self._image_frame = ImageFrame(self, is_tilemap_mode=False)
        self._color_picker = ColorPicker(self, 11, 156, 7, with_shadow=False)
        self._tool_button = RadioButton(self, 81, 161, 3, 63, 16, 7, TOOL_PENCIL)
        self._image_picker = NumberPicker(
            self, 192, 161, 0, RENDERER_IMAGE_COUNT - 2, 0
        )

        self.add_event_handler("undo", self.__on_undo)
        self.add_event_handler("redo", self.__on_redo)
        self.add_event_handler("update", self.__on_update)
        self.add_event_handler("draw", self.__on_draw)
        self.add_number_picker_help(self._image_picker)
        self._tool_button.add_event_handler(
            "mouse_hover", self.__tool_button_on_mouse_hover
        )

    @property
    def color(self):
        return self._color_picker.value

    @color.setter
    def color(self, value):
        self._color_picker.value = value

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
    def edit_x(self):
        return self._edit_frame.viewport_x

    @edit_x.setter
    def edit_x(self, value):
        self._edit_frame.viewport_x = value

    @property
    def edit_y(self):
        return self._edit_frame.viewport_y

    @edit_y.setter
    def edit_y(self, value):
        self._edit_frame.viewport_y = value

    def __on_undo(self, data):
        img = data["image"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["before"]

        self.edit_x = x
        self.edit_y = y
        self.parent.image = img

    def __on_redo(self, data):
        img = data["image"]
        x, y = data["pos"]
        dest = pyxel.image(img).data[y : y + 16, x : x + 16]
        dest[:, :] = data["after"]

        self.edit_x = x
        self.edit_y = y
        self.parent.image = img

    def __on_update(self):
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

    def __on_draw(self):
        self.draw_frame(11, 156, 136, 17)
        self.draw_frame(157, 156, 72, 17)
        pyxel.text(170, 162, "IMAGE", 6)

    def __tool_button_on_mouse_hover(self, x, y):
        value = self._tool_button.hit_value(x, y)

        if value == TOOL_SELECT:
            s = "SELECT:S"
        elif value == TOOL_PENCIL:
            s = "PENCIL:P"
        elif value == TOOL_RECTB:
            s = "RECTANGLE:R"
        elif value == TOOL_RECT:
            s = "F.RECTANGLE:SHIFT+R"
        elif value == TOOL_CIRCB:
            s = "CIRCLE:C"
        elif value == TOOL_CIRC:
            s = "F.CIRCLE:SHIFT+C"
        elif value == TOOL_BUCKET:
            s = "BUCKET:B"
        else:
            s = ""

        self.help_message = s
